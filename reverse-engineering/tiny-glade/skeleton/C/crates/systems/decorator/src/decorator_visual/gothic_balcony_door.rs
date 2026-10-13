// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 20b03a6b67703ffce542325d23fc71254262bc03664229c9cabee70a689845d3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gothic_balcony_door.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 470
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.24.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
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
// 源码行: 0, 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BE7
// 首条位置: {"module":470,"inlinee":27623}
// 编译实例 RVAs: 0x8c5720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2228711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377906464,"raw_hex":"e819feddff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377907394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2228942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377906464,"raw_hex":"e832fdddff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377907641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9185192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377906464,"raw_hex":"e858d873ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377907379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401ab390
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7447621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377906464,"raw_hex":"e8bb5b8eff","size":5,"soft_fail":false,"target_va":5370459024,"va":5377906640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2077609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377906464,"raw_hex":"e8574ce0ff","size":5,"soft_fail":false,"target_va":5375829056,"va":5377906660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8c720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3960550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377906464,"raw_hex":"e8e66e3c00","size":5,"soft_fail":false,"target_va":5381867296,"va":5377906741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38712992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377906464,"raw_hex":"e8a0b64e02","size":5,"soft_fail":false,"target_va":5416620769,"va":5377907772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":470,"record_offset":744,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":470,"record_offset":2700,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":470,"record_offset":624,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":8966,"source_line":122,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":470,"record_offset":772,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":27618,"source_line":188,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":470,"record_offset":828,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":3692,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":1576,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":2460,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":1724,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":1780,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::add_door_stairs
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":470,"record_offset":2756,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":27619,"source_line":266,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":470,"record_offset":800,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":470,"record_offset":1696,"parent_record_offset":412,"ancestor_record_offset":412,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":412,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":412,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":412,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":412,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":412,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":412,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":2868,"parent_record_offset":2836,"ancestor_record_offset":412,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":412,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":470,"record_offset":2836,"parent_record_offset":2792,"ancestor_record_offset":412,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":470,"record_offset":2792,"parent_record_offset":2756,"ancestor_record_offset":412,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":470,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":412,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/lib.rs
//     evidence={"module":470,"record_offset":656,"parent_record_offset":624,"ancestor_record_offset":412,"inlinee":8834,"source_line":408,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/internal.rs
//     evidence={"module":470,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":412,"inlinee":8836,"source_line":254,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/internal.rs
//     evidence={"module":470,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":412,"inlinee":8837,"source_line":353,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":470,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":412,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":470,"record_offset":1272,"parent_record_offset":852,"ancestor_record_offset":412,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":470,"record_offset":1180,"parent_record_offset":1124,"ancestor_record_offset":412,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":470,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":412,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":470,"record_offset":980,"parent_record_offset":896,"ancestor_record_offset":412,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":470,"record_offset":1044,"parent_record_offset":852,"ancestor_record_offset":412,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":470,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":412,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":470,"record_offset":1604,"parent_record_offset":1576,"ancestor_record_offset":412,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":470,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":412,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":470,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":412,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":470,"record_offset":2488,"parent_record_offset":2460,"ancestor_record_offset":412,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":470,"record_offset":2992,"parent_record_offset":2756,"ancestor_record_offset":412,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":470,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":412,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":470,"record_offset":1300,"parent_record_offset":852,"ancestor_record_offset":412,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":470,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":412,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":470,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":412,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":3784,"parent_record_offset":3752,"ancestor_record_offset":412,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":3752,"parent_record_offset":3720,"ancestor_record_offset":412,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":3720,"parent_record_offset":3692,"ancestor_record_offset":412,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":470,"record_offset":924,"parent_record_offset":896,"ancestor_record_offset":412,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":470,"record_offset":896,"parent_record_offset":852,"ancestor_record_offset":412,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":470,"record_offset":3816,"parent_record_offset":3784,"ancestor_record_offset":412,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_balcony_door::gothic_hat_visible_height
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BE6
// 首条位置: {"module":470,"inlinee":27622}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":470,"record_offset":16740,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":470,"record_offset":15764,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":470,"record_offset":16260,"parent_record_offset":15712,"ancestor_record_offset":15712,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":470,"record_offset":15828,"parent_record_offset":15796,"ancestor_record_offset":15712,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":470,"record_offset":15860,"parent_record_offset":15828,"ancestor_record_offset":15712,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":470,"record_offset":15892,"parent_record_offset":15860,"ancestor_record_offset":15712,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":470,"record_offset":15956,"parent_record_offset":15924,"ancestor_record_offset":15712,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":470,"record_offset":16188,"parent_record_offset":16052,"ancestor_record_offset":15712,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":470,"record_offset":16668,"parent_record_offset":16548,"ancestor_record_offset":15712,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":470,"record_offset":16052,"parent_record_offset":16020,"ancestor_record_offset":15712,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":470,"record_offset":16548,"parent_record_offset":16516,"ancestor_record_offset":15712,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":470,"record_offset":16148,"parent_record_offset":16120,"ancestor_record_offset":15712,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":470,"record_offset":16020,"parent_record_offset":15988,"ancestor_record_offset":15712,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":470,"record_offset":16516,"parent_record_offset":16484,"ancestor_record_offset":15712,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":470,"record_offset":15796,"parent_record_offset":15764,"ancestor_record_offset":15712,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":470,"record_offset":16120,"parent_record_offset":16092,"ancestor_record_offset":15712,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":470,"record_offset":16092,"parent_record_offset":16052,"ancestor_record_offset":15712,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":470,"record_offset":15924,"parent_record_offset":15892,"ancestor_record_offset":15712,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":470,"record_offset":15988,"parent_record_offset":15956,"ancestor_record_offset":15712,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":470,"record_offset":3920,"rva":9198672,"va":5377907792}
// 编译实例 RVAs: 0x8c5c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9189088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e820c973ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377911275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401aaf30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7456994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e81e378eff","size":5,"soft_fail":false,"target_va":5370457904,"va":5377914893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401ab390
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7455287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8c93d8eff","size":5,"soft_fail":false,"target_va":5370459024,"va":5377914306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4430160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8b066bcff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377909195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4429041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e80f6bbcff","size":5,"soft_fail":false,"target_va":5373479776,"va":5377908812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2085269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e86b2ee0ff","size":5,"soft_fail":false,"target_va":5375829056,"va":5377914320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=3
//     target_aliases=12 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t208664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8182f0300","size":5,"soft_fail":false,"target_va":5378120480,"va":5377911811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b30
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t207652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8242b0300","size":5,"soft_fail":false,"target_va":5378120496,"va":5377912839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=3
//     target_aliases=20 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t217522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8b2510300","size":5,"soft_fail":false,"target_va":5378126944,"va":5377909417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14094d4f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t553903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8af730800","size":5,"soft_fail":false,"target_va":5378462960,"va":5377909052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a44730
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1566875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e89be81700","size":5,"soft_fail":false,"target_va":5379475248,"va":5377908368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3752446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8fe413900","size":5,"soft_fail":false,"target_va":5381660768,"va":5377908317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8c720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3952874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8ea503c00","size":5,"soft_fail":false,"target_va":5381867296,"va":5377914417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c915a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3969656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e878923c00","size":5,"soft_fail":false,"target_va":5381887392,"va":5377917731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5415180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e80ca15200","size":5,"soft_fail":false,"target_va":5383325024,"va":5377909839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fd3a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11759323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8db6eb300","size":5,"soft_fail":false,"target_va":5389669280,"va":5377909952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12431107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e803afbd00","size":5,"soft_fail":false,"target_va":5390340576,"va":5377909464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414dff60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gothic_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12682589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e85d85c100","size":5,"soft_fail":false,"target_va":5390597984,"va":5377915390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t38469860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8e4004b02","size":5,"soft_fail":false,"target_va":5416381344,"va":5377911479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38704891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8fb964e02","size":5,"soft_fail":false,"target_va":5416621408,"va":5377916512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38705182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e81e984e02","size":5,"soft_fail":false,"target_va":5416621536,"va":5377916349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38705276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e87c984e02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377916319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38705605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8c5994e02","size":5,"soft_fail":false,"target_va":5416622208,"va":5377916598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38704930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e822974e02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377916827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38705826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8a29a4e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5377916409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e85b280000","size":5,"soft_fail":false,"target_va":5377918288,"va":5377907952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38704681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e829964e02","size":5,"soft_fail":false,"target_va":5416621376,"va":5377916690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorDst::as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t212223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8ff3c0300","size":5,"soft_fail":false,"target_va":5378126416,"va":5377914188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t217164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e84c500300","size":5,"soft_fail":false,"target_va":5378126544,"va":5377909375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1566566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e866e71700","size":5,"soft_fail":false,"target_va":5379475184,"va":5377908613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3741427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8f3163900","size":5,"soft_fail":false,"target_va":5381650864,"va":5377909432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3745303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e817263900","size":5,"soft_fail":false,"target_va":5381654320,"va":5377909012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::stone_foundation_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3319020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8eca43200","size":5,"soft_fail":false,"target_va":5381231968,"va":5377912943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutout::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t281194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e86a4a0400","size":5,"soft_fail":false,"target_va":5378193456,"va":5377912257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutKind::gothic_balcony_door_rank
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t281444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8644b0400","size":5,"soft_fail":false,"target_va":5378193664,"va":5377912215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6055495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e847665c00","size":5,"soft_fail":false,"target_va":5383963792,"va":5377908292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6064002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e882875c00","size":5,"soft_fail":false,"target_va":5383972224,"va":5377908217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6048709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8c54b5c00","size":5,"soft_fail":false,"target_va":5383964912,"va":5377916198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6054889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8e9635c00","size":5,"soft_fail":false,"target_va":5383962816,"va":5377907922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38801169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8110f5002","size":5,"soft_fail":false,"target_va":5416717696,"va":5377916522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38702812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8dc8e4e02","size":5,"soft_fail":false,"target_va":5416619360,"va":5377916543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::add_door_stairs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t155718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e846600200","size":5,"soft_fail":false,"target_va":5378070992,"va":5377915269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t159159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8b76d0200","size":5,"soft_fail":false,"target_va":5378070256,"va":5377911092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t162730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8aa7b0200","size":5,"soft_fail":false,"target_va":5378071840,"va":5377909105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_balcony_door::add_door_autoclutter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e820150000","size":5,"soft_fail":false,"target_va":5377919584,"va":5377914171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-36872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e8f86fffff","size":5,"soft_fail":false,"target_va":5377872704,"va":5377909571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5413240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e878995200","size":5,"soft_fail":false,"target_va":5383324048,"va":5377910803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5370392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377907792,"raw_hex":"e818f25100","size":5,"soft_fail":false,"target_va":5383280256,"va":5377909859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":470,"record_offset":14216,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":470,"record_offset":8752,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":470,"record_offset":8808,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":470,"record_offset":6524,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":470,"record_offset":6496,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":470,"record_offset":21244,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::impl$22::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":470,"record_offset":8836,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":14661,"source_line":28,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":470,"record_offset":20844,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":8966,"source_line":122,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":9852,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":26697,"source_line":291,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":10316,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":26677,"source_line":298,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":11580,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":26686,"source_line":309,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":6440,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":26241,"source_line":273,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::has_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":470,"record_offset":15596,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":26244,"source_line":143,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":470,"record_offset":14816,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":470,"record_offset":12956,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":10104,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":10288,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":10020,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":12900,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":12844,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":9816,"source_line":1553,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":13824,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":10048,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":470,"record_offset":5632,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":470,"record_offset":4232,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":470,"record_offset":4620,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":470,"record_offset":8864,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":470,"record_offset":13916,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":26690,"source_line":270,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs
//     evidence={"module":470,"record_offset":17076,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":27623,"source_line":280,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::gothic_balcony_door::get_door_holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs
//     evidence={"module":470,"record_offset":14012,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":27621,"source_line":337,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::gothic_balcony_door::gothic_hat_visible_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs
//     evidence={"module":470,"record_offset":15712,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":27622,"source_line":51,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":470,"record_offset":15348,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":26693,"source_line":85,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":470,"record_offset":15320,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":26692,"source_line":55,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":470,"record_offset":19848,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":26598,"source_line":1171,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":470,"record_offset":20148,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":470,"record_offset":9992,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::x
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":470,"record_offset":15072,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":27592,"source_line":151,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":14116,"parent_record_offset":14092,"ancestor_record_offset":3920,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":14140,"parent_record_offset":14116,"ancestor_record_offset":3920,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":14164,"parent_record_offset":14140,"ancestor_record_offset":3920,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":14092,"parent_record_offset":14068,"ancestor_record_offset":3920,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":470,"record_offset":14068,"parent_record_offset":14044,"ancestor_record_offset":3920,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":470,"record_offset":14044,"parent_record_offset":14012,"ancestor_record_offset":3920,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":470,"record_offset":14396,"parent_record_offset":14328,"ancestor_record_offset":3920,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":470,"record_offset":14460,"parent_record_offset":14428,"ancestor_record_offset":3920,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":470,"record_offset":14428,"parent_record_offset":14396,"ancestor_record_offset":3920,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":470,"record_offset":23060,"parent_record_offset":23024,"ancestor_record_offset":3920,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":470,"record_offset":11428,"parent_record_offset":10348,"ancestor_record_offset":3920,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":470,"record_offset":22872,"parent_record_offset":22848,"ancestor_record_offset":3920,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":470,"record_offset":14328,"parent_record_offset":14248,"ancestor_record_offset":3920,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":470,"record_offset":14248,"parent_record_offset":14216,"ancestor_record_offset":3920,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":470,"record_offset":20184,"parent_record_offset":20148,"ancestor_record_offset":3920,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":470,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":3920,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/lib.rs
//     evidence={"module":470,"record_offset":17232,"parent_record_offset":17196,"ancestor_record_offset":3920,"inlinee":8834,"source_line":408,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/internal.rs
//     evidence={"module":470,"record_offset":17256,"parent_record_offset":17232,"ancestor_record_offset":3920,"inlinee":8836,"source_line":254,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bytemuck","version":"1.24.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.24.0/src/internal.rs
//     evidence={"module":470,"record_offset":17280,"parent_record_offset":17256,"ancestor_record_offset":3920,"inlinee":8837,"source_line":353,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":470,"record_offset":15828,"parent_record_offset":15796,"ancestor_record_offset":3920,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":470,"record_offset":15860,"parent_record_offset":15828,"ancestor_record_offset":3920,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":470,"record_offset":15892,"parent_record_offset":15860,"ancestor_record_offset":3920,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":470,"record_offset":15956,"parent_record_offset":15924,"ancestor_record_offset":3920,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":470,"record_offset":5960,"parent_record_offset":5932,"ancestor_record_offset":3920,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":470,"record_offset":5988,"parent_record_offset":5960,"ancestor_record_offset":3920,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":470,"record_offset":5908,"parent_record_offset":5880,"ancestor_record_offset":3920,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":470,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":3920,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":470,"record_offset":6048,"parent_record_offset":5932,"ancestor_record_offset":3920,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":470,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":3920,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":470,"record_offset":16188,"parent_record_offset":16052,"ancestor_record_offset":3920,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":470,"record_offset":16668,"parent_record_offset":16548,"ancestor_record_offset":3920,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":470,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":3920,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":470,"record_offset":16052,"parent_record_offset":16020,"ancestor_record_offset":3920,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":470,"record_offset":16548,"parent_record_offset":16516,"ancestor_record_offset":3920,"inlinee":5404,"source_line":1605,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":470,"record_offset":6464,"parent_record_offset":6440,"ancestor_record_offset":3920,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":470,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":3920,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":470,"record_offset":8036,"parent_record_offset":8012,"ancestor_record_offset":3920,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":470,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":3920,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":470,"record_offset":13484,"parent_record_offset":13064,"ancestor_record_offset":3920,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":470,"record_offset":13392,"parent_record_offset":13336,"ancestor_record_offset":3920,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":470,"record_offset":13160,"parent_record_offset":13136,"ancestor_record_offset":3920,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":470,"record_offset":13192,"parent_record_offset":13108,"ancestor_record_offset":3920,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":470,"record_offset":13256,"parent_record_offset":13064,"ancestor_record_offset":3920,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":470,"record_offset":16740,"parent_record_offset":15712,"ancestor_record_offset":3920,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":470,"record_offset":19040,"parent_record_offset":17076,"ancestor_record_offset":3920,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":470,"record_offset":16148,"parent_record_offset":16120,"ancestor_record_offset":3920,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":470,"record_offset":10240,"parent_record_offset":10216,"ancestor_record_offset":3920,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":470,"record_offset":8536,"parent_record_offset":7900,"ancestor_record_offset":3920,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":470,"record_offset":15764,"parent_record_offset":15712,"ancestor_record_offset":3920,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":470,"record_offset":16020,"parent_record_offset":15988,"ancestor_record_offset":3920,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":470,"record_offset":16260,"parent_record_offset":15712,"ancestor_record_offset":3920,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":470,"record_offset":16516,"parent_record_offset":16484,"ancestor_record_offset":3920,"inlinee":5402,"source_line":3391,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":470,"record_offset":15796,"parent_record_offset":15764,"ancestor_record_offset":3920,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":470,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":3920,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":470,"record_offset":8268,"parent_record_offset":8240,"ancestor_record_offset":3920,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":470,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":3920,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":470,"record_offset":13856,"parent_record_offset":13824,"ancestor_record_offset":3920,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":470,"record_offset":12868,"parent_record_offset":12844,"ancestor_record_offset":3920,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":470,"record_offset":18180,"parent_record_offset":18152,"ancestor_record_offset":3920,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":470,"record_offset":10072,"parent_record_offset":10048,"ancestor_record_offset":3920,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":470,"record_offset":16120,"parent_record_offset":16092,"ancestor_record_offset":3920,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":470,"record_offset":5780,"parent_record_offset":5740,"ancestor_record_offset":3920,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":470,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":3920,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":470,"record_offset":15924,"parent_record_offset":15892,"ancestor_record_offset":3920,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":470,"record_offset":15988,"parent_record_offset":15956,"ancestor_record_offset":3920,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":470,"record_offset":13948,"parent_record_offset":13916,"ancestor_record_offset":3920,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":470,"record_offset":13972,"parent_record_offset":13948,"ancestor_record_offset":3920,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":470,"record_offset":7028,"parent_record_offset":6524,"ancestor_record_offset":3920,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":470,"record_offset":8708,"parent_record_offset":7728,"ancestor_record_offset":3920,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":470,"record_offset":9920,"parent_record_offset":9884,"ancestor_record_offset":3920,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":470,"record_offset":4568,"parent_record_offset":4540,"ancestor_record_offset":3920,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":470,"record_offset":6396,"parent_record_offset":5740,"ancestor_record_offset":3920,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":470,"record_offset":8060,"parent_record_offset":8036,"ancestor_record_offset":3920,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":470,"record_offset":6112,"parent_record_offset":5880,"ancestor_record_offset":3920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":470,"record_offset":6896,"parent_record_offset":6872,"ancestor_record_offset":3920,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":470,"record_offset":23092,"parent_record_offset":23060,"ancestor_record_offset":3920,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":470,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":3920,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":470,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":3920,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":470,"record_offset":5224,"parent_record_offset":5196,"ancestor_record_offset":3920,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":470,"record_offset":14508,"parent_record_offset":14248,"ancestor_record_offset":3920,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":470,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":3920,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":470,"record_offset":5296,"parent_record_offset":5268,"ancestor_record_offset":3920,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":470,"record_offset":20216,"parent_record_offset":20184,"ancestor_record_offset":3920,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":470,"record_offset":20248,"parent_record_offset":20216,"ancestor_record_offset":3920,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":470,"record_offset":4968,"parent_record_offset":4944,"ancestor_record_offset":3920,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":470,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":3920,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":470,"record_offset":5352,"parent_record_offset":5324,"ancestor_record_offset":3920,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":470,"record_offset":4944,"parent_record_offset":4828,"ancestor_record_offset":3920,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":470,"record_offset":4360,"parent_record_offset":4336,"ancestor_record_offset":3920,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":470,"record_offset":4656,"parent_record_offset":4620,"ancestor_record_offset":3920,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":470,"record_offset":4796,"parent_record_offset":4764,"ancestor_record_offset":3920,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":470,"record_offset":4764,"parent_record_offset":4728,"ancestor_record_offset":3920,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":470,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":3920,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":470,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":3920,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":470,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":3920,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":470,"record_offset":5268,"parent_record_offset":4728,"ancestor_record_offset":3920,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":470,"record_offset":4336,"parent_record_offset":4304,"ancestor_record_offset":3920,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":470,"record_offset":4304,"parent_record_offset":4276,"ancestor_record_offset":3920,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":470,"record_offset":4460,"parent_record_offset":4428,"ancestor_record_offset":3920,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":470,"record_offset":4516,"parent_record_offset":4460,"ancestor_record_offset":3920,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":470,"record_offset":4428,"parent_record_offset":4400,"ancestor_record_offset":3920,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":470,"record_offset":4276,"parent_record_offset":4232,"ancestor_record_offset":3920,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":470,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":3920,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":470,"record_offset":4828,"parent_record_offset":4796,"ancestor_record_offset":3920,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":470,"record_offset":5112,"parent_record_offset":4828,"ancestor_record_offset":3920,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":470,"record_offset":5084,"parent_record_offset":4828,"ancestor_record_offset":3920,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":470,"record_offset":4888,"parent_record_offset":4828,"ancestor_record_offset":3920,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":470,"record_offset":11456,"parent_record_offset":11428,"ancestor_record_offset":3920,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":470,"record_offset":22908,"parent_record_offset":22872,"ancestor_record_offset":3920,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":470,"record_offset":11480,"parent_record_offset":11456,"ancestor_record_offset":3920,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":470,"record_offset":22932,"parent_record_offset":22908,"ancestor_record_offset":3920,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":470,"record_offset":22964,"parent_record_offset":22872,"ancestor_record_offset":3920,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":470,"record_offset":17196,"parent_record_offset":17076,"ancestor_record_offset":3920,"inlinee":8966,"source_line":122,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":470,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":3920,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":470,"record_offset":7728,"parent_record_offset":7692,"ancestor_record_offset":3920,"inlinee":26676,"source_line":77,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":10348,"parent_record_offset":10316,"ancestor_record_offset":3920,"inlinee":26679,"source_line":249,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":9884,"parent_record_offset":9852,"ancestor_record_offset":3920,"inlinee":26698,"source_line":223,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":11612,"parent_record_offset":11580,"ancestor_record_offset":3920,"inlinee":26687,"source_line":258,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":11204,"parent_record_offset":11176,"ancestor_record_offset":3920,"inlinee":26681,"source_line":94,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":11540,"parent_record_offset":11516,"ancestor_record_offset":3920,"inlinee":26703,"source_line":252,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":9952,"parent_record_offset":9920,"ancestor_record_offset":3920,"inlinee":26707,"source_line":226,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":470,"record_offset":12804,"parent_record_offset":12780,"ancestor_record_offset":3920,"inlinee":26705,"source_line":261,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":470,"record_offset":9632,"parent_record_offset":9608,"ancestor_record_offset":3920,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":470,"record_offset":15648,"parent_record_offset":15624,"ancestor_record_offset":3920,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::supports_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":470,"record_offset":15624,"parent_record_offset":15596,"ancestor_record_offset":3920,"inlinee":26245,"source_line":137,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":470,"record_offset":17320,"parent_record_offset":17076,"ancestor_record_offset":3920,"inlinee":27618,"source_line":188,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":470,"record_offset":17376,"parent_record_offset":17076,"ancestor_record_offset":3920,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":470,"record_offset":13064,"parent_record_offset":12956,"ancestor_record_offset":3920,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":470,"record_offset":13512,"parent_record_offset":13064,"ancestor_record_offset":3920,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":470,"record_offset":13028,"parent_record_offset":13004,"ancestor_record_offset":3920,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":470,"record_offset":13004,"parent_record_offset":12980,"ancestor_record_offset":3920,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":10184,"parent_record_offset":10152,"ancestor_record_offset":3920,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":10152,"parent_record_offset":10128,"ancestor_record_offset":3920,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":3920,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":19572,"parent_record_offset":17076,"ancestor_record_offset":3920,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":17884,"parent_record_offset":17076,"ancestor_record_offset":3920,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":470,"record_offset":18864,"parent_record_offset":17076,"ancestor_record_offset":3920,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":20468,"parent_record_offset":20440,"ancestor_record_offset":3920,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":18096,"parent_record_offset":17076,"ancestor_record_offset":3920,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":20708,"parent_record_offset":20468,"ancestor_record_offset":3920,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":470,"record_offset":18152,"parent_record_offset":17076,"ancestor_record_offset":3920,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":470,"record_offset":13136,"parent_record_offset":13108,"ancestor_record_offset":3920,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":470,"record_offset":13108,"parent_record_offset":13064,"ancestor_record_offset":3920,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":470,"record_offset":6720,"parent_record_offset":6692,"ancestor_record_offset":3920,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":470,"record_offset":6752,"parent_record_offset":6720,"ancestor_record_offset":3920,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":470,"record_offset":6692,"parent_record_offset":6652,"ancestor_record_offset":3920,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":470,"record_offset":8012,"parent_record_offset":7900,"ancestor_record_offset":3920,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":470,"record_offset":6816,"parent_record_offset":6652,"ancestor_record_offset":3920,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":470,"record_offset":8564,"parent_record_offset":7900,"ancestor_record_offset":3920,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":470,"record_offset":8096,"parent_record_offset":7900,"ancestor_record_offset":3920,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":470,"record_offset":7956,"parent_record_offset":7900,"ancestor_record_offset":3920,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":470,"record_offset":8444,"parent_record_offset":8420,"ancestor_record_offset":3920,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":470,"record_offset":7788,"parent_record_offset":7760,"ancestor_record_offset":3920,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":470,"record_offset":8420,"parent_record_offset":8312,"ancestor_record_offset":3920,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":470,"record_offset":6580,"parent_record_offset":6552,"ancestor_record_offset":3920,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":470,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":3920,"inlinee":9034,"source_line":3365,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":470,"record_offset":7824,"parent_record_offset":7788,"ancestor_record_offset":3920,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":8360,"parent_record_offset":8336,"ancestor_record_offset":3920,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":6872,"parent_record_offset":6652,"ancestor_record_offset":3920,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":8656,"parent_record_offset":7900,"ancestor_record_offset":3920,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":6652,"parent_record_offset":6616,"ancestor_record_offset":3920,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":3920,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":3920,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":7852,"parent_record_offset":7788,"ancestor_record_offset":3920,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":3920,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":7984,"parent_record_offset":7900,"ancestor_record_offset":3920,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":6616,"parent_record_offset":6580,"ancestor_record_offset":3920,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":470,"record_offset":8312,"parent_record_offset":7900,"ancestor_record_offset":3920,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":470,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":3920,"inlinee":11498,"source_line":1798,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":470,"record_offset":5668,"parent_record_offset":5632,"ancestor_record_offset":3920,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":470,"record_offset":21308,"parent_record_offset":21276,"ancestor_record_offset":3920,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":470,"record_offset":21760,"parent_record_offset":21556,"ancestor_record_offset":3920,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":470,"record_offset":5808,"parent_record_offset":5780,"ancestor_record_offset":3920,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":470,"record_offset":5880,"parent_record_offset":5740,"ancestor_record_offset":3920,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":470,"record_offset":6084,"parent_record_offset":5880,"ancestor_record_offset":3920,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":470,"record_offset":7760,"parent_record_offset":7728,"ancestor_record_offset":3920,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":470,"record_offset":6552,"parent_record_offset":6524,"ancestor_record_offset":3920,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":470,"record_offset":7148,"parent_record_offset":7116,"ancestor_record_offset":3920,"inlinee":16842,"source_line":1676,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":470,"record_offset":10216,"parent_record_offset":10184,"ancestor_record_offset":3920,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":470,"record_offset":5836,"parent_record_offset":5808,"ancestor_record_offset":3920,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":470,"record_offset":5740,"parent_record_offset":5704,"ancestor_record_offset":3920,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":470,"record_offset":5704,"parent_record_offset":5668,"ancestor_record_offset":3920,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":470,"record_offset":20292,"parent_record_offset":20148,"ancestor_record_offset":3920,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":470,"record_offset":20116,"parent_record_offset":19848,"ancestor_record_offset":3920,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":470,"record_offset":20088,"parent_record_offset":19848,"ancestor_record_offset":3920,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":470,"record_offset":19892,"parent_record_offset":19848,"ancestor_record_offset":3920,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":470,"record_offset":19924,"parent_record_offset":19892,"ancestor_record_offset":3920,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":470,"record_offset":20024,"parent_record_offset":19996,"ancestor_record_offset":3920,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":470,"record_offset":19996,"parent_record_offset":19848,"ancestor_record_offset":3920,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":470,"record_offset":17348,"parent_record_offset":17076,"ancestor_record_offset":3920,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":470,"record_offset":20440,"parent_record_offset":20148,"ancestor_record_offset":3920,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::gothic_balcony_door::get_door_holes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 337
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BE5
// 首条位置: {"module":470,"inlinee":27621}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":470,"record_offset":14044,"parent_record_offset":14012,"ancestor_record_offset":14012,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":14116,"parent_record_offset":14092,"ancestor_record_offset":14012,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":14140,"parent_record_offset":14116,"ancestor_record_offset":14012,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":14164,"parent_record_offset":14140,"ancestor_record_offset":14012,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":470,"record_offset":14092,"parent_record_offset":14068,"ancestor_record_offset":14012,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00470.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":470,"record_offset":14068,"parent_record_offset":14044,"ancestor_record_offset":14012,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00470.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// 信息 ID: 8cfee098c59abc9189fff93d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x8ca319 -> 0x8c5720 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=470, inline_record=17076; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors
// 信息 ID: e8c671b0a1968e631ef4e6ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dc172a4ba3b8f88c1aeeacf2: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)> @ 0x1bab98 -> 0x8c5c50 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)> > @ 0x295e30 -> 0x8c5c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_balcony_door::get_door_holes
// 信息 ID: 89ae774f35a9123b5212e24e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=470, inline_record=14012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::gothic_balcony_door::gothic_hat_visible_height
// 信息 ID: 1d8af760e641d1c0d704f7e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=470, inline_record=15712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
