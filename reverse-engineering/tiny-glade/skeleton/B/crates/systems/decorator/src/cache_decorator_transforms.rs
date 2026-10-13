// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/cache_decorator_transforms.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3715b6edc20020712fa0dcd762eac5d038e76f10588a0eff07317614b742439e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cache_decorator_transforms.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 458
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use petgraph; // namespace candidate
//   package identities: [{"name":"petgraph","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::cache_decorator_transforms::cache_decorator_transforms
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":458,"record_offset":2156,"rva":8475600,"va":5377184720}
// 编译实例 RVAs: 0x8153d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7920470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8aa2487ff","size":5,"soft_fail":false,"target_va":5369267664,"va":5377188129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140257ba0
//     owner=unresolved_namespace; count=5
//     target_aliases=3 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6022629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e81b1aa4ff","size":5,"soft_fail":false,"target_va":5371165600,"va":5377188224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408394b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t144794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e89a350200","size":5,"soft_fail":false,"target_va":5377332400,"va":5377187601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408e4750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t846323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8f3e90c00","size":5,"soft_fail":false,"target_va":5378033488,"va":5377187160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b100
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2120239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e82f5a2000","size":5,"soft_fail":false,"target_va":5379305728,"va":5377185484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2117897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e809512000","size":5,"soft_fail":false,"target_va":5379305984,"va":5377188082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4742961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8315f4800","size":5,"soft_fail":false,"target_va":5381934528,"va":5377191562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4748198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8a6734800","size":5,"soft_fail":false,"target_va":5381934560,"va":5377186357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9d180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4748572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e81c754800","size":5,"soft_fail":false,"target_va":5381935488,"va":5377186911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125d930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10779234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8627aa400","size":5,"soft_fail":false,"target_va":5387966768,"va":5377187529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125de50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10781048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e87881a400","size":5,"soft_fail":false,"target_va":5387968080,"va":5377187027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141269710
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10825128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8a82da500","size":5,"soft_fail":false,"target_va":5388015376,"va":5377190243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34656156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e89ccf1002","size":5,"soft_fail":false,"target_va":5411847776,"va":5377191615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34659518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8bedc1002","size":5,"soft_fail":false,"target_va":5411847808,"va":5377188285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34657140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e874d31002","size":5,"soft_fail":false,"target_va":5411848096,"va":5377190951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: cache_decorator_transforms.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34659866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e81ade1002","size":5,"soft_fail":false,"target_va":5411848144,"va":5377188273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34348589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e82d1e0c02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377192990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34348685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e88d1e0c02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377193086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34349496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8b8210c02","size":5,"soft_fail":false,"target_va":5411542422,"va":5377192921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t34349448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e888210c02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377192611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t34349110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e836200c02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377193181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34349146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e85a200c02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377192753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34349806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8ee220c02","size":5,"soft_fail":false,"target_va":5411542880,"va":5377193069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34350254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8ae240c02","size":5,"soft_fail":false,"target_va":5411543104,"va":5377192845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34350313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8e9240c02","size":5,"soft_fail":false,"target_va":5411543120,"va":5377192802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e868faffff","size":5,"soft_fail":false,"target_va":5377183456,"va":5377184883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34358395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e87b440c02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377184960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_meshes::clutter_mesh_ty_data
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t666673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8312c0a00","size":5,"soft_fail":false,"target_va":5377855120,"va":5377188442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t841659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8bbd70c00","size":5,"soft_fail":false,"target_va":5378028416,"va":5377186752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t843648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e880df0c00","size":5,"soft_fail":false,"target_va":5378030352,"va":5377186699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::get_position_from_mesh_uv
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t846734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e88eeb0c00","size":5,"soft_fail":false,"target_va":5378033360,"va":5377186621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2118446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e82e532000","size":5,"soft_fail":false,"target_va":5379305280,"va":5377186829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::insert
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2117094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8e64d2000","size":5,"soft_fail":false,"target_va":5379304864,"va":5377187765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::swap_buffers_and_clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2119676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8fc572000","size":5,"soft_fail":false,"target_va":5379305136,"va":5377185455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorRotationXZ::from_xz_forward_vector
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t117097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e869c90100","size":5,"soft_fail":false,"target_va":5377309184,"va":5377192082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorRotationXZ::offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t117473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8e1ca0100","size":5,"soft_fail":false,"target_va":5377309568,"va":5377192090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorRotationXZ::to_quat
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t122639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e80fdf0100","size":5,"soft_fail":false,"target_va":5377309376,"va":5377186732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2116206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e86e4a2000","size":5,"soft_fail":false,"target_va":5379305952,"va":5377189741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2125264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8d06d2000","size":5,"soft_fail":false,"target_va":5379313120,"va":5377187851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::corner_pos_and_normal_xz
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3231239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8074e3100","size":5,"soft_fail":false,"target_va":5380417536,"va":5377186292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2606373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e825c52700","size":5,"soft_fail":false,"target_va":5379792624,"va":5377186246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2607593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8e9c92700","size":5,"soft_fail":false,"target_va":5379793584,"va":5377185986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3210513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e811fd3000","size":5,"soft_fail":false,"target_va":5380396784,"va":5377186266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10394071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8d7999e00","size":5,"soft_fail":false,"target_va":5387581248,"va":5377187172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4200147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8d3164000","size":5,"soft_fail":false,"target_va":5381385536,"va":5377185384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4205336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8182b4000","size":5,"soft_fail":false,"target_va":5381390640,"va":5377185299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4194058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e80aff3f00","size":5,"soft_fail":false,"target_va":5381386480,"va":5377192417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4199841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8a1154000","size":5,"soft_fail":false,"target_va":5381384704,"va":5377184858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34452258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e822b30d02","size":5,"soft_fail":false,"target_va":5411644800,"va":5377192537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10748893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8dd03a400","size":5,"soft_fail":false,"target_va":5387942848,"va":5377193950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34344512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8400e0c02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377192587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::stairs_assemble::calculate_adjusted_ladder_positions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-253355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e85522fcff","size":5,"soft_fail":false,"target_va":5376937952,"va":5377191302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::components::transform::Transform::abs_diff_eq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4745655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8b7694800","size":5,"soft_fail":false,"target_va":5381935312,"va":5377189652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::closest_edge_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4737886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e85e4b4800","size":5,"soft_fail":false,"target_va":5381925104,"va":5377187213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::set_orientation_xz
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4727267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8e3214800","size":5,"soft_fail":false,"target_va":5381915936,"va":5377188664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::translate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4727134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e85e214800","size":5,"soft_fail":false,"target_va":5381915840,"va":5377188701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::get_tangent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4704881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e871ca4700","size":5,"soft_fail":false,"target_va":5381891312,"va":5377186426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4703733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e8f5c54700","size":5,"soft_fail":false,"target_va":5381890192,"va":5377186454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4699524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e884b54700","size":5,"soft_fail":false,"target_va":5381885904,"va":5377186375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_from_forward
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4697397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377184720,"raw_hex":"e835ad4700","size":5,"soft_fail":false,"target_va":5381886944,"va":5377189542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":18204,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":458,"record_offset":4920,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":458,"record_offset":18588,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":458,"record_offset":16800,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":458,"record_offset":14960,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":458,"record_offset":14932,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":458,"record_offset":5008,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":458,"record_offset":4980,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":6804,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":17580,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":9380,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":11452,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":9276,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":20328,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":458,"record_offset":18412,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":458,"record_offset":16196,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4466,"source_line":296,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":458,"record_offset":16712,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":458,"record_offset":12124,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4643,"source_line":118,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::Decorators::iter_maybe_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":458,"record_offset":11620,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":25693,"source_line":217,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":458,"record_offset":12244,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":25694,"source_line":168,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":458,"record_offset":12684,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":458,"record_offset":10392,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":13688,"source_line":894,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":13308,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":17364,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4745,"source_line":516,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":17552,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":13748,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":13924,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":14132,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":458,"record_offset":19148,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":458,"record_offset":19708,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":458,"record_offset":19796,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::GraphMap::edges
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":458,"record_offset":9448,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4971,"source_line":494,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] petgraph::graphmap::impl$11::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":458,"record_offset":11000,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":10647,"source_line":809,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":458,"record_offset":3936,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":458,"record_offset":2504,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":458,"record_offset":2960,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$26::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":458,"record_offset":16828,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":25698,"source_line":2030,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$50::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":458,"record_offset":16556,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4599,"source_line":2307,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":458,"record_offset":5768,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":458,"record_offset":6128,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":458,"record_offset":19540,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":458,"record_offset":18560,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cache_decorator_transforms::roof_decorator_normal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cache_decorator_transforms.rs
//     evidence={"module":458,"record_offset":9740,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":25691,"source_line":451,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::from_translation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":458,"record_offset":9416,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":24594,"source_line":44,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":458,"record_offset":17644,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":458,"record_offset":16680,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":458,"record_offset":16344,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":458,"record_offset":18476,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":458,"record_offset":16924,"parent_record_offset":2156,"ancestor_record_offset":2156,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":7188,"parent_record_offset":7164,"ancestor_record_offset":2156,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":7236,"parent_record_offset":7212,"ancestor_record_offset":2156,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":7212,"parent_record_offset":7188,"ancestor_record_offset":2156,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":2156,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":7140,"parent_record_offset":7040,"ancestor_record_offset":2156,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":16972,"parent_record_offset":16924,"ancestor_record_offset":2156,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":7576,"parent_record_offset":7552,"ancestor_record_offset":2156,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":458,"record_offset":7340,"parent_record_offset":7008,"ancestor_record_offset":2156,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":458,"record_offset":4948,"parent_record_offset":4920,"ancestor_record_offset":2156,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":458,"record_offset":7008,"parent_record_offset":6936,"ancestor_record_offset":2156,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":8912,"parent_record_offset":8888,"ancestor_record_offset":2156,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":7844,"parent_record_offset":7812,"ancestor_record_offset":2156,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":7872,"parent_record_offset":7844,"ancestor_record_offset":2156,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":8888,"parent_record_offset":8864,"ancestor_record_offset":2156,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":8936,"parent_record_offset":8912,"ancestor_record_offset":2156,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":8864,"parent_record_offset":6936,"ancestor_record_offset":2156,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":458,"record_offset":7812,"parent_record_offset":6936,"ancestor_record_offset":2156,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":458,"record_offset":8316,"parent_record_offset":8292,"ancestor_record_offset":2156,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":458,"record_offset":7988,"parent_record_offset":7960,"ancestor_record_offset":2156,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":458,"record_offset":7960,"parent_record_offset":7932,"ancestor_record_offset":2156,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":458,"record_offset":8176,"parent_record_offset":8152,"ancestor_record_offset":2156,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":458,"record_offset":7932,"parent_record_offset":7900,"ancestor_record_offset":2156,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":458,"record_offset":7900,"parent_record_offset":7872,"ancestor_record_offset":2156,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":458,"record_offset":7552,"parent_record_offset":6936,"ancestor_record_offset":2156,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":458,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":2156,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":458,"record_offset":8268,"parent_record_offset":8244,"ancestor_record_offset":2156,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":458,"record_offset":6900,"parent_record_offset":6844,"ancestor_record_offset":2156,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":458,"record_offset":6936,"parent_record_offset":6900,"ancestor_record_offset":2156,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":458,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":2156,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":458,"record_offset":12156,"parent_record_offset":12124,"ancestor_record_offset":2156,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":458,"record_offset":12180,"parent_record_offset":12156,"ancestor_record_offset":2156,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":458,"record_offset":12204,"parent_record_offset":12180,"ancestor_record_offset":2156,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":458,"record_offset":7400,"parent_record_offset":7372,"ancestor_record_offset":2156,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":458,"record_offset":4272,"parent_record_offset":4244,"ancestor_record_offset":2156,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":458,"record_offset":4324,"parent_record_offset":4300,"ancestor_record_offset":2156,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":458,"record_offset":4300,"parent_record_offset":4272,"ancestor_record_offset":2156,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":458,"record_offset":4220,"parent_record_offset":4192,"ancestor_record_offset":2156,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":458,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":2156,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":458,"record_offset":4416,"parent_record_offset":4244,"ancestor_record_offset":2156,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":458,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":2156,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":458,"record_offset":16432,"parent_record_offset":16408,"ancestor_record_offset":2156,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":458,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":2156,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":458,"record_offset":16456,"parent_record_offset":16432,"ancestor_record_offset":2156,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":458,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":2156,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":458,"record_offset":5516,"parent_record_offset":5480,"ancestor_record_offset":2156,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":458,"record_offset":10792,"parent_record_offset":10420,"ancestor_record_offset":2156,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":458,"record_offset":13036,"parent_record_offset":12980,"ancestor_record_offset":2156,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":458,"record_offset":10616,"parent_record_offset":10420,"ancestor_record_offset":2156,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":458,"record_offset":10488,"parent_record_offset":10420,"ancestor_record_offset":2156,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":458,"record_offset":12900,"parent_record_offset":12708,"ancestor_record_offset":2156,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":458,"record_offset":16496,"parent_record_offset":16344,"ancestor_record_offset":2156,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":458,"record_offset":16524,"parent_record_offset":16344,"ancestor_record_offset":2156,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":458,"record_offset":11052,"parent_record_offset":11028,"ancestor_record_offset":2156,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":458,"record_offset":11076,"parent_record_offset":11052,"ancestor_record_offset":2156,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":458,"record_offset":11148,"parent_record_offset":11124,"ancestor_record_offset":2156,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":458,"record_offset":11100,"parent_record_offset":11076,"ancestor_record_offset":2156,"inlinee":10601,"source_line":4049,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::spec_try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":458,"record_offset":11124,"parent_record_offset":11100,"ancestor_record_offset":2156,"inlinee":10603,"source_line":4098,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":458,"record_offset":4348,"parent_record_offset":4324,"ancestor_record_offset":2156,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":458,"record_offset":18368,"parent_record_offset":18328,"ancestor_record_offset":2156,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":458,"record_offset":2660,"parent_record_offset":2628,"ancestor_record_offset":2156,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":458,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":2156,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":458,"record_offset":5296,"parent_record_offset":5268,"ancestor_record_offset":2156,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":458,"record_offset":7284,"parent_record_offset":7008,"ancestor_record_offset":2156,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":458,"record_offset":9868,"parent_record_offset":9836,"ancestor_record_offset":2156,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":458,"record_offset":10324,"parent_record_offset":10192,"ancestor_record_offset":2156,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":458,"record_offset":19824,"parent_record_offset":19796,"ancestor_record_offset":2156,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":458,"record_offset":9980,"parent_record_offset":9952,"ancestor_record_offset":2156,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":458,"record_offset":4088,"parent_record_offset":4044,"ancestor_record_offset":2156,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":458,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":2156,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":458,"record_offset":16408,"parent_record_offset":16384,"ancestor_record_offset":2156,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":458,"record_offset":16384,"parent_record_offset":16344,"ancestor_record_offset":2156,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":458,"record_offset":18300,"parent_record_offset":18268,"ancestor_record_offset":2156,"inlinee":4608,"source_line":454,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":2156,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":8108,"parent_record_offset":8080,"ancestor_record_offset":2156,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":5708,"parent_record_offset":5008,"ancestor_record_offset":2156,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":9208,"parent_record_offset":6844,"ancestor_record_offset":2156,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":458,"record_offset":2908,"parent_record_offset":2880,"ancestor_record_offset":2156,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":458,"record_offset":4876,"parent_record_offset":4044,"ancestor_record_offset":2156,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":4480,"parent_record_offset":4192,"ancestor_record_offset":2156,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":458,"record_offset":7504,"parent_record_offset":7472,"ancestor_record_offset":2156,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":458,"record_offset":5576,"parent_record_offset":5552,"ancestor_record_offset":2156,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":458,"record_offset":11288,"parent_record_offset":11264,"ancestor_record_offset":2156,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":458,"record_offset":3372,"parent_record_offset":3348,"ancestor_record_offset":2156,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":458,"record_offset":3348,"parent_record_offset":3324,"ancestor_record_offset":2156,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":458,"record_offset":3556,"parent_record_offset":3528,"ancestor_record_offset":2156,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":2156,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":458,"record_offset":9240,"parent_record_offset":6844,"ancestor_record_offset":2156,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":458,"record_offset":9172,"parent_record_offset":6936,"ancestor_record_offset":2156,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":458,"record_offset":7040,"parent_record_offset":7008,"ancestor_record_offset":2156,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":458,"record_offset":7108,"parent_record_offset":7080,"ancestor_record_offset":2156,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":458,"record_offset":19076,"parent_record_offset":19004,"ancestor_record_offset":2156,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":458,"record_offset":16856,"parent_record_offset":16828,"ancestor_record_offset":2156,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":458,"record_offset":18268,"parent_record_offset":18236,"ancestor_record_offset":2156,"inlinee":4606,"source_line":680,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":458,"record_offset":7080,"parent_record_offset":7040,"ancestor_record_offset":2156,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":458,"record_offset":16884,"parent_record_offset":16856,"ancestor_record_offset":2156,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":458,"record_offset":18328,"parent_record_offset":18268,"ancestor_record_offset":2156,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":458,"record_offset":9672,"parent_record_offset":9500,"ancestor_record_offset":2156,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":458,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":2156,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":458,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":2156,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":458,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":2156,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":458,"record_offset":3628,"parent_record_offset":3600,"ancestor_record_offset":2156,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":458,"record_offset":11176,"parent_record_offset":11148,"ancestor_record_offset":2156,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":458,"record_offset":3276,"parent_record_offset":3160,"ancestor_record_offset":2156,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":458,"record_offset":11240,"parent_record_offset":11176,"ancestor_record_offset":2156,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":458,"record_offset":2628,"parent_record_offset":2600,"ancestor_record_offset":2156,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":458,"record_offset":2988,"parent_record_offset":2960,"ancestor_record_offset":2156,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":458,"record_offset":3128,"parent_record_offset":3096,"ancestor_record_offset":2156,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":458,"record_offset":3096,"parent_record_offset":3060,"ancestor_record_offset":2156,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":458,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":2156,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":458,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":2156,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":458,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":2156,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":458,"record_offset":3600,"parent_record_offset":3060,"ancestor_record_offset":2156,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":458,"record_offset":2600,"parent_record_offset":2568,"ancestor_record_offset":2156,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":458,"record_offset":2568,"parent_record_offset":2540,"ancestor_record_offset":2156,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":458,"record_offset":2760,"parent_record_offset":2728,"ancestor_record_offset":2156,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":458,"record_offset":2856,"parent_record_offset":2760,"ancestor_record_offset":2156,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":458,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":2156,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":458,"record_offset":2540,"parent_record_offset":2504,"ancestor_record_offset":2156,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":458,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":2156,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":458,"record_offset":3160,"parent_record_offset":3128,"ancestor_record_offset":2156,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":458,"record_offset":3444,"parent_record_offset":3160,"ancestor_record_offset":2156,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":458,"record_offset":3416,"parent_record_offset":3160,"ancestor_record_offset":2156,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":458,"record_offset":3220,"parent_record_offset":3160,"ancestor_record_offset":2156,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":458,"record_offset":16308,"parent_record_offset":16284,"ancestor_record_offset":2156,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":458,"record_offset":7372,"parent_record_offset":6936,"ancestor_record_offset":2156,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":458,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":2156,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":458,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":2156,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":458,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":2156,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":458,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":2156,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":458,"record_offset":12360,"parent_record_offset":12244,"ancestor_record_offset":2156,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":458,"record_offset":10420,"parent_record_offset":10392,"ancestor_record_offset":2156,"inlinee":13682,"source_line":727,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":458,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":2156,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":458,"record_offset":13156,"parent_record_offset":12708,"ancestor_record_offset":2156,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":9928,"parent_record_offset":9740,"ancestor_record_offset":2156,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":10008,"parent_record_offset":9952,"ancestor_record_offset":2156,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":10224,"parent_record_offset":10192,"ancestor_record_offset":2156,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":10192,"parent_record_offset":10168,"ancestor_record_offset":2156,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":10168,"parent_record_offset":9740,"ancestor_record_offset":2156,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":17388,"parent_record_offset":17364,"ancestor_record_offset":2156,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":17248,"parent_record_offset":16924,"ancestor_record_offset":2156,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":17304,"parent_record_offset":16924,"ancestor_record_offset":2156,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":2156,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":458,"record_offset":19248,"parent_record_offset":19176,"ancestor_record_offset":2156,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":458,"record_offset":17920,"parent_record_offset":17892,"ancestor_record_offset":2156,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":458,"record_offset":17948,"parent_record_offset":17920,"ancestor_record_offset":2156,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":458,"record_offset":9836,"parent_record_offset":9740,"ancestor_record_offset":2156,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":458,"record_offset":19176,"parent_record_offset":19148,"ancestor_record_offset":2156,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":458,"record_offset":12780,"parent_record_offset":12752,"ancestor_record_offset":2156,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":458,"record_offset":12752,"parent_record_offset":12708,"ancestor_record_offset":2156,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":458,"record_offset":5232,"parent_record_offset":5196,"ancestor_record_offset":2156,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":458,"record_offset":5268,"parent_record_offset":5232,"ancestor_record_offset":2156,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":458,"record_offset":5364,"parent_record_offset":5196,"ancestor_record_offset":2156,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":458,"record_offset":5196,"parent_record_offset":5144,"ancestor_record_offset":2156,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":458,"record_offset":5396,"parent_record_offset":5144,"ancestor_record_offset":2156,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":458,"record_offset":5480,"parent_record_offset":5452,"ancestor_record_offset":2156,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":458,"record_offset":5452,"parent_record_offset":5144,"ancestor_record_offset":2156,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":458,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":2156,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":458,"record_offset":11684,"parent_record_offset":11656,"ancestor_record_offset":2156,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":458,"record_offset":5072,"parent_record_offset":5040,"ancestor_record_offset":2156,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":458,"record_offset":5552,"parent_record_offset":5144,"ancestor_record_offset":2156,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":458,"record_offset":5924,"parent_record_offset":5864,"ancestor_record_offset":2156,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":458,"record_offset":5144,"parent_record_offset":5104,"ancestor_record_offset":2156,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":458,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":2156,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":458,"record_offset":11804,"parent_record_offset":11740,"ancestor_record_offset":2156,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":458,"record_offset":5896,"parent_record_offset":5864,"ancestor_record_offset":2156,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":458,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":2156,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":458,"record_offset":5104,"parent_record_offset":5072,"ancestor_record_offset":2156,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":458,"record_offset":8200,"parent_record_offset":8176,"ancestor_record_offset":2156,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::GraphMap::edge_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":458,"record_offset":11412,"parent_record_offset":11380,"ancestor_record_offset":2156,"inlinee":4651,"source_line":234,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::GraphMap::neighbors
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":458,"record_offset":9472,"parent_record_offset":9448,"ancestor_record_offset":2156,"inlinee":4972,"source_line":454,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$11::next::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":458,"record_offset":11380,"parent_record_offset":11352,"ancestor_record_offset":2156,"inlinee":10605,"source_line":810,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] petgraph::graphmap::impl$9::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"petgraph","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/petgraph-0.8.1/src/graphmap.rs
//     evidence={"module":458,"record_offset":11028,"parent_record_offset":11000,"ancestor_record_offset":2156,"inlinee":10600,"source_line":719,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":458,"record_offset":3972,"parent_record_offset":3936,"ancestor_record_offset":2156,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":458,"record_offset":20392,"parent_record_offset":20360,"ancestor_record_offset":2156,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":458,"record_offset":20908,"parent_record_offset":20648,"ancestor_record_offset":2156,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":458,"record_offset":4116,"parent_record_offset":4088,"ancestor_record_offset":2156,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":458,"record_offset":4192,"parent_record_offset":4044,"ancestor_record_offset":2156,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":458,"record_offset":4452,"parent_record_offset":4192,"ancestor_record_offset":2156,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":458,"record_offset":16640,"parent_record_offset":16612,"ancestor_record_offset":2156,"inlinee":4604,"source_line":1001,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":458,"record_offset":16612,"parent_record_offset":16584,"ancestor_record_offset":2156,"inlinee":4603,"source_line":971,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":458,"record_offset":16584,"parent_record_offset":16556,"ancestor_record_offset":2156,"inlinee":4601,"source_line":1846,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":458,"record_offset":5792,"parent_record_offset":5768,"ancestor_record_offset":2156,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":458,"record_offset":11656,"parent_record_offset":11620,"ancestor_record_offset":2156,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":458,"record_offset":5040,"parent_record_offset":5008,"ancestor_record_offset":2156,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":458,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":2156,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":458,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":2156,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":458,"record_offset":10040,"parent_record_offset":10008,"ancestor_record_offset":2156,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":458,"record_offset":4144,"parent_record_offset":4116,"ancestor_record_offset":2156,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":458,"record_offset":4044,"parent_record_offset":4008,"ancestor_record_offset":2156,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":458,"record_offset":4008,"parent_record_offset":3972,"ancestor_record_offset":2156,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cache_decorator_transforms.rs
//     evidence={"module":458,"record_offset":9344,"parent_record_offset":9312,"ancestor_record_offset":2156,"inlinee":25689,"source_line":63,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cache_decorator_transforms.rs
//     evidence={"module":458,"record_offset":6844,"parent_record_offset":6804,"ancestor_record_offset":2156,"inlinee":25688,"source_line":90,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cache_decorator_transforms.rs
//     evidence={"module":458,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":2156,"inlinee":25692,"source_line":130,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cache_decorator_transforms.rs
//     evidence={"module":458,"record_offset":15548,"parent_record_offset":15524,"ancestor_record_offset":2156,"inlinee":25695,"source_line":241,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cache_decorator_transforms.rs
//     evidence={"module":458,"record_offset":16284,"parent_record_offset":16196,"ancestor_record_offset":2156,"inlinee":25696,"source_line":281,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":458,"record_offset":9800,"parent_record_offset":9740,"ancestor_record_offset":2156,"inlinee":25174,"source_line":32,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":458,"record_offset":17068,"parent_record_offset":16924,"ancestor_record_offset":2156,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":458,"record_offset":17892,"parent_record_offset":17644,"ancestor_record_offset":2156,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6459
// 首条位置: {"module":458,"inlinee":25689}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6458
// 首条位置: {"module":458,"inlinee":25688}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":458,"record_offset":6900,"parent_record_offset":6844,"ancestor_record_offset":6844,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":9208,"parent_record_offset":6844,"ancestor_record_offset":6844,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":458,"record_offset":9240,"parent_record_offset":6844,"ancestor_record_offset":6844,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":7188,"parent_record_offset":7164,"ancestor_record_offset":6844,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":7236,"parent_record_offset":7212,"ancestor_record_offset":6844,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":7212,"parent_record_offset":7188,"ancestor_record_offset":6844,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":6844,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":7140,"parent_record_offset":7040,"ancestor_record_offset":6844,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":7576,"parent_record_offset":7552,"ancestor_record_offset":6844,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":458,"record_offset":7340,"parent_record_offset":7008,"ancestor_record_offset":6844,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":458,"record_offset":7008,"parent_record_offset":6936,"ancestor_record_offset":6844,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":8912,"parent_record_offset":8888,"ancestor_record_offset":6844,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":7844,"parent_record_offset":7812,"ancestor_record_offset":6844,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":7872,"parent_record_offset":7844,"ancestor_record_offset":6844,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":8888,"parent_record_offset":8864,"ancestor_record_offset":6844,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":8936,"parent_record_offset":8912,"ancestor_record_offset":6844,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":458,"record_offset":8864,"parent_record_offset":6936,"ancestor_record_offset":6844,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":458,"record_offset":7812,"parent_record_offset":6936,"ancestor_record_offset":6844,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":458,"record_offset":8316,"parent_record_offset":8292,"ancestor_record_offset":6844,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":458,"record_offset":7988,"parent_record_offset":7960,"ancestor_record_offset":6844,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":458,"record_offset":7960,"parent_record_offset":7932,"ancestor_record_offset":6844,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":458,"record_offset":8176,"parent_record_offset":8152,"ancestor_record_offset":6844,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":458,"record_offset":7932,"parent_record_offset":7900,"ancestor_record_offset":6844,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":458,"record_offset":7900,"parent_record_offset":7872,"ancestor_record_offset":6844,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":458,"record_offset":7552,"parent_record_offset":6936,"ancestor_record_offset":6844,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":458,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":6844,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":458,"record_offset":8268,"parent_record_offset":8244,"ancestor_record_offset":6844,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":458,"record_offset":6936,"parent_record_offset":6900,"ancestor_record_offset":6844,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":458,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":6844,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":458,"record_offset":7400,"parent_record_offset":7372,"ancestor_record_offset":6844,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":458,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":6844,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":458,"record_offset":7284,"parent_record_offset":7008,"ancestor_record_offset":6844,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":458,"record_offset":8080,"parent_record_offset":8016,"ancestor_record_offset":6844,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":6844,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":8108,"parent_record_offset":8080,"ancestor_record_offset":6844,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":458,"record_offset":8152,"parent_record_offset":7960,"ancestor_record_offset":6844,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":458,"record_offset":7504,"parent_record_offset":7472,"ancestor_record_offset":6844,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":458,"record_offset":9172,"parent_record_offset":6936,"ancestor_record_offset":6844,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":458,"record_offset":7040,"parent_record_offset":7008,"ancestor_record_offset":6844,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":458,"record_offset":7108,"parent_record_offset":7080,"ancestor_record_offset":6844,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":458,"record_offset":7080,"parent_record_offset":7040,"ancestor_record_offset":6844,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":458,"record_offset":7372,"parent_record_offset":6936,"ancestor_record_offset":6844,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":458,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":6844,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":458,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":6844,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":458,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":6844,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":458,"record_offset":8200,"parent_record_offset":8176,"ancestor_record_offset":6844,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x645C
// 首条位置: {"module":458,"inlinee":25692}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x645F
// 首条位置: {"module":458,"inlinee":25695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6460
// 首条位置: {"module":458,"inlinee":25696}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":458,"record_offset":16308,"parent_record_offset":16284,"ancestor_record_offset":16284,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::cache_decorator_transforms::roof_decorator_normal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 451
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x645B
// 首条位置: {"module":458,"inlinee":25691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":9928,"parent_record_offset":9740,"ancestor_record_offset":9740,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":10168,"parent_record_offset":9740,"ancestor_record_offset":9740,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":10080,"parent_record_offset":9740,"ancestor_record_offset":9740,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":458,"record_offset":9836,"parent_record_offset":9740,"ancestor_record_offset":9740,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":458,"record_offset":9800,"parent_record_offset":9740,"ancestor_record_offset":9740,"inlinee":25174,"source_line":32,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":458,"record_offset":9868,"parent_record_offset":9836,"ancestor_record_offset":9740,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":458,"record_offset":10324,"parent_record_offset":10192,"ancestor_record_offset":9740,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":458,"record_offset":9980,"parent_record_offset":9952,"ancestor_record_offset":9740,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":10008,"parent_record_offset":9952,"ancestor_record_offset":9740,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":10224,"parent_record_offset":10192,"ancestor_record_offset":9740,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":10192,"parent_record_offset":10168,"ancestor_record_offset":9740,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":458,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":9740,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":458,"record_offset":10040,"parent_record_offset":10008,"ancestor_record_offset":9740,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00458.jsonl.gz"}
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
// 完整结构及原始类型记录: cache_decorator_transforms.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cache_decorator_transforms.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cache_decorator_transforms.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::system::system_param::Local<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::system::system_param::Local<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>

// ------------------------------------------------------------------
// 函数: system_decorator::cache_decorator_transforms::cache_decorator_transforms
// 信息 ID: 271a34bc41c91fca1ab9a20b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f9ea3bd7ad9711ee362baef4: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::system::system_param::Local<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::system::system_param::Local<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::system::system_param::Local<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > @ 0xc552b -> 0x8153d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::system::system_param::Local<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::system::system_param::Local<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransformStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x441cf1 -> 0x8153d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$0
// 信息 ID: 2a37865684b7c13e85fdb859
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::cache_decorator_transforms::cache_decorator_transforms", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cache_decorator_transforms::cache_decorator_transforms; module=458, inline_record=9344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$1
// 信息 ID: 5d3431a26474caa0e48998d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::cache_decorator_transforms::cache_decorator_transforms", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cache_decorator_transforms::cache_decorator_transforms; module=458, inline_record=6844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$2
// 信息 ID: 23af4cd87f70217d1dae03d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::cache_decorator_transforms::cache_decorator_transforms", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cache_decorator_transforms::cache_decorator_transforms; module=458, inline_record=11476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$3
// 信息 ID: a26c34653f3dd229dfb8eb8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::cache_decorator_transforms::cache_decorator_transforms", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cache_decorator_transforms::cache_decorator_transforms; module=458, inline_record=15548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cache_decorator_transforms::cache_decorator_transforms::closure$4
// 信息 ID: 42e056c996570266ec263942
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::cache_decorator_transforms::cache_decorator_transforms", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cache_decorator_transforms::cache_decorator_transforms; module=458, inline_record=16284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::cache_decorator_transforms::roof_decorator_normal
// 信息 ID: 4636f45c1788eb1544cb3bf9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cache_decorator_transforms::cache_decorator_transforms; module=458, inline_record=9740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
