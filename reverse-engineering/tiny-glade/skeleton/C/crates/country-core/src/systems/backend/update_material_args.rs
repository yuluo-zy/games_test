// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/backend/update_material_args.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e6ce130d1d0679241275f2a9c95a5d24d16eee7502853b9e381a69d497eba907
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: update_material_args.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 500, 1646
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::material_update_firefly_glow
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":61120,"rva":9400928,"va":5378110048}
// 编译实例 RVAs: 0x8f7260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3467564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378110048,"raw_hex":"e82ce93400","size":5,"soft_fail":false,"target_va":5381577744,"va":5378110175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416c9f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14494595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378110048,"raw_hex":"e8832bdd00","size":5,"soft_fail":false,"target_va":5392605056,"va":5378110456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416ca270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14495446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378110048,"raw_hex":"e8d62edd00","size":5,"soft_fail":false,"target_va":5392605808,"va":5378110357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38511058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378110048,"raw_hex":"e8d2a14b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5378110537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378110048,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5378110196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":500,"record_offset":61800,"parent_record_offset":61120,"ancestor_record_offset":61120,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Single::into_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":500,"record_offset":61304,"parent_record_offset":61120,"ancestor_record_offset":61120,"inlinee":28020,"source_line":2660,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":61448,"parent_record_offset":61120,"ancestor_record_offset":61120,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":500,"record_offset":61388,"parent_record_offset":61120,"ancestor_record_offset":61120,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":500,"record_offset":61360,"parent_record_offset":61120,"ancestor_record_offset":61120,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":500,"record_offset":61824,"parent_record_offset":61800,"ancestor_record_offset":61120,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":500,"record_offset":61412,"parent_record_offset":61388,"ancestor_record_offset":61120,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::material_update_transforms
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":39272,"rva":9394208,"va":5378103328}
// 编译实例 RVAs: 0x8f5820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140992ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t642051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e803cc0900","size":5,"soft_fail":false,"target_va":5378747616,"va":5378105560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e6ac10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5721585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e8f14d5700","size":5,"soft_fail":false,"target_va":5383826448,"va":5378104858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141564200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13033806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e84ee1c600","size":5,"soft_fail":false,"target_va":5391139328,"va":5378105517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14674083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e8a3e8df00","size":5,"soft_fail":false,"target_va":5392780768,"va":5378106680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38279273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e869184802","size":5,"soft_fail":false,"target_va":5416385024,"va":5378105746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38952060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e87c5c5202","size":5,"soft_fail":false,"target_va":5417057040,"va":5378104975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12844585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e829fec300","size":5,"soft_fail":false,"target_va":5390948448,"va":5378103858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::storage::table::Table::get_changed_ticks_slice_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6294347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e84b0b6000","size":5,"soft_fail":false,"target_va":5384398656,"va":5378104304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38515667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e8d3b34b02","size":5,"soft_fail":false,"target_va":5416621408,"va":5378105736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e8da220000","size":5,"soft_fail":false,"target_va":5378112368,"va":5378103441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38515548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e85cb34b02","size":5,"soft_fail":false,"target_va":5416621376,"va":5378105823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5859996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e89c6a5900","size":5,"soft_fail":false,"target_va":5383963792,"va":5378103791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5868503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e8d78b5900","size":5,"soft_fail":false,"target_va":5383972224,"va":5378103716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5859260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e8bc675900","size":5,"soft_fail":false,"target_va":5383964912,"va":5378105647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5859393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e841685900","size":5,"soft_fail":false,"target_va":5383962816,"va":5378103418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::clone_binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5594053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e8c55b5500","size":5,"soft_fail":false,"target_va":5383699088,"va":5378105030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5597909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e8d56a5500","size":5,"soft_fail":false,"target_va":5383702992,"va":5378105078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5302869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e855ea5000","size":5,"soft_fail":false,"target_va":5383408256,"va":5378105382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38513567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378103328,"raw_hex":"e89fab4b02","size":5,"soft_fail":false,"target_va":5416619360,"va":5378105788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":500,"record_offset":47296,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":41792,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":500,"record_offset":41692,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":47236,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":46132,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":47352,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":46160,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":500,"record_offset":46460,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":40884,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":39484,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":39872,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":500,"record_offset":47264,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":27992,"source_line":406,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":500,"record_offset":46576,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":39272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":45300,"parent_record_offset":45276,"ancestor_record_offset":39272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":45276,"parent_record_offset":45252,"ancestor_record_offset":39272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":500,"record_offset":46484,"parent_record_offset":46460,"ancestor_record_offset":39272,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":45228,"parent_record_offset":45204,"ancestor_record_offset":39272,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":45204,"parent_record_offset":45180,"ancestor_record_offset":39272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":42132,"parent_record_offset":42108,"ancestor_record_offset":39272,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":42156,"parent_record_offset":42132,"ancestor_record_offset":39272,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":45180,"parent_record_offset":45156,"ancestor_record_offset":39272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":45156,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":43700,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":43784,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":45408,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":43616,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":500,"record_offset":47320,"parent_record_offset":47296,"ancestor_record_offset":39272,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::tick::Tick::is_newer_than
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/tick.rs
//     evidence={"module":500,"record_offset":43424,"parent_record_offset":43400,"ancestor_record_offset":39272,"inlinee":28015,"source_line":53,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::tick::Tick::relative_to
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/tick.rs
//     evidence={"module":500,"record_offset":43452,"parent_record_offset":43424,"ancestor_record_offset":39272,"inlinee":4143,"source_line":67,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":43376,"parent_record_offset":43352,"ancestor_record_offset":39272,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":44016,"parent_record_offset":43876,"ancestor_record_offset":39272,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":42300,"parent_record_offset":42192,"ancestor_record_offset":39272,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":45916,"parent_record_offset":45892,"ancestor_record_offset":39272,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":43904,"parent_record_offset":43876,"ancestor_record_offset":39272,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":42216,"parent_record_offset":42192,"ancestor_record_offset":39272,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":45692,"parent_record_offset":45668,"ancestor_record_offset":39272,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":45744,"parent_record_offset":45720,"ancestor_record_offset":39272,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":45668,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::filter::impl$11::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/filter.rs
//     evidence={"module":500,"record_offset":45100,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":28016,"source_line":992,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::filter::impl$11::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/filter.rs
//     evidence={"module":500,"record_offset":43324,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":28010,"source_line":1007,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::filter::impl$12::filter_fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/filter.rs
//     evidence={"module":500,"record_offset":43352,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":28012,"source_line":1051,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::filter::impl$12::filter_fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/filter.rs
//     evidence={"module":500,"record_offset":43400,"parent_record_offset":43376,"ancestor_record_offset":39272,"inlinee":28014,"source_line":1059,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":41716,"parent_record_offset":41692,"ancestor_record_offset":39272,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":41816,"parent_record_offset":41792,"ancestor_record_offset":39272,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":43876,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":42192,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":500,"record_offset":42748,"parent_record_offset":42724,"ancestor_record_offset":39272,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":42400,"parent_record_offset":42376,"ancestor_record_offset":39272,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":42264,"parent_record_offset":42240,"ancestor_record_offset":39272,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":42572,"parent_record_offset":42548,"ancestor_record_offset":39272,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":42240,"parent_record_offset":42216,"ancestor_record_offset":39272,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":42324,"parent_record_offset":42300,"ancestor_record_offset":39272,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":42108,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":45512,"parent_record_offset":45484,"ancestor_record_offset":39272,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":42024,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":500,"record_offset":42724,"parent_record_offset":42700,"ancestor_record_offset":39272,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":42700,"parent_record_offset":42676,"ancestor_record_offset":39272,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":500,"record_offset":41756,"parent_record_offset":41716,"ancestor_record_offset":39272,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":500,"record_offset":45772,"parent_record_offset":45744,"ancestor_record_offset":39272,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":500,"record_offset":41212,"parent_record_offset":41184,"ancestor_record_offset":39272,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":500,"record_offset":41240,"parent_record_offset":41212,"ancestor_record_offset":39272,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":500,"record_offset":41160,"parent_record_offset":41132,"ancestor_record_offset":39272,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":500,"record_offset":41184,"parent_record_offset":41160,"ancestor_record_offset":39272,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":500,"record_offset":41300,"parent_record_offset":41184,"ancestor_record_offset":39272,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":500,"record_offset":41412,"parent_record_offset":41388,"ancestor_record_offset":39272,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":46296,"parent_record_offset":46268,"ancestor_record_offset":39272,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":47144,"parent_record_offset":46600,"ancestor_record_offset":39272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":500,"record_offset":46856,"parent_record_offset":46832,"ancestor_record_offset":39272,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":500,"record_offset":46888,"parent_record_offset":46808,"ancestor_record_offset":39272,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":500,"record_offset":46832,"parent_record_offset":46808,"ancestor_record_offset":39272,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":500,"record_offset":41264,"parent_record_offset":41240,"ancestor_record_offset":39272,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":500,"record_offset":43476,"parent_record_offset":43452,"ancestor_record_offset":39272,"inlinee":4145,"source_line":2493,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":500,"record_offset":41032,"parent_record_offset":40992,"ancestor_record_offset":39272,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":42508,"parent_record_offset":42484,"ancestor_record_offset":39272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":42548,"parent_record_offset":42376,"ancestor_record_offset":39272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":500,"record_offset":39820,"parent_record_offset":39792,"ancestor_record_offset":39272,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":500,"record_offset":41648,"parent_record_offset":40992,"ancestor_record_offset":39272,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":41364,"parent_record_offset":41132,"ancestor_record_offset":39272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":45796,"parent_record_offset":45772,"ancestor_record_offset":39272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":41992,"parent_record_offset":41932,"ancestor_record_offset":39272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":40292,"parent_record_offset":40268,"ancestor_record_offset":39272,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":40268,"parent_record_offset":40244,"ancestor_record_offset":39272,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":40476,"parent_record_offset":40448,"ancestor_record_offset":39272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":46380,"parent_record_offset":46352,"ancestor_record_offset":39272,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":40576,"parent_record_offset":40548,"ancestor_record_offset":39272,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":46408,"parent_record_offset":46380,"ancestor_record_offset":39272,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":40548,"parent_record_offset":40520,"ancestor_record_offset":39272,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":42048,"parent_record_offset":42024,"ancestor_record_offset":39272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":45352,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":42072,"parent_record_offset":42048,"ancestor_record_offset":39272,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":42628,"parent_record_offset":42596,"ancestor_record_offset":39272,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":40220,"parent_record_offset":40196,"ancestor_record_offset":39272,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":40244,"parent_record_offset":40220,"ancestor_record_offset":39272,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":40604,"parent_record_offset":40576,"ancestor_record_offset":39272,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":41932,"parent_record_offset":41816,"ancestor_record_offset":39272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":40196,"parent_record_offset":40080,"ancestor_record_offset":39272,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":500,"record_offset":39612,"parent_record_offset":39588,"ancestor_record_offset":39272,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":500,"record_offset":39908,"parent_record_offset":39872,"ancestor_record_offset":39272,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":500,"record_offset":40048,"parent_record_offset":40016,"ancestor_record_offset":39272,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":500,"record_offset":40016,"parent_record_offset":39980,"ancestor_record_offset":39272,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":500,"record_offset":39956,"parent_record_offset":39932,"ancestor_record_offset":39272,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":500,"record_offset":39980,"parent_record_offset":39956,"ancestor_record_offset":39272,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":500,"record_offset":39932,"parent_record_offset":39908,"ancestor_record_offset":39272,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":500,"record_offset":40520,"parent_record_offset":39980,"ancestor_record_offset":39272,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":39588,"parent_record_offset":39556,"ancestor_record_offset":39272,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":39556,"parent_record_offset":39528,"ancestor_record_offset":39272,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":39712,"parent_record_offset":39680,"ancestor_record_offset":39272,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":39768,"parent_record_offset":39712,"ancestor_record_offset":39272,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":39680,"parent_record_offset":39652,"ancestor_record_offset":39272,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":46324,"parent_record_offset":46296,"ancestor_record_offset":39272,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":39528,"parent_record_offset":39484,"ancestor_record_offset":39272,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":39792,"parent_record_offset":39768,"ancestor_record_offset":39272,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":500,"record_offset":40080,"parent_record_offset":40048,"ancestor_record_offset":39272,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":500,"record_offset":40364,"parent_record_offset":40080,"ancestor_record_offset":39272,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":500,"record_offset":40336,"parent_record_offset":40080,"ancestor_record_offset":39272,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":500,"record_offset":40140,"parent_record_offset":40080,"ancestor_record_offset":39272,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::backend::update_material_args::material_update_transforms::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/backend/update_material_args.rs
//     evidence={"module":500,"record_offset":46268,"parent_record_offset":46160,"ancestor_record_offset":39272,"inlinee":28018,"source_line":35,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":500,"record_offset":46600,"parent_record_offset":46576,"ancestor_record_offset":39272,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":500,"record_offset":46640,"parent_record_offset":46600,"ancestor_record_offset":39272,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":46712,"parent_record_offset":46640,"ancestor_record_offset":39272,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":47168,"parent_record_offset":47144,"ancestor_record_offset":39272,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":46808,"parent_record_offset":46600,"ancestor_record_offset":39272,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":500,"record_offset":45536,"parent_record_offset":45512,"ancestor_record_offset":39272,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":500,"record_offset":42596,"parent_record_offset":42572,"ancestor_record_offset":39272,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":40920,"parent_record_offset":40884,"ancestor_record_offset":39272,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":47444,"parent_record_offset":47412,"ancestor_record_offset":39272,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":47896,"parent_record_offset":47692,"ancestor_record_offset":39272,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":500,"record_offset":41060,"parent_record_offset":41032,"ancestor_record_offset":39272,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":41132,"parent_record_offset":40992,"ancestor_record_offset":39272,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":500,"record_offset":41336,"parent_record_offset":41132,"ancestor_record_offset":39272,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":500,"record_offset":41088,"parent_record_offset":41060,"ancestor_record_offset":39272,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":500,"record_offset":40992,"parent_record_offset":40956,"ancestor_record_offset":39272,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":500,"record_offset":40956,"parent_record_offset":40920,"ancestor_record_offset":39272,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::material_update_transforms::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D72
// 首条位置: {"module":500,"inlinee":28018}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":46296,"parent_record_offset":46268,"ancestor_record_offset":46268,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":500,"record_offset":46352,"parent_record_offset":46324,"ancestor_record_offset":46268,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":46380,"parent_record_offset":46352,"ancestor_record_offset":46268,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":500,"record_offset":46408,"parent_record_offset":46380,"ancestor_record_offset":46268,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":500,"record_offset":46324,"parent_record_offset":46296,"ancestor_record_offset":46268,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::material_update_arrow_gizmos
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":54748,"rva":9399280,"va":5378108400}
// 编译实例 RVAs: 0x8f6bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1416c9c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14494987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378108400,"raw_hex":"e80b2ddd00","size":5,"soft_fail":false,"target_va":5392604256,"va":5378109264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12840026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378108400,"raw_hex":"e85aecc300","size":5,"soft_fail":false,"target_va":5390948560,"va":5378108529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5299094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378108400,"raw_hex":"e896db5000","size":5,"soft_fail":false,"target_va":5383408256,"va":5378109157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":55028,"parent_record_offset":54748,"ancestor_record_offset":54748,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":500,"record_offset":54932,"parent_record_offset":54748,"ancestor_record_offset":54748,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":61088,"parent_record_offset":54748,"ancestor_record_offset":54748,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":500,"record_offset":60372,"parent_record_offset":54748,"ancestor_record_offset":54748,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":500,"record_offset":60428,"parent_record_offset":54748,"ancestor_record_offset":54748,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":59984,"parent_record_offset":59960,"ancestor_record_offset":54748,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":60032,"parent_record_offset":60008,"ancestor_record_offset":54748,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":60008,"parent_record_offset":59984,"ancestor_record_offset":54748,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":500,"record_offset":60396,"parent_record_offset":60372,"ancestor_record_offset":54748,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":59960,"parent_record_offset":59936,"ancestor_record_offset":54748,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":59936,"parent_record_offset":59912,"ancestor_record_offset":54748,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":56000,"parent_record_offset":55976,"ancestor_record_offset":54748,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":56024,"parent_record_offset":56000,"ancestor_record_offset":54748,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":59912,"parent_record_offset":59888,"ancestor_record_offset":54748,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":59888,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":56208,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":57984,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":60140,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":56124,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":55252,"parent_record_offset":55224,"ancestor_record_offset":54748,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":58244,"parent_record_offset":58108,"ancestor_record_offset":54748,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":56432,"parent_record_offset":56324,"ancestor_record_offset":54748,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":55504,"parent_record_offset":55480,"ancestor_record_offset":54748,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":58132,"parent_record_offset":58108,"ancestor_record_offset":54748,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":56348,"parent_record_offset":56324,"ancestor_record_offset":54748,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":55224,"parent_record_offset":55196,"ancestor_record_offset":54748,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":55276,"parent_record_offset":55252,"ancestor_record_offset":54748,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":55196,"parent_record_offset":55172,"ancestor_record_offset":54748,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":55172,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":54956,"parent_record_offset":54932,"ancestor_record_offset":54748,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":55060,"parent_record_offset":55028,"ancestor_record_offset":54748,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":58108,"parent_record_offset":58076,"ancestor_record_offset":54748,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":56324,"parent_record_offset":56292,"ancestor_record_offset":54748,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":58076,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":56292,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":500,"record_offset":56880,"parent_record_offset":56856,"ancestor_record_offset":54748,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":56532,"parent_record_offset":56508,"ancestor_record_offset":54748,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":56396,"parent_record_offset":56372,"ancestor_record_offset":54748,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":56704,"parent_record_offset":56680,"ancestor_record_offset":54748,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":56372,"parent_record_offset":56348,"ancestor_record_offset":54748,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":56456,"parent_record_offset":56432,"ancestor_record_offset":54748,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":55976,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":60288,"parent_record_offset":60264,"ancestor_record_offset":54748,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":55892,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":500,"record_offset":56856,"parent_record_offset":56832,"ancestor_record_offset":54748,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":56832,"parent_record_offset":56808,"ancestor_record_offset":54748,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":500,"record_offset":54992,"parent_record_offset":54956,"ancestor_record_offset":54748,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":500,"record_offset":55304,"parent_record_offset":55276,"ancestor_record_offset":54748,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":60996,"parent_record_offset":60452,"ancestor_record_offset":54748,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":500,"record_offset":60708,"parent_record_offset":60684,"ancestor_record_offset":54748,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":500,"record_offset":60740,"parent_record_offset":60660,"ancestor_record_offset":54748,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":500,"record_offset":60684,"parent_record_offset":60660,"ancestor_record_offset":54748,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":500,"record_offset":56616,"parent_record_offset":56556,"ancestor_record_offset":54748,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":56640,"parent_record_offset":56616,"ancestor_record_offset":54748,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":56680,"parent_record_offset":56508,"ancestor_record_offset":54748,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":55328,"parent_record_offset":55304,"ancestor_record_offset":54748,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":55856,"parent_record_offset":55828,"ancestor_record_offset":54748,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":55916,"parent_record_offset":55892,"ancestor_record_offset":54748,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":60084,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":55940,"parent_record_offset":55916,"ancestor_record_offset":54748,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":56760,"parent_record_offset":56728,"ancestor_record_offset":54748,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":55828,"parent_record_offset":55060,"ancestor_record_offset":54748,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":500,"record_offset":60452,"parent_record_offset":60428,"ancestor_record_offset":54748,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":500,"record_offset":60492,"parent_record_offset":60452,"ancestor_record_offset":54748,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":60564,"parent_record_offset":60492,"ancestor_record_offset":54748,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":61020,"parent_record_offset":60996,"ancestor_record_offset":54748,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":60660,"parent_record_offset":60452,"ancestor_record_offset":54748,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":500,"record_offset":60312,"parent_record_offset":60288,"ancestor_record_offset":54748,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":500,"record_offset":56728,"parent_record_offset":56704,"ancestor_record_offset":54748,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::material_update_duck
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":21492,"rva":9389344,"va":5378098464}
// 编译实例 RVAs: 0x8f4520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1416ca3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14506304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378098464,"raw_hex":"e84059dd00","size":5,"soft_fail":false,"target_va":5392606176,"va":5378099867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12850426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378098464,"raw_hex":"e8fa14c400","size":5,"soft_fail":false,"target_va":5390949024,"va":5378098593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5308926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378098464,"raw_hex":"e8fe015100","size":5,"soft_fail":false,"target_va":5383408256,"va":5378099325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":21764,"parent_record_offset":21492,"ancestor_record_offset":21492,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":500,"record_offset":21668,"parent_record_offset":21492,"ancestor_record_offset":21492,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":30240,"parent_record_offset":21492,"ancestor_record_offset":21492,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":500,"record_offset":29524,"parent_record_offset":21492,"ancestor_record_offset":21492,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":500,"record_offset":29580,"parent_record_offset":21492,"ancestor_record_offset":21492,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":29136,"parent_record_offset":29112,"ancestor_record_offset":21492,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":29184,"parent_record_offset":29160,"ancestor_record_offset":21492,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":29160,"parent_record_offset":29136,"ancestor_record_offset":21492,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":500,"record_offset":29548,"parent_record_offset":29524,"ancestor_record_offset":21492,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":29112,"parent_record_offset":29088,"ancestor_record_offset":21492,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":29088,"parent_record_offset":29064,"ancestor_record_offset":21492,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":22976,"parent_record_offset":22952,"ancestor_record_offset":21492,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":23000,"parent_record_offset":22976,"ancestor_record_offset":21492,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":29064,"parent_record_offset":29040,"ancestor_record_offset":21492,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":29040,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":23184,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":26016,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":29292,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":23100,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":22004,"parent_record_offset":21976,"ancestor_record_offset":21492,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":26284,"parent_record_offset":26148,"ancestor_record_offset":21492,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":23416,"parent_record_offset":23308,"ancestor_record_offset":21492,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":22256,"parent_record_offset":22232,"ancestor_record_offset":21492,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":26172,"parent_record_offset":26148,"ancestor_record_offset":21492,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":23332,"parent_record_offset":23308,"ancestor_record_offset":21492,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":21976,"parent_record_offset":21948,"ancestor_record_offset":21492,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":22028,"parent_record_offset":22004,"ancestor_record_offset":21492,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":21948,"parent_record_offset":21924,"ancestor_record_offset":21492,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$98::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":21924,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":28005,"source_line":2520,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":21692,"parent_record_offset":21668,"ancestor_record_offset":21492,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":21796,"parent_record_offset":21764,"ancestor_record_offset":21492,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":26148,"parent_record_offset":26108,"ancestor_record_offset":21492,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":23308,"parent_record_offset":23268,"ancestor_record_offset":21492,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":26108,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":28008,"source_line":182,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":23268,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":28007,"source_line":195,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":500,"record_offset":23864,"parent_record_offset":23840,"ancestor_record_offset":21492,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":23516,"parent_record_offset":23492,"ancestor_record_offset":21492,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":23380,"parent_record_offset":23356,"ancestor_record_offset":21492,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":23688,"parent_record_offset":23664,"ancestor_record_offset":21492,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":23356,"parent_record_offset":23332,"ancestor_record_offset":21492,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":23440,"parent_record_offset":23416,"ancestor_record_offset":21492,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":22952,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":29440,"parent_record_offset":29416,"ancestor_record_offset":21492,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":22868,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":500,"record_offset":23840,"parent_record_offset":23816,"ancestor_record_offset":21492,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":23816,"parent_record_offset":23792,"ancestor_record_offset":21492,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":500,"record_offset":21728,"parent_record_offset":21692,"ancestor_record_offset":21492,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":500,"record_offset":22056,"parent_record_offset":22028,"ancestor_record_offset":21492,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":30148,"parent_record_offset":29604,"ancestor_record_offset":21492,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":500,"record_offset":29860,"parent_record_offset":29836,"ancestor_record_offset":21492,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":500,"record_offset":29892,"parent_record_offset":29812,"ancestor_record_offset":21492,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":500,"record_offset":29836,"parent_record_offset":29812,"ancestor_record_offset":21492,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":500,"record_offset":23600,"parent_record_offset":23540,"ancestor_record_offset":21492,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":23624,"parent_record_offset":23600,"ancestor_record_offset":21492,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":23664,"parent_record_offset":23492,"ancestor_record_offset":21492,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":22080,"parent_record_offset":22056,"ancestor_record_offset":21492,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":22832,"parent_record_offset":22804,"ancestor_record_offset":21492,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":22892,"parent_record_offset":22868,"ancestor_record_offset":21492,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":29236,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":22916,"parent_record_offset":22892,"ancestor_record_offset":21492,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":23744,"parent_record_offset":23712,"ancestor_record_offset":21492,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":22804,"parent_record_offset":21796,"ancestor_record_offset":21492,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":500,"record_offset":29604,"parent_record_offset":29580,"ancestor_record_offset":21492,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":500,"record_offset":29644,"parent_record_offset":29604,"ancestor_record_offset":21492,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":29716,"parent_record_offset":29644,"ancestor_record_offset":21492,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":30172,"parent_record_offset":30148,"ancestor_record_offset":21492,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":29812,"parent_record_offset":29604,"ancestor_record_offset":21492,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":500,"record_offset":29464,"parent_record_offset":29440,"ancestor_record_offset":21492,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":500,"record_offset":23712,"parent_record_offset":23688,"ancestor_record_offset":21492,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::material_update_duck_splash
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":48376,"rva":9397632,"va":5378106752}
// 编译实例 RVAs: 0x8f6580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1416ca0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14497806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378106752,"raw_hex":"e80e38dd00","size":5,"soft_fail":false,"target_va":5392605424,"va":5378107613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12841674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378106752,"raw_hex":"e8caf2c300","size":5,"soft_fail":false,"target_va":5390948560,"va":5378106881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5300743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378106752,"raw_hex":"e807e25000","size":5,"soft_fail":false,"target_va":5383408256,"va":5378107508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":48656,"parent_record_offset":48376,"ancestor_record_offset":48376,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":500,"record_offset":48560,"parent_record_offset":48376,"ancestor_record_offset":48376,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":54716,"parent_record_offset":48376,"ancestor_record_offset":48376,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":500,"record_offset":54000,"parent_record_offset":48376,"ancestor_record_offset":48376,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":500,"record_offset":54056,"parent_record_offset":48376,"ancestor_record_offset":48376,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":53612,"parent_record_offset":53588,"ancestor_record_offset":48376,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":53660,"parent_record_offset":53636,"ancestor_record_offset":48376,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":53636,"parent_record_offset":53612,"ancestor_record_offset":48376,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":500,"record_offset":54024,"parent_record_offset":54000,"ancestor_record_offset":48376,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":53588,"parent_record_offset":53564,"ancestor_record_offset":48376,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":53564,"parent_record_offset":53540,"ancestor_record_offset":48376,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":49628,"parent_record_offset":49604,"ancestor_record_offset":48376,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":49652,"parent_record_offset":49628,"ancestor_record_offset":48376,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":53540,"parent_record_offset":53516,"ancestor_record_offset":48376,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":53516,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":49836,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":51612,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":53768,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":49752,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":48880,"parent_record_offset":48852,"ancestor_record_offset":48376,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":51872,"parent_record_offset":51736,"ancestor_record_offset":48376,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":50060,"parent_record_offset":49952,"ancestor_record_offset":48376,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":49132,"parent_record_offset":49108,"ancestor_record_offset":48376,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":51760,"parent_record_offset":51736,"ancestor_record_offset":48376,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":49976,"parent_record_offset":49952,"ancestor_record_offset":48376,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":48852,"parent_record_offset":48824,"ancestor_record_offset":48376,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":48904,"parent_record_offset":48880,"ancestor_record_offset":48376,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":48824,"parent_record_offset":48800,"ancestor_record_offset":48376,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":48800,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":48584,"parent_record_offset":48560,"ancestor_record_offset":48376,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":48688,"parent_record_offset":48656,"ancestor_record_offset":48376,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":51736,"parent_record_offset":51704,"ancestor_record_offset":48376,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":49952,"parent_record_offset":49920,"ancestor_record_offset":48376,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":51704,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":49920,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":500,"record_offset":50508,"parent_record_offset":50484,"ancestor_record_offset":48376,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":50160,"parent_record_offset":50136,"ancestor_record_offset":48376,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":50024,"parent_record_offset":50000,"ancestor_record_offset":48376,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":50332,"parent_record_offset":50308,"ancestor_record_offset":48376,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":50000,"parent_record_offset":49976,"ancestor_record_offset":48376,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":50084,"parent_record_offset":50060,"ancestor_record_offset":48376,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":49604,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":53916,"parent_record_offset":53892,"ancestor_record_offset":48376,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":49520,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":500,"record_offset":50484,"parent_record_offset":50460,"ancestor_record_offset":48376,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":50460,"parent_record_offset":50436,"ancestor_record_offset":48376,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":500,"record_offset":48620,"parent_record_offset":48584,"ancestor_record_offset":48376,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":500,"record_offset":48932,"parent_record_offset":48904,"ancestor_record_offset":48376,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":54624,"parent_record_offset":54080,"ancestor_record_offset":48376,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":500,"record_offset":54336,"parent_record_offset":54312,"ancestor_record_offset":48376,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":500,"record_offset":54368,"parent_record_offset":54288,"ancestor_record_offset":48376,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":500,"record_offset":54312,"parent_record_offset":54288,"ancestor_record_offset":48376,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":500,"record_offset":50244,"parent_record_offset":50184,"ancestor_record_offset":48376,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":50268,"parent_record_offset":50244,"ancestor_record_offset":48376,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":50308,"parent_record_offset":50136,"ancestor_record_offset":48376,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":48956,"parent_record_offset":48932,"ancestor_record_offset":48376,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":49484,"parent_record_offset":49456,"ancestor_record_offset":48376,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":49544,"parent_record_offset":49520,"ancestor_record_offset":48376,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":53712,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":49568,"parent_record_offset":49544,"ancestor_record_offset":48376,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":50388,"parent_record_offset":50356,"ancestor_record_offset":48376,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":49456,"parent_record_offset":48688,"ancestor_record_offset":48376,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":500,"record_offset":54080,"parent_record_offset":54056,"ancestor_record_offset":48376,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":500,"record_offset":54120,"parent_record_offset":54080,"ancestor_record_offset":48376,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":54192,"parent_record_offset":54120,"ancestor_record_offset":48376,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":54648,"parent_record_offset":54624,"ancestor_record_offset":48376,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":54288,"parent_record_offset":54080,"ancestor_record_offset":48376,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":500,"record_offset":53940,"parent_record_offset":53916,"ancestor_record_offset":48376,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":500,"record_offset":50356,"parent_record_offset":50332,"ancestor_record_offset":48376,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::material_update_sheep
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":30960,"rva":9391840,"va":5378100960}
// 编译实例 RVAs: 0x8f4ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3474814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378100960,"raw_hex":"e87e053500","size":5,"soft_fail":false,"target_va":5381577744,"va":5378102925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416c9de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14501422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378100960,"raw_hex":"e82e46dd00","size":5,"soft_fail":false,"target_va":5392604640,"va":5378103213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12847012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378100960,"raw_hex":"e8a407c400","size":5,"soft_fail":false,"target_va":5390948112,"va":5378101095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38518273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378100960,"raw_hex":"e801be4b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5378103322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5305632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378100960,"raw_hex":"e820f55000","size":5,"soft_fail":false,"target_va":5383408256,"va":5378102619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378100960,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5378102946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":31240,"parent_record_offset":30960,"ancestor_record_offset":30960,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":500,"record_offset":31144,"parent_record_offset":30960,"ancestor_record_offset":30960,"inlinee":10314,"source_line":2505,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":38456,"parent_record_offset":30960,"ancestor_record_offset":30960,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":39232,"parent_record_offset":30960,"ancestor_record_offset":30960,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":500,"record_offset":39172,"parent_record_offset":30960,"ancestor_record_offset":30960,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":500,"record_offset":37768,"parent_record_offset":30960,"ancestor_record_offset":30960,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":500,"record_offset":37796,"parent_record_offset":30960,"ancestor_record_offset":30960,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":34156,"parent_record_offset":34132,"ancestor_record_offset":30960,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":34204,"parent_record_offset":34180,"ancestor_record_offset":30960,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":500,"record_offset":34180,"parent_record_offset":34156,"ancestor_record_offset":30960,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":34132,"parent_record_offset":34108,"ancestor_record_offset":30960,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":34108,"parent_record_offset":34084,"ancestor_record_offset":30960,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":31548,"parent_record_offset":31524,"ancestor_record_offset":30960,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":31572,"parent_record_offset":31548,"ancestor_record_offset":30960,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":34084,"parent_record_offset":34060,"ancestor_record_offset":30960,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":34060,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":31524,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":31608,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":34312,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":500,"record_offset":31440,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":34412,"parent_record_offset":34388,"ancestor_record_offset":30960,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":31860,"parent_record_offset":31724,"ancestor_record_offset":30960,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":31884,"parent_record_offset":31860,"ancestor_record_offset":30960,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":37308,"parent_record_offset":37284,"ancestor_record_offset":30960,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":31748,"parent_record_offset":31724,"ancestor_record_offset":30960,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":31772,"parent_record_offset":31748,"ancestor_record_offset":30960,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":34388,"parent_record_offset":34364,"ancestor_record_offset":30960,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":34436,"parent_record_offset":34412,"ancestor_record_offset":30960,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":34364,"parent_record_offset":34340,"ancestor_record_offset":30960,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$95::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":500,"record_offset":34340,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":18908,"source_line":2520,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":31168,"parent_record_offset":31144,"ancestor_record_offset":30960,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":500,"record_offset":31272,"parent_record_offset":31240,"ancestor_record_offset":30960,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":31724,"parent_record_offset":31700,"ancestor_record_offset":30960,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":34792,"parent_record_offset":34768,"ancestor_record_offset":30960,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":31700,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":18906,"source_line":182,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":500,"record_offset":34768,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":18905,"source_line":195,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":500,"record_offset":32332,"parent_record_offset":32308,"ancestor_record_offset":30960,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":31984,"parent_record_offset":31960,"ancestor_record_offset":30960,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":31820,"parent_record_offset":31796,"ancestor_record_offset":30960,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":500,"record_offset":32156,"parent_record_offset":32132,"ancestor_record_offset":30960,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":31796,"parent_record_offset":31772,"ancestor_record_offset":30960,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":31908,"parent_record_offset":31884,"ancestor_record_offset":30960,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":34684,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":34460,"parent_record_offset":34436,"ancestor_record_offset":30960,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":34600,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":500,"record_offset":32308,"parent_record_offset":32284,"ancestor_record_offset":30960,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":500,"record_offset":32284,"parent_record_offset":32260,"ancestor_record_offset":30960,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":500,"record_offset":31204,"parent_record_offset":31168,"ancestor_record_offset":30960,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":500,"record_offset":37108,"parent_record_offset":37080,"ancestor_record_offset":30960,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":38364,"parent_record_offset":37820,"ancestor_record_offset":30960,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":500,"record_offset":38076,"parent_record_offset":38052,"ancestor_record_offset":30960,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":500,"record_offset":38108,"parent_record_offset":38028,"ancestor_record_offset":30960,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":500,"record_offset":38052,"parent_record_offset":38028,"ancestor_record_offset":30960,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":500,"record_offset":32068,"parent_record_offset":32008,"ancestor_record_offset":30960,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":32092,"parent_record_offset":32068,"ancestor_record_offset":30960,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":500,"record_offset":32132,"parent_record_offset":31960,"ancestor_record_offset":30960,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":500,"record_offset":37132,"parent_record_offset":37108,"ancestor_record_offset":30960,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":500,"record_offset":31404,"parent_record_offset":31376,"ancestor_record_offset":30960,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":31464,"parent_record_offset":31440,"ancestor_record_offset":30960,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":500,"record_offset":34256,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":31488,"parent_record_offset":31464,"ancestor_record_offset":30960,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":500,"record_offset":32212,"parent_record_offset":32180,"ancestor_record_offset":30960,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":500,"record_offset":31376,"parent_record_offset":31272,"ancestor_record_offset":30960,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":500,"record_offset":39196,"parent_record_offset":39172,"ancestor_record_offset":30960,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":500,"record_offset":37820,"parent_record_offset":37796,"ancestor_record_offset":30960,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":500,"record_offset":37860,"parent_record_offset":37820,"ancestor_record_offset":30960,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":37932,"parent_record_offset":37860,"ancestor_record_offset":30960,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":38388,"parent_record_offset":38364,"ancestor_record_offset":30960,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":500,"record_offset":38028,"parent_record_offset":37820,"ancestor_record_offset":30960,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":500,"record_offset":34484,"parent_record_offset":34460,"ancestor_record_offset":30960,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":500,"record_offset":32180,"parent_record_offset":32156,"ancestor_record_offset":30960,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::critters::duck_animated::MaterialData,country_core::shader::critters::duck_animated::MaterialData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1646,"record_offset":928996,"rva":23897056,"va":5392606176}
// 编译实例 RVAs: 0x16ca3e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140992ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13858803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392606176,"raw_hex":"e80d882cff","size":5,"soft_fail":false,"target_va":5378747616,"va":5392606414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14155ea90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1489443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392606176,"raw_hex":"e8dd45e9ff","size":5,"soft_fail":false,"target_va":5391116944,"va":5392606382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24015141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392606176,"raw_hex":"e825716e01","size":5,"soft_fail":false,"target_va":5416621600,"va":5392606454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-8903258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392606176,"raw_hex":"e8a62578ff","size":5,"soft_fail":false,"target_va":5383702992,"va":5392606245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":929384,"parent_record_offset":928996,"ancestor_record_offset":928996,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":929288,"parent_record_offset":928996,"ancestor_record_offset":928996,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":929316,"parent_record_offset":928996,"ancestor_record_offset":928996,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1646,"record_offset":929444,"parent_record_offset":928996,"ancestor_record_offset":928996,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update_dynamic
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":1646,"record_offset":929352,"parent_record_offset":928996,"ancestor_record_offset":928996,"inlinee":81558,"source_line":437,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":929412,"parent_record_offset":929384,"ancestor_record_offset":928996,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::critters::firefly::MaterialData<shabriri_rt::api::EmptyProtocol>,country_core::shader::critters::firefly::MaterialData<rhapsody::raw::binding_set::RawProtocol> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1646,"record_offset":927440,"rva":23895936,"va":5392605056}
// 编译实例 RVAs: 0x16c9f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140992ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13857629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605056,"raw_hex":"e8a38c2cff","size":5,"soft_fail":false,"target_va":5378747616,"va":5392605240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1325309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605056,"raw_hex":"e803c7ebff","size":5,"soft_fail":false,"target_va":5391279904,"va":5392605208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24016311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605056,"raw_hex":"e8b7756e01","size":5,"soft_fail":false,"target_va":5416621600,"va":5392605284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-8902149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605056,"raw_hex":"e8fb2978ff","size":5,"soft_fail":false,"target_va":5383702992,"va":5392605136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":927888,"parent_record_offset":927440,"ancestor_record_offset":927440,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":927792,"parent_record_offset":927440,"ancestor_record_offset":927440,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":927820,"parent_record_offset":927440,"ancestor_record_offset":927440,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1646,"record_offset":927948,"parent_record_offset":927440,"ancestor_record_offset":927440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update_dynamic
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":1646,"record_offset":927856,"parent_record_offset":927440,"ancestor_record_offset":927440,"inlinee":81558,"source_line":437,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":927916,"parent_record_offset":927888,"ancestor_record_offset":927440,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::critters::firefly_glow::MaterialData<shabriri_rt::api::EmptyProtocol>,country_core::shader::critters::firefly_glow::MaterialData<rhapsody::raw::binding_set::RawProtocol> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1646,"record_offset":928444,"rva":23896688,"va":5392605808}
// 编译实例 RVAs: 0x16ca270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140992ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13858384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605808,"raw_hex":"e8b0892cff","size":5,"soft_fail":false,"target_va":5378747616,"va":5392605995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a0820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1219312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605808,"raw_hex":"e81065edff","size":5,"soft_fail":false,"target_va":5391386656,"va":5392605963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24015560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605808,"raw_hex":"e8c8726e01","size":5,"soft_fail":false,"target_va":5416621600,"va":5392606035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-8902890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605808,"raw_hex":"e8162778ff","size":5,"soft_fail":false,"target_va":5383702992,"va":5392605877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":928904,"parent_record_offset":928444,"ancestor_record_offset":928444,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":928808,"parent_record_offset":928444,"ancestor_record_offset":928444,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":928836,"parent_record_offset":928444,"ancestor_record_offset":928444,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1646,"record_offset":928964,"parent_record_offset":928444,"ancestor_record_offset":928444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update_dynamic
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":1646,"record_offset":928872,"parent_record_offset":928444,"ancestor_record_offset":928444,"inlinee":81558,"source_line":437,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":928932,"parent_record_offset":928904,"ancestor_record_offset":928444,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::sheep::MaterialData<shabriri_rt::api::EmptyProtocol>,country_core::shader::sheep::MaterialData<rhapsody::raw::binding_set::RawProtocol> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1646,"record_offset":926924,"rva":23895520,"va":5392604640}
// 编译实例 RVAs: 0x16c9de0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140992ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13857267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392604640,"raw_hex":"e80d8e2cff","size":5,"soft_fail":false,"target_va":5378747616,"va":5392604878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a6160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1195347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392604640,"raw_hex":"e8adc2edff","size":5,"soft_fail":false,"target_va":5391409504,"va":5392604846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24016677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392604640,"raw_hex":"e825776e01","size":5,"soft_fail":false,"target_va":5416621600,"va":5392604918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-8901722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392604640,"raw_hex":"e8a62b78ff","size":5,"soft_fail":false,"target_va":5383702992,"va":5392604709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":927348,"parent_record_offset":926924,"ancestor_record_offset":926924,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":927252,"parent_record_offset":926924,"ancestor_record_offset":926924,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":927280,"parent_record_offset":926924,"ancestor_record_offset":926924,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1646,"record_offset":927408,"parent_record_offset":926924,"ancestor_record_offset":926924,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update_dynamic
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":1646,"record_offset":927316,"parent_record_offset":926924,"ancestor_record_offset":926924,"inlinee":81558,"source_line":437,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":927376,"parent_record_offset":927348,"ancestor_record_offset":926924,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::splash_particle::MaterialData,country_core::shader::splash_particle::MaterialData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1646,"record_offset":927980,"rva":23896304,"va":5392605424}
// 编译实例 RVAs: 0x16ca0f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140992ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13858014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605424,"raw_hex":"e8228b2cff","size":5,"soft_fail":false,"target_va":5378747616,"va":5392605625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415c9da0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1049598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605424,"raw_hex":"e802fcefff","size":5,"soft_fail":false,"target_va":5391556000,"va":5392605593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24015930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605424,"raw_hex":"e83a746e01","size":5,"soft_fail":false,"target_va":5416621600,"va":5392605665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-8902506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392605424,"raw_hex":"e8962878ff","size":5,"soft_fail":false,"target_va":5383702992,"va":5392605493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":928352,"parent_record_offset":927980,"ancestor_record_offset":927980,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":928256,"parent_record_offset":927980,"ancestor_record_offset":927980,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":928284,"parent_record_offset":927980,"ancestor_record_offset":927980,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1646,"record_offset":928412,"parent_record_offset":927980,"ancestor_record_offset":927980,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update_dynamic
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":1646,"record_offset":928320,"parent_record_offset":927980,"ancestor_record_offset":927980,"inlinee":81558,"source_line":437,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":928380,"parent_record_offset":928352,"ancestor_record_offset":927980,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::timed_terrain_gizmo::MaterialData,country_core::shader::timed_terrain_gizmo::MaterialData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1646,"record_offset":926452,"rva":23895136,"va":5392604256}
// 编译实例 RVAs: 0x16c9c60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140992ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13856848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392604256,"raw_hex":"e8b08f2cff","size":5,"soft_fail":false,"target_va":5378747616,"va":5392604459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415cc600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: update_material_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1038096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392604256,"raw_hex":"e8f028f0ff","size":5,"soft_fail":false,"target_va":5391566336,"va":5392604427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24017096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392604256,"raw_hex":"e8c8786e01","size":5,"soft_fail":false,"target_va":5416621600,"va":5392604499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::binding_set::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_binding_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-8901338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392604256,"raw_hex":"e8262d78ff","size":5,"soft_fail":false,"target_va":5383702992,"va":5392604325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":926832,"parent_record_offset":926452,"ancestor_record_offset":926452,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":926736,"parent_record_offset":926452,"ancestor_record_offset":926452,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1646,"record_offset":926764,"parent_record_offset":926452,"ancestor_record_offset":926452,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1646,"record_offset":926892,"parent_record_offset":926452,"ancestor_record_offset":926452,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update_dynamic
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":1646,"record_offset":926800,"parent_record_offset":926452,"ancestor_record_offset":926452,"inlinee":81558,"source_line":437,"archive":"_pdb/C/modules/01646.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1646,"record_offset":926860,"parent_record_offset":926832,"ancestor_record_offset":926452,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/01646.jsonl.gz"}
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
// 完整结构及原始类型记录: update_material_args.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: update_material_args.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: update_material_args.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::ArrowGizmo> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::ArrowGizmo> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::ArrowGizmo> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyGlowMarker> > >,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyGlowMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyMarker> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>", "bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyGlowMarker> > >", "bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyGlowMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyMarker> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyMarker>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyGlowMarker>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyGlowMarker>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyMarker>

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::critters::duck_animated::MaterialData,country_core::shader::critters::duck_animated::MaterialData>
// 信息 ID: 4dbc701ad784823173c81541
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::backend::update_material_args::impl$0::update -> ["country_core::shader::critters::duck_animated::MaterialData", "country_core::shader::critters::duck_animated::MaterialData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::update_material_args::material_update_duck @ 0x8f4a9b -> 0x16ca3e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::critters::firefly::MaterialData<shabriri_rt::api::EmptyProtocol>,country_core::shader::critters::firefly::MaterialData<rhapsody::raw::binding_set::RawProtocol> >
// 信息 ID: dc7fe61c25271c0e1ebbfeb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::critters::firefly::MaterialData -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// [类型关系] country_core::shader::critters::firefly::MaterialData -> ["rhapsody::raw::binding_set::RawProtocol"]; 字段布局 = unknown。
// [类型关系] country_core::systems::backend::update_material_args::impl$0::update -> ["country_core::shader::critters::firefly::MaterialData<shabriri_rt::api::EmptyProtocol>", "country_core::shader::critters::firefly::MaterialData<rhapsody::raw::binding_set::RawProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::update_material_args::material_update_firefly_glow @ 0x8f73f8 -> 0x16c9f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::critters::firefly_glow::MaterialData<shabriri_rt::api::EmptyProtocol>,country_core::shader::critters::firefly_glow::MaterialData<rhapsody::raw::binding_set::RawProtocol> >
// 信息 ID: fd066c58499a10be69fd70e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::critters::firefly_glow::MaterialData -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// [类型关系] country_core::shader::critters::firefly_glow::MaterialData -> ["rhapsody::raw::binding_set::RawProtocol"]; 字段布局 = unknown。
// [类型关系] country_core::systems::backend::update_material_args::impl$0::update -> ["country_core::shader::critters::firefly_glow::MaterialData<shabriri_rt::api::EmptyProtocol>", "country_core::shader::critters::firefly_glow::MaterialData<rhapsody::raw::binding_set::RawProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::update_material_args::material_update_firefly_glow @ 0x8f7395 -> 0x16ca270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::sheep::MaterialData<shabriri_rt::api::EmptyProtocol>,country_core::shader::sheep::MaterialData<rhapsody::raw::binding_set::RawProtocol> >
// 信息 ID: 54ce98915fed6c7bfc466c7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::shader::sheep::MaterialData -> ["shabriri_rt::api::EmptyProtocol"]; 字段布局 = unknown。
// [类型关系] country_core::shader::sheep::MaterialData -> ["rhapsody::raw::binding_set::RawProtocol"]; 字段布局 = unknown。
// [类型关系] country_core::systems::backend::update_material_args::impl$0::update -> ["country_core::shader::sheep::MaterialData<shabriri_rt::api::EmptyProtocol>", "country_core::shader::sheep::MaterialData<rhapsody::raw::binding_set::RawProtocol>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::update_material_args::material_update_sheep @ 0x8f57ad -> 0x16c9de0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::splash_particle::MaterialData,country_core::shader::splash_particle::MaterialData>
// 信息 ID: 27dab7b77f5176b124625141
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::backend::update_material_args::impl$0::update -> ["country_core::shader::splash_particle::MaterialData", "country_core::shader::splash_particle::MaterialData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::update_material_args::material_update_duck_splash @ 0x8f68dd -> 0x16ca0f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::impl$0::update<country_core::shader::timed_terrain_gizmo::MaterialData,country_core::shader::timed_terrain_gizmo::MaterialData>
// 信息 ID: 27aa8c6bd5b7de4bc72cf032
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::backend::update_material_args::impl$0::update -> ["country_core::shader::timed_terrain_gizmo::MaterialData", "country_core::shader::timed_terrain_gizmo::MaterialData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::update_material_args::material_update_arrow_gizmos @ 0x8f6f50 -> 0x16c9c60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::material_update_arrow_gizmos
// 信息 ID: c77ce874d0d5bff34025f522
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 39c890e8923cee7a56f9ce0e: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::ArrowGizmo> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::ArrowGizmo> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::ArrowGizmo> >,tuple$<> >)> > @ 0x1959e2e -> 0x8f6bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::ArrowGizmo> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::ArrowGizmo> >,tuple$<> >)> @ 0x2599e56 -> 0x8f6bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::material_update_duck
// 信息 ID: c90a7d74981c1b9de71ed35b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f18d26dd9ebd4f504e03d901: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >)> > @ 0x191a0f1 -> 0x8f4520 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::Duck>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >)> @ 0x259a199 -> 0x8f4520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::material_update_duck_splash
// 信息 ID: 96c926a8f8dc2db3bfec8194
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 238bf8b25a97b263c8555532: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >)> > @ 0x192aece -> 0x8f6580 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >)> @ 0x2599ff6 -> 0x8f6580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::material_update_firefly_glow
// 信息 ID: c4ccba5bcb485eb9b4b6d5f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fdbf918976fd48da58123832: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyGlowMarker> > >,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyGlowMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyMarker> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyGlowMarker> > >,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyGlowMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyMarker> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyGlowMarker> > >,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyGlowMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyMarker> > >)> > @ 0x1941789 -> 0x8f7260 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyGlowMarker> > >,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyGlowMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyMarker> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyGlowMarker> > >,bevy_ecs::system::query::Single<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref_mut$<country_core::components::drawable::Instanced> >,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::InstancedFireflyGlowMarker>,bevy_ecs::query::filter::Without<country_core::resources::critters::InstancedFireflyMarker> > >)> @ 0x25994ae -> 0x8f7260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::material_update_sheep
// 信息 ID: 9349d839471820b68dcb7327
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5b3e96906718c1b855651e58: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >)> > @ 0x194a5e8 -> 0x8f4ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >)> @ 0x2599b09 -> 0x8f4ee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::material_update_transforms
// 信息 ID: 6762956655cdbbf269667a1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0d9633d7844e7a7cec584789: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >)> > @ 0x194fd9e -> 0x8f5820 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >)> @ 0x2599cb6 -> 0x8f5820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::update_material_args::material_update_transforms::closure$0
// 信息 ID: 003ac10ac1913b327fe45b8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::backend::update_material_args::material_update_transforms", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::backend::update_material_args::material_update_transforms; module=500, inline_record=46268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
