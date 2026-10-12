// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/move_decorators_following_anchors.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a80ef77106e544bca85a267025b77b9e667edd3b8aacaf2f96cf5550e894e54e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: move_decorators_following_anchors.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 492
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
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
// 函数符号: system_decorator::move_decorators_following_anchors::move_decorators_following_anchors
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":492,"record_offset":272,"rva":9321968,"va":5378031088}
// 编译实例 RVAs: 0x8e3df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8490627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e87d717eff","size":5,"soft_fail":false,"target_va":5369542000,"va":5378032622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040ebd0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5068420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e87ca9b2ff","size":5,"soft_fail":false,"target_va":5372963792,"va":5378032207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4554288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8d081baff","size":5,"soft_fail":false,"target_va":5373479040,"va":5378033323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t92990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e83e6b0100","size":5,"soft_fail":false,"target_va":5378126944,"va":5378033949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a4ec70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1484965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8a5a81600","size":5,"soft_fail":false,"target_va":5379517552,"va":5378032582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a4ed00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1484190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e89ea51600","size":5,"soft_fail":false,"target_va":5379517696,"va":5378033501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a4ee40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1484731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8bba71600","size":5,"soft_fail":false,"target_va":5379518016,"va":5378033280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3628890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e85a5f3700","size":5,"soft_fail":false,"target_va":5381660768,"va":5378031873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5249439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e89f195000","size":5,"soft_fail":false,"target_va":5383283184,"va":5378033740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141499c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12276887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e89754bb00","size":5,"soft_fail":false,"target_va":5390310496,"va":5378033604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12306582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e896c8bb00","size":5,"soft_fail":false,"target_va":5390340576,"va":5378033989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414dff60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: move_decorators_following_anchors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12564108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e88cb6bf00","size":5,"soft_fail":false,"target_va":5390597984,"va":5378033871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38586935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e837ca4c02","size":5,"soft_fail":false,"target_va":5416621408,"va":5378034468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38587282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e892cb4c02","size":5,"soft_fail":false,"target_va":5416621600,"va":5378034313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38587813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8a5cd4c02","size":5,"soft_fail":false,"target_va":5416622208,"va":5378034390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38587221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e855cb4c02","size":5,"soft_fail":false,"target_va":5416621762,"va":5378034536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38587875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8e3cd4c02","size":5,"soft_fail":false,"target_va":5416622240,"va":5378034360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e87a100000","size":5,"soft_fail":false,"target_va":5378035472,"va":5378031249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38586698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e84ac94c02","size":5,"soft_fail":false,"target_va":5416621376,"va":5378034673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorDst::as_wall_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t92763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e85b6a0100","size":5,"soft_fail":false,"target_va":5378126320,"va":5378033552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1475390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e83e831600","size":5,"soft_fail":false,"target_va":5379508032,"va":5378032637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter_walls_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1449900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8ac1f1600","size":5,"soft_fail":false,"target_va":5379482992,"va":5378033087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_removed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1452938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e88a2b1600","size":5,"soft_fail":false,"target_va":5379484768,"va":5378031825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1466703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e84f611600","size":5,"soft_fail":false,"target_va":5379498880,"va":5378032172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1442101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e835011600","size":5,"soft_fail":false,"target_va":5379475376,"va":5378033270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::mark_all_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1443265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8c1051600","size":5,"soft_fail":false,"target_va":5379475456,"va":5378032186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3616899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e883303700","size":5,"soft_fail":false,"target_va":5381650864,"va":5378033960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::try_get_subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3620311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8d73d3700","size":5,"soft_fail":false,"target_va":5381654096,"va":5378033780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3198930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8d2cf3000","size":5,"soft_fail":false,"target_va":5381232512,"va":5378033577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3200520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e808d63000","size":5,"soft_fail":false,"target_va":5381234608,"va":5378034083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3198025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e849cc3000","size":5,"soft_fail":false,"target_va":5381231744,"va":5378033714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5932201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8a9845a00","size":5,"soft_fail":false,"target_va":5383963792,"va":5378031586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5940708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8e4a55a00","size":5,"soft_fail":false,"target_va":5383972224,"va":5378031511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5930715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e8db7e5a00","size":5,"soft_fail":false,"target_va":5383964912,"va":5378034192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5931589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e845825a00","size":5,"soft_fail":false,"target_va":5383962816,"va":5378031222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38683213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e84d424e02","size":5,"soft_fail":false,"target_va":5416717696,"va":5378034478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38584856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e818c24c02","size":5,"soft_fail":false,"target_va":5416619360,"va":5378034499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5247880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378031088,"raw_hex":"e888135000","size":5,"soft_fail":false,"target_va":5383281568,"va":5378033683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":492,"record_offset":3808,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":492,"record_offset":4620,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":492,"record_offset":2732,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":492,"record_offset":3428,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":492,"record_offset":8332,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":492,"record_offset":8924,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":492,"record_offset":9892,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":492,"record_offset":12120,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":11236,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":492,"record_offset":3864,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26241,"source_line":273,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":492,"record_offset":1924,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":492,"record_offset":520,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":492,"record_offset":912,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":492,"record_offset":8952,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker::on_wall_changed
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/deco_differential_state.rs
//     evidence={"module":492,"record_offset":5016,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27839,"source_line":195,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::move_decorators_following_anchors::invalidate_roof_decorators
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/move_decorators_following_anchors.rs
//     evidence={"module":492,"record_offset":6220,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27841,"source_line":133,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::move_decorators_following_anchors::reanchor_doors_attached_to_bottom
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/move_decorators_following_anchors.rs
//     evidence={"module":492,"record_offset":10412,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27842,"source_line":151,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":492,"record_offset":9920,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26649,"source_line":845,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":492,"record_offset":10356,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":492,"record_offset":10384,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":12108,"source_line":116,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":492,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":492,"record_offset":9980,"parent_record_offset":9920,"ancestor_record_offset":272,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":492,"record_offset":3048,"parent_record_offset":2820,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":492,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":272,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":492,"record_offset":6256,"parent_record_offset":6220,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":492,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":272,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":492,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":272,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":492,"record_offset":3484,"parent_record_offset":3456,"ancestor_record_offset":272,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":492,"record_offset":2796,"parent_record_offset":2756,"ancestor_record_offset":272,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":492,"record_offset":2756,"parent_record_offset":2732,"ancestor_record_offset":272,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":492,"record_offset":2252,"parent_record_offset":2224,"ancestor_record_offset":272,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":492,"record_offset":2280,"parent_record_offset":2252,"ancestor_record_offset":272,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":492,"record_offset":2200,"parent_record_offset":2172,"ancestor_record_offset":272,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":492,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":272,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":492,"record_offset":2340,"parent_record_offset":2224,"ancestor_record_offset":272,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":492,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":272,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":492,"record_offset":10284,"parent_record_offset":10260,"ancestor_record_offset":272,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":492,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":272,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":492,"record_offset":9652,"parent_record_offset":9628,"ancestor_record_offset":272,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":492,"record_offset":3888,"parent_record_offset":3864,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":492,"record_offset":6608,"parent_record_offset":6580,"ancestor_record_offset":272,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":492,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":272,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":492,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":272,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":492,"record_offset":10764,"parent_record_offset":10412,"ancestor_record_offset":272,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":492,"record_offset":9720,"parent_record_offset":9084,"ancestor_record_offset":272,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":492,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":272,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":492,"record_offset":3516,"parent_record_offset":3484,"ancestor_record_offset":272,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":492,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":272,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":492,"record_offset":2304,"parent_record_offset":2280,"ancestor_record_offset":272,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":492,"record_offset":3004,"parent_record_offset":2976,"ancestor_record_offset":272,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":492,"record_offset":2864,"parent_record_offset":2820,"ancestor_record_offset":272,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":492,"record_offset":9452,"parent_record_offset":9424,"ancestor_record_offset":272,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":492,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":272,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":492,"record_offset":2072,"parent_record_offset":2032,"ancestor_record_offset":272,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":492,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":272,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":492,"record_offset":10260,"parent_record_offset":10236,"ancestor_record_offset":272,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":492,"record_offset":10236,"parent_record_offset":10200,"ancestor_record_offset":272,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":492,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":272,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":492,"record_offset":3748,"parent_record_offset":3428,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":492,"record_offset":3596,"parent_record_offset":3516,"ancestor_record_offset":272,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":492,"record_offset":10612,"parent_record_offset":10576,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":492,"record_offset":11076,"parent_record_offset":11040,"ancestor_record_offset":272,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":492,"record_offset":860,"parent_record_offset":832,"ancestor_record_offset":272,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":492,"record_offset":2688,"parent_record_offset":2032,"ancestor_record_offset":272,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":272,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":2404,"parent_record_offset":2172,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":492,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":492,"record_offset":8732,"parent_record_offset":8708,"ancestor_record_offset":272,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":492,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":492,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":272,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":492,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":272,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":492,"record_offset":1516,"parent_record_offset":1488,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":492,"record_offset":1616,"parent_record_offset":1588,"ancestor_record_offset":272,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":492,"record_offset":1588,"parent_record_offset":1560,"ancestor_record_offset":272,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":492,"record_offset":2920,"parent_record_offset":2820,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":492,"record_offset":3272,"parent_record_offset":2820,"ancestor_record_offset":272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":492,"record_offset":10008,"parent_record_offset":9980,"ancestor_record_offset":272,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":492,"record_offset":10036,"parent_record_offset":10008,"ancestor_record_offset":272,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":492,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":272,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":492,"record_offset":2948,"parent_record_offset":2920,"ancestor_record_offset":272,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":492,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":492,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":272,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":492,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":272,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":492,"record_offset":1644,"parent_record_offset":1616,"ancestor_record_offset":272,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":492,"record_offset":3672,"parent_record_offset":3620,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":492,"record_offset":1236,"parent_record_offset":1120,"ancestor_record_offset":272,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":492,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":272,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":492,"record_offset":948,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":492,"record_offset":1088,"parent_record_offset":1056,"ancestor_record_offset":272,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":492,"record_offset":1056,"parent_record_offset":1020,"ancestor_record_offset":272,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":492,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":272,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":492,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":272,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":492,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":272,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":492,"record_offset":1560,"parent_record_offset":1020,"ancestor_record_offset":272,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":492,"record_offset":624,"parent_record_offset":592,"ancestor_record_offset":272,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":492,"record_offset":592,"parent_record_offset":564,"ancestor_record_offset":272,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":492,"record_offset":752,"parent_record_offset":720,"ancestor_record_offset":272,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":492,"record_offset":808,"parent_record_offset":752,"ancestor_record_offset":272,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":492,"record_offset":720,"parent_record_offset":688,"ancestor_record_offset":272,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":492,"record_offset":564,"parent_record_offset":520,"ancestor_record_offset":272,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":492,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":272,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":492,"record_offset":1120,"parent_record_offset":1088,"ancestor_record_offset":272,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":492,"record_offset":1404,"parent_record_offset":1120,"ancestor_record_offset":272,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":492,"record_offset":1376,"parent_record_offset":1120,"ancestor_record_offset":272,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":492,"record_offset":1180,"parent_record_offset":1120,"ancestor_record_offset":272,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::Decorators::iter_maybe_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":492,"record_offset":6312,"parent_record_offset":6220,"ancestor_record_offset":272,"inlinee":27190,"source_line":217,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":492,"record_offset":11040,"parent_record_offset":11008,"ancestor_record_offset":272,"inlinee":26698,"source_line":223,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":492,"record_offset":11008,"parent_record_offset":10412,"ancestor_record_offset":272,"inlinee":26697,"source_line":291,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":492,"record_offset":11108,"parent_record_offset":11076,"ancestor_record_offset":272,"inlinee":26707,"source_line":226,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":492,"record_offset":9676,"parent_record_offset":9652,"ancestor_record_offset":272,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":492,"record_offset":8556,"parent_record_offset":8528,"ancestor_record_offset":272,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":492,"record_offset":8588,"parent_record_offset":8556,"ancestor_record_offset":272,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":492,"record_offset":8528,"parent_record_offset":8488,"ancestor_record_offset":272,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":492,"record_offset":9196,"parent_record_offset":9084,"ancestor_record_offset":272,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":492,"record_offset":6580,"parent_record_offset":6520,"ancestor_record_offset":272,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":492,"record_offset":9748,"parent_record_offset":9084,"ancestor_record_offset":272,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":492,"record_offset":6636,"parent_record_offset":6580,"ancestor_record_offset":272,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":492,"record_offset":9280,"parent_record_offset":9084,"ancestor_record_offset":272,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":492,"record_offset":9140,"parent_record_offset":9084,"ancestor_record_offset":272,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":492,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":272,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":492,"record_offset":8976,"parent_record_offset":8952,"ancestor_record_offset":272,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":492,"record_offset":6372,"parent_record_offset":6344,"ancestor_record_offset":272,"inlinee":20113,"source_line":799,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":492,"record_offset":9604,"parent_record_offset":9496,"ancestor_record_offset":272,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":492,"record_offset":8416,"parent_record_offset":8388,"ancestor_record_offset":272,"inlinee":13411,"source_line":3267,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":492,"record_offset":9008,"parent_record_offset":8976,"ancestor_record_offset":272,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":9544,"parent_record_offset":9520,"ancestor_record_offset":272,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":8708,"parent_record_offset":8488,"ancestor_record_offset":272,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":9840,"parent_record_offset":9084,"ancestor_record_offset":272,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":6520,"parent_record_offset":6428,"ancestor_record_offset":272,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":8488,"parent_record_offset":8452,"ancestor_record_offset":272,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":272,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":272,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":9036,"parent_record_offset":8976,"ancestor_record_offset":272,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":6400,"parent_record_offset":6372,"ancestor_record_offset":272,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":6492,"parent_record_offset":6428,"ancestor_record_offset":272,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":6464,"parent_record_offset":6428,"ancestor_record_offset":272,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":9084,"parent_record_offset":9060,"ancestor_record_offset":272,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":6428,"parent_record_offset":6400,"ancestor_record_offset":272,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":9168,"parent_record_offset":9084,"ancestor_record_offset":272,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":8452,"parent_record_offset":8416,"ancestor_record_offset":272,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":9496,"parent_record_offset":9084,"ancestor_record_offset":272,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":492,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":272,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":492,"record_offset":1960,"parent_record_offset":1924,"ancestor_record_offset":272,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":492,"record_offset":11300,"parent_record_offset":11268,"ancestor_record_offset":272,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":492,"record_offset":11756,"parent_record_offset":11552,"ancestor_record_offset":272,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":492,"record_offset":2100,"parent_record_offset":2072,"ancestor_record_offset":272,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":492,"record_offset":2172,"parent_record_offset":2032,"ancestor_record_offset":272,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":492,"record_offset":2376,"parent_record_offset":2172,"ancestor_record_offset":272,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":492,"record_offset":6344,"parent_record_offset":6312,"ancestor_record_offset":272,"inlinee":20112,"source_line":673,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$41::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":492,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":272,"inlinee":21911,"source_line":2113,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":492,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":272,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":492,"record_offset":2128,"parent_record_offset":2100,"ancestor_record_offset":272,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":492,"record_offset":2032,"parent_record_offset":1996,"ancestor_record_offset":272,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":492,"record_offset":1996,"parent_record_offset":1960,"ancestor_record_offset":272,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/move_decorators_following_anchors.rs
//     evidence={"module":492,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":272,"inlinee":27844,"source_line":83,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":492,"record_offset":10496,"parent_record_offset":10412,"ancestor_record_offset":272,"inlinee":26153,"source_line":1161,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":492,"record_offset":9952,"parent_record_offset":9920,"ancestor_record_offset":272,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":492,"record_offset":10076,"parent_record_offset":9920,"ancestor_record_offset":272,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10732,"parent_record_offset":10496,"ancestor_record_offset":272,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10704,"parent_record_offset":10496,"ancestor_record_offset":272,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10540,"parent_record_offset":10496,"ancestor_record_offset":272,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10576,"parent_record_offset":10540,"ancestor_record_offset":272,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10672,"parent_record_offset":10648,"ancestor_record_offset":272,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10648,"parent_record_offset":10496,"ancestor_record_offset":272,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":492,"record_offset":10200,"parent_record_offset":9920,"ancestor_record_offset":272,"inlinee":26650,"source_line":136,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":492,"record_offset":10792,"parent_record_offset":10412,"ancestor_record_offset":272,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":492,"record_offset":11148,"parent_record_offset":10412,"ancestor_record_offset":272,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::move_decorators_following_anchors::move_decorators_following_anchors::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CC4
// 首条位置: {"module":492,"inlinee":27844}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::move_decorators_following_anchors::invalidate_roof_decorators
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CC1
// 首条位置: {"module":492,"inlinee":27841}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":492,"record_offset":6256,"parent_record_offset":6220,"ancestor_record_offset":6220,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::Decorators::iter_maybe_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":492,"record_offset":6312,"parent_record_offset":6220,"ancestor_record_offset":6220,"inlinee":27190,"source_line":217,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":492,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":6220,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":492,"record_offset":6608,"parent_record_offset":6580,"ancestor_record_offset":6220,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":492,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":6220,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":492,"record_offset":6552,"parent_record_offset":6520,"ancestor_record_offset":6220,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":492,"record_offset":6580,"parent_record_offset":6520,"ancestor_record_offset":6220,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":492,"record_offset":6636,"parent_record_offset":6580,"ancestor_record_offset":6220,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":492,"record_offset":6372,"parent_record_offset":6344,"ancestor_record_offset":6220,"inlinee":20113,"source_line":799,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":6520,"parent_record_offset":6428,"ancestor_record_offset":6220,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":6400,"parent_record_offset":6372,"ancestor_record_offset":6220,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":6492,"parent_record_offset":6428,"ancestor_record_offset":6220,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":6464,"parent_record_offset":6428,"ancestor_record_offset":6220,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":492,"record_offset":6428,"parent_record_offset":6400,"ancestor_record_offset":6220,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":492,"record_offset":6344,"parent_record_offset":6312,"ancestor_record_offset":6220,"inlinee":20112,"source_line":673,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::move_decorators_following_anchors::reanchor_doors_attached_to_bottom
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6CC2
// 首条位置: {"module":492,"inlinee":27842}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":492,"record_offset":10764,"parent_record_offset":10412,"ancestor_record_offset":10412,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":492,"record_offset":11008,"parent_record_offset":10412,"ancestor_record_offset":10412,"inlinee":26697,"source_line":291,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":492,"record_offset":10496,"parent_record_offset":10412,"ancestor_record_offset":10412,"inlinee":26153,"source_line":1161,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":492,"record_offset":10792,"parent_record_offset":10412,"ancestor_record_offset":10412,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":492,"record_offset":11148,"parent_record_offset":10412,"ancestor_record_offset":10412,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":492,"record_offset":10612,"parent_record_offset":10576,"ancestor_record_offset":10412,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":492,"record_offset":11076,"parent_record_offset":11040,"ancestor_record_offset":10412,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":492,"record_offset":10832,"parent_record_offset":10792,"ancestor_record_offset":10412,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":492,"record_offset":10864,"parent_record_offset":10832,"ancestor_record_offset":10412,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":492,"record_offset":11040,"parent_record_offset":11008,"ancestor_record_offset":10412,"inlinee":26698,"source_line":223,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":492,"record_offset":11108,"parent_record_offset":11076,"ancestor_record_offset":10412,"inlinee":26707,"source_line":226,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":492,"record_offset":10904,"parent_record_offset":10792,"ancestor_record_offset":10412,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10732,"parent_record_offset":10496,"ancestor_record_offset":10412,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10704,"parent_record_offset":10496,"ancestor_record_offset":10412,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10540,"parent_record_offset":10496,"ancestor_record_offset":10412,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10576,"parent_record_offset":10540,"ancestor_record_offset":10412,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10672,"parent_record_offset":10648,"ancestor_record_offset":10412,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":492,"record_offset":10648,"parent_record_offset":10496,"ancestor_record_offset":10412,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/00492.jsonl.gz"}
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
// 完整结构及原始类型记录: move_decorators_following_anchors.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: move_decorators_following_anchors.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: move_decorators_following_anchors.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>

// ------------------------------------------------------------------
// 函数: system_decorator::move_decorators_following_anchors::invalidate_roof_decorators
// 信息 ID: e57c5ab7299e20e9df9471d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors; module=492, inline_record=6220; 不是独立调用边。
// [PDB 内联宿主] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors; module=492, inline_record=7744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::move_decorators_following_anchors::move_decorators_following_anchors
// 信息 ID: 1467779827e0d5efe3814961
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 10ebcedd888e7c780ebae760: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources)> > @ 0x2c2e62 -> 0x8e3df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources)> @ 0x511e9b -> 0x8e3df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::move_decorators_following_anchors::move_decorators_following_anchors::closure$1
// 信息 ID: 875eac5edf3aa4fee741941c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::move_decorators_following_anchors::move_decorators_following_anchors", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors; module=492, inline_record=12144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::move_decorators_following_anchors::reanchor_doors_attached_to_bottom
// 信息 ID: 0cd6503defa48645ad56ea07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors; module=492, inline_record=10412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
