// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/load_decorators.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d0a193c987b7ef897b326a24333160d88cdc273540dd45104d2ddc308d21ae1e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 428
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":428,"record_offset":1172,"rva":8087264,"va":5376796384}
// 编译实例 RVAs: 0x7b66e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..ReadDir$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5912257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8c1365a00","size":5,"soft_fail":false,"target_va":5382708864,"va":5376796602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..fs..FindNextFileHandle$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5970522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e85a1a5b00","size":5,"soft_fail":false,"target_va":5382768048,"va":5376797521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140002370
//     owner=unresolved_namespace; count=1
//     target_aliases=1937 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8079371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8f5b784ff","size":5,"soft_fail":false,"target_va":5368718192,"va":5376797558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e69f0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1916957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8e3bfe2ff","size":5,"soft_fail":false,"target_va":5374896624,"va":5376813576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1907276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8b4e5e2ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5376808007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1715558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e89ad2e5ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376797041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1715599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e871d2e5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376797098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=19
//     target_aliases=14 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t509527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e857c60700","size":5,"soft_fail":false,"target_va":5377307216,"va":5376797684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140897820
//     owner=unresolved_namespace; count=34
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t909658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e85ae10d00","size":5,"soft_fail":false,"target_va":5377718304,"va":5376808641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b99e0
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1060377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8192e1000","size":5,"soft_fail":false,"target_va":5377858016,"va":5376797634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a06b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2424543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8dffe2400","size":5,"soft_fail":false,"target_va":5379222272,"va":5376797724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=19
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3600525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e88df03600","size":5,"soft_fail":false,"target_va":5380398224,"va":5376797694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5908045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e84d265a00","size":5,"soft_fail":false,"target_va":5382704752,"va":5376796702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6031039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8bf065c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5376796684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412598b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11141955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e84303aa00","size":5,"soft_fail":false,"target_va":5387950256,"va":5376808296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141263de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11194669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e82dd1aa00","size":5,"soft_fail":false,"target_va":5387992544,"va":5376797870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141286bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11338547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e83303ad00","size":5,"soft_fail":false,"target_va":5388135408,"va":5376796856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35050660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8a4d41602","size":5,"soft_fail":false,"target_va":5411847168,"va":5376796503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6042865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8f1345c00","size":5,"soft_fail":false,"target_va":5382840208,"va":5376797338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t34734155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e84b001202","size":5,"soft_fail":false,"target_va":5411540963,"va":5376806803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t5886085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e885d05900","size":5,"soft_fail":false,"target_va":5382683584,"va":5376797494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::hf4c9c9092b5c8731 (.llvm.6409737604219609296)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t34860398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e86eed1302","size":5,"soft_fail":false,"target_va":5411668288,"va":5376807885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5846966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8b6375900","size":5,"soft_fail":false,"target_va":5382655024,"va":5376808053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34734997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e895031202","size":5,"soft_fail":false,"target_va":5411541776,"va":5376806774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t34735155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e833041202","size":5,"soft_fail":false,"target_va":5411542064,"va":5376806904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >
//     owner=rust_standard_library; count=19
//     evidence={"assembly":"\tcallq\t-12217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e847d0ffff","size":5,"soft_fail":false,"target_va":5376796304,"va":5376808516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34736000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e880071202","size":5,"soft_fail":false,"target_va":5411542880,"va":5376806875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34735649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e821061202","size":5,"soft_fail":false,"target_va":5411543360,"va":5376807706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1531758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e86e5f1700","size":5,"soft_fail":false,"target_va":5378331728,"va":5376799965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::sphere::Sphere::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3752781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e84d433900","size":5,"soft_fail":false,"target_va":5380550496,"va":5376797710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::add_color
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2427734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8560b2500","size":5,"soft_fail":false,"target_va":5379224896,"va":5376797157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::max_rank
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t508882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8d2c30700","size":5,"soft_fail":false,"target_va":5377306928,"va":5376798041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorCollision::empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3597731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8a3e53600","size":5,"soft_fail":false,"target_va":5380398240,"va":5376800504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert
//     owner=package_namespace_candidate; count=15
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3601191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e827f33600","size":5,"soft_fail":false,"target_va":5380399776,"va":5376798580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert_collision_and_interaction_as_same
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3599904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e820ee3600","size":5,"soft_fail":false,"target_va":5380398832,"va":5376798923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3555779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8c3413600","size":5,"soft_fail":false,"target_va":5380353184,"va":5376797400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5886845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e87dd35900","size":5,"soft_fail":false,"target_va":5382694656,"va":5376807806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5912871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e827395a00","size":5,"soft_fail":false,"target_va":5382709552,"va":5376796676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirEntry::path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5912166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e866365a00","size":5,"soft_fail":false,"target_va":5382709008,"va":5376796837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::readdir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5974092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e84c285b00","size":5,"soft_fail":false,"target_va":5382770560,"va":5376796463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_collision_as_bbx
//     owner=package_namespace_candidate; count=35
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t18013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e85d460000","size":5,"soft_fail":false,"target_va":5376816304,"va":5376798286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_decorators::closure$0
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t15932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e83c3e0000","size":5,"soft_fail":false,"target_va":5376813776,"va":5376797839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_decorators::closure$1
//     owner=package_namespace_candidate; count=28
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e8c4410000","size":5,"soft_fail":false,"target_va":5376814848,"va":5376798007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_decorators::closure$3
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376796384,"raw_hex":"e87a430000","size":5,"soft_fail":false,"target_va":5376815456,"va":5376798177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":428,"record_offset":2964,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":428,"record_offset":2136,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$88::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":428,"record_offset":2608,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":22271,"source_line":2533,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":428,"record_offset":16088,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":428,"record_offset":4420,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":428,"record_offset":8304,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":428,"record_offset":3048,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":428,"record_offset":6076,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":428,"record_offset":1660,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":2768,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":428,"record_offset":1452,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":428,"record_offset":1696,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":25083,"source_line":2425,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":1980,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":428,"record_offset":4328,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":7661,"source_line":100,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":428,"record_offset":4356,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":8819,"source_line":221,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":428,"record_offset":1628,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":428,"record_offset":1572,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":428,"record_offset":1384,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":12459,"source_line":2936,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::load_decorators::load_decorators::closure$2
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":428,"record_offset":7232,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":25087,"source_line":85,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::load_decorators::load_decorators::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":428,"record_offset":4476,"parent_record_offset":1172,"ancestor_record_offset":1172,"inlinee":25086,"source_line":104,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":1172,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1172,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":1172,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":2428,"parent_record_offset":2400,"ancestor_record_offset":1172,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":1172,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":428,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":1172,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":1172,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":3200,"parent_record_offset":3172,"ancestor_record_offset":1172,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":2864,"parent_record_offset":2840,"ancestor_record_offset":1172,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":2400,"parent_record_offset":2368,"ancestor_record_offset":1172,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":2368,"parent_record_offset":2340,"ancestor_record_offset":1172,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":1172,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":428,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":1172,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":428,"record_offset":2248,"parent_record_offset":2220,"ancestor_record_offset":1172,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":428,"record_offset":2192,"parent_record_offset":2164,"ancestor_record_offset":1172,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":428,"record_offset":2220,"parent_record_offset":2192,"ancestor_record_offset":1172,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":428,"record_offset":2164,"parent_record_offset":2136,"ancestor_record_offset":1172,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":428,"record_offset":8360,"parent_record_offset":8332,"ancestor_record_offset":1172,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":428,"record_offset":8332,"parent_record_offset":8304,"ancestor_record_offset":1172,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":428,"record_offset":3696,"parent_record_offset":3660,"ancestor_record_offset":1172,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":428,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":1172,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":2312,"parent_record_offset":2276,"ancestor_record_offset":1172,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":428,"record_offset":16164,"parent_record_offset":16116,"ancestor_record_offset":1172,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":428,"record_offset":16116,"parent_record_offset":16088,"ancestor_record_offset":1172,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":428,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":1172,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":428,"record_offset":6172,"parent_record_offset":6104,"ancestor_record_offset":1172,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":428,"record_offset":1808,"parent_record_offset":1780,"ancestor_record_offset":1172,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":428,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":1172,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":428,"record_offset":6104,"parent_record_offset":6076,"ancestor_record_offset":1172,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":428,"record_offset":5076,"parent_record_offset":5044,"ancestor_record_offset":1172,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":428,"record_offset":2092,"parent_record_offset":2064,"ancestor_record_offset":1172,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":428,"record_offset":6140,"parent_record_offset":6104,"ancestor_record_offset":1172,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":428,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":1172,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":428,"record_offset":7260,"parent_record_offset":7232,"ancestor_record_offset":1172,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":428,"record_offset":4388,"parent_record_offset":4356,"ancestor_record_offset":1172,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":428,"record_offset":2528,"parent_record_offset":2276,"ancestor_record_offset":1172,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":1496,"parent_record_offset":1452,"ancestor_record_offset":1172,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":428,"record_offset":3728,"parent_record_offset":3696,"ancestor_record_offset":1172,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":428,"record_offset":1412,"parent_record_offset":1384,"ancestor_record_offset":1172,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":428,"record_offset":4528,"parent_record_offset":4476,"ancestor_record_offset":1172,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":428,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1172,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":428,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":1172,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":428,"record_offset":1780,"parent_record_offset":1752,"ancestor_record_offset":1172,"inlinee":5362,"source_line":2612,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":428,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1172,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":428,"record_offset":1752,"parent_record_offset":1724,"ancestor_record_offset":1172,"inlinee":8777,"source_line":1024,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":428,"record_offset":1724,"parent_record_offset":1696,"ancestor_record_offset":1172,"inlinee":25084,"source_line":1033,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":1172,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":2040,"parent_record_offset":2008,"ancestor_record_offset":1172,"inlinee":9164,"source_line":162,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":2008,"parent_record_offset":1980,"ancestor_record_offset":1172,"inlinee":9151,"source_line":236,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":428,"record_offset":3756,"parent_record_offset":3696,"ancestor_record_offset":1172,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":428,"record_offset":3788,"parent_record_offset":3660,"ancestor_record_offset":1172,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":428,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":1172,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":428,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":1172,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":428,"record_offset":17044,"rva":8104656,"va":5376813776}
// 编译实例 RVAs: 0x7baad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140601be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1806740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e86c6ee4ff","size":5,"soft_fail":false,"target_va":5375007712,"va":5376814447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1732723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e88d8fe5ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376814206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1732881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e8ef8ee5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376814380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412612d0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11167649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e8a167aa00","size":5,"soft_fail":false,"target_va":5387981520,"va":5376813866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141261a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11169385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e8696eaa00","size":5,"soft_fail":false,"target_va":5387983376,"va":5376813986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35032931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e8638f1602","size":5,"soft_fail":false,"target_va":5411847168,"va":5376814232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34726681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e819e31102","size":5,"soft_fail":false,"target_va":5411540992,"va":5376814306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::arrayed_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2410960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e8d0c92400","size":5,"soft_fail":false,"target_va":5379224784,"va":5376813819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::as_triangles
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2414213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e885d62400","size":5,"soft_fail":false,"target_va":5379228288,"va":5376814070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2412127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e85fce2400","size":5,"soft_fail":false,"target_va":5379226224,"va":5376814092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::normals_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2413250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e8c2d22400","size":5,"soft_fail":false,"target_va":5379227136,"va":5376813881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2412945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e891d12400","size":5,"soft_fail":false,"target_va":5379226784,"va":5376813834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::uvs_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2413515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e8cbd32400","size":5,"soft_fail":false,"target_va":5379227472,"va":5376813952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_indexed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1131099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376813776,"raw_hex":"e85b421100","size":5,"soft_fail":false,"target_va":5377945152,"va":5376814048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":428,"record_offset":17492,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":428,"record_offset":17204,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":428,"record_offset":17260,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":17748,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":17616,"parent_record_offset":17592,"ancestor_record_offset":17044,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":17640,"parent_record_offset":17616,"ancestor_record_offset":17044,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":17044,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":17560,"parent_record_offset":17524,"ancestor_record_offset":17044,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":17592,"parent_record_offset":17560,"ancestor_record_offset":17044,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":17912,"parent_record_offset":17820,"ancestor_record_offset":17044,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":428,"record_offset":17524,"parent_record_offset":17492,"ancestor_record_offset":17044,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":17848,"parent_record_offset":17820,"ancestor_record_offset":17044,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":17820,"parent_record_offset":17796,"ancestor_record_offset":17044,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":17796,"parent_record_offset":17772,"ancestor_record_offset":17044,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":17044,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":428,"record_offset":17308,"parent_record_offset":17288,"ancestor_record_offset":17044,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":428,"record_offset":17880,"parent_record_offset":17848,"ancestor_record_offset":17044,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":17712,"parent_record_offset":17524,"ancestor_record_offset":17044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":17668,"parent_record_offset":17640,"ancestor_record_offset":17044,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::load_decorators::load_decorators::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":428,"record_offset":17288,"parent_record_offset":17260,"ancestor_record_offset":17044,"inlinee":25089,"source_line":65,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::load_decorators::load_decorators::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":428,"record_offset":17404,"parent_record_offset":17372,"ancestor_record_offset":17044,"inlinee":25090,"source_line":66,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6201
// 首条位置: {"module":428,"inlinee":25089}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":428,"record_offset":17308,"parent_record_offset":17288,"ancestor_record_offset":17288,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":17332,"parent_record_offset":17308,"ancestor_record_offset":17288,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6202
// 首条位置: {"module":428,"inlinee":25090}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":428,"record_offset":17428,"parent_record_offset":17404,"ancestor_record_offset":17404,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":17452,"parent_record_offset":17428,"ancestor_record_offset":17404,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":428,"record_offset":18716,"rva":8105728,"va":5376814848}
// 编译实例 RVAs: 0x7baf00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1733891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376814848,"raw_hex":"e8fd8ae5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376815390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412598b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11135147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376814848,"raw_hex":"e8abe8a900","size":5,"soft_fail":false,"target_va":5387950256,"va":5376815104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412612d0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11166420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376814848,"raw_hex":"e8d462aa00","size":5,"soft_fail":false,"target_va":5387981520,"va":5376815095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141287d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11324846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376814848,"raw_hex":"e8aecdac00","size":5,"soft_fail":false,"target_va":5388139824,"va":5376814973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6025242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376814848,"raw_hex":"e81af05b00","size":5,"soft_fail":false,"target_va":5382840208,"va":5376814961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4079943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376814848,"raw_hex":"e847413e00","size":5,"soft_fail":false,"target_va":5380895104,"va":5376815156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5839588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376814848,"raw_hex":"e8e41a5900","size":5,"soft_fail":false,"target_va":5382655024,"va":5376815431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34727017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376814848,"raw_hex":"e869e41102","size":5,"soft_fail":false,"target_va":5411542296,"va":5376815274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34727619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376814848,"raw_hex":"e8c3e61102","size":5,"soft_fail":false,"target_va":5411542880,"va":5376815256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2411706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376814848,"raw_hex":"e8bacc2400","size":5,"soft_fail":false,"target_va":5379226784,"va":5376815073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":428,"record_offset":18884,"parent_record_offset":18716,"ancestor_record_offset":18716,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":19200,"parent_record_offset":18716,"ancestor_record_offset":18716,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":19104,"parent_record_offset":18716,"ancestor_record_offset":18716,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":428,"record_offset":19040,"parent_record_offset":18716,"ancestor_record_offset":18716,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":19388,"parent_record_offset":18716,"ancestor_record_offset":18716,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":428,"record_offset":18964,"parent_record_offset":18716,"ancestor_record_offset":18716,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":19576,"parent_record_offset":19552,"ancestor_record_offset":18716,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":19552,"parent_record_offset":19460,"ancestor_record_offset":18716,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":428,"record_offset":18932,"parent_record_offset":18908,"ancestor_record_offset":18716,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":19268,"parent_record_offset":19236,"ancestor_record_offset":18716,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":19488,"parent_record_offset":19460,"ancestor_record_offset":18716,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":19460,"parent_record_offset":19436,"ancestor_record_offset":18716,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":18716,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":19292,"parent_record_offset":19268,"ancestor_record_offset":18716,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":19436,"parent_record_offset":19412,"ancestor_record_offset":18716,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":19236,"parent_record_offset":19200,"ancestor_record_offset":18716,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":19068,"parent_record_offset":19040,"ancestor_record_offset":18716,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":428,"record_offset":19520,"parent_record_offset":19488,"ancestor_record_offset":18716,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":428,"record_offset":18908,"parent_record_offset":18884,"ancestor_record_offset":18716,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":19008,"parent_record_offset":18964,"ancestor_record_offset":18716,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":19356,"parent_record_offset":19200,"ancestor_record_offset":18716,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":428,"record_offset":19132,"parent_record_offset":19104,"ancestor_record_offset":18716,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":428,"record_offset":19160,"parent_record_offset":19132,"ancestor_record_offset":18716,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x61FF
// 首条位置: {"module":428,"inlinee":25087}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":428,"record_offset":7260,"parent_record_offset":7232,"ancestor_record_offset":7232,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":428,"record_offset":19628,"rva":8106336,"va":5376815456}
// 编译实例 RVAs: 0x7bb160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140601be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1808500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e88c67e4ff","size":5,"soft_fail":false,"target_va":5375007712,"va":5376816207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1734364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e82489e5ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376815847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1734193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e8cf89e5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376815692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412612d0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11165972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e81461aa00","size":5,"soft_fail":false,"target_va":5387981520,"va":5376815543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35031290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e8fa881602","size":5,"soft_fail":false,"target_va":5411847168,"va":5376815873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34725002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e88adc1102","size":5,"soft_fail":false,"target_va":5411540992,"va":5376815985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34725835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e8cbdf1102","size":5,"soft_fail":false,"target_va":5411541808,"va":5376815968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34725827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e8c3df1102","size":5,"soft_fail":false,"target_va":5411541776,"va":5376815944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1516225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e8c1221700","size":5,"soft_fail":false,"target_va":5378331728,"va":5376815498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::as_triangles
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2412569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e819d02400","size":5,"soft_fail":false,"target_va":5379228288,"va":5376815714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2410483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e8f3c72400","size":5,"soft_fail":false,"target_va":5379226224,"va":5376815736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2411261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e8fdca2400","size":5,"soft_fail":false,"target_va":5379226784,"va":5376815518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_convex
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1129014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376815456,"raw_hex":"e8363a1100","size":5,"soft_fail":false,"target_va":5377944592,"va":5376815573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":428,"record_offset":20152,"parent_record_offset":19628,"ancestor_record_offset":19628,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":428,"record_offset":19828,"parent_record_offset":19628,"ancestor_record_offset":19628,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":428,"record_offset":19884,"parent_record_offset":19628,"ancestor_record_offset":19628,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":428,"record_offset":19792,"parent_record_offset":19628,"ancestor_record_offset":19628,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":19920,"parent_record_offset":19628,"ancestor_record_offset":19628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":20276,"parent_record_offset":20252,"ancestor_record_offset":19628,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":20300,"parent_record_offset":20276,"ancestor_record_offset":19628,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":19628,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":20220,"parent_record_offset":20184,"ancestor_record_offset":19628,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":20252,"parent_record_offset":20220,"ancestor_record_offset":19628,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":20080,"parent_record_offset":19992,"ancestor_record_offset":19628,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":428,"record_offset":20184,"parent_record_offset":20152,"ancestor_record_offset":19628,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":20020,"parent_record_offset":19992,"ancestor_record_offset":19628,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":19992,"parent_record_offset":19968,"ancestor_record_offset":19628,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":19968,"parent_record_offset":19944,"ancestor_record_offset":19628,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":19852,"parent_record_offset":19828,"ancestor_record_offset":19628,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":428,"record_offset":20048,"parent_record_offset":20020,"ancestor_record_offset":19628,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":19944,"parent_record_offset":19920,"ancestor_record_offset":19628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":20328,"parent_record_offset":20300,"ancestor_record_offset":19628,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x61FE
// 首条位置: {"module":428,"inlinee":25086}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":4804,"parent_record_offset":4476,"ancestor_record_offset":4476,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":428,"record_offset":4528,"parent_record_offset":4476,"ancestor_record_offset":4476,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":4476,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":4724,"parent_record_offset":4668,"ancestor_record_offset":4476,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":4696,"parent_record_offset":4668,"ancestor_record_offset":4476,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":4668,"parent_record_offset":4644,"ancestor_record_offset":4476,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":4476,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":428,"record_offset":5076,"parent_record_offset":5044,"ancestor_record_offset":4476,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":4572,"parent_record_offset":4528,"ancestor_record_offset":4476,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":428,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":4476,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_collision_as_bbx
// 记录形式: compiled_procedure; instances=1
// 源码行: 645
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":428,"record_offset":20880,"rva":8107184,"va":5376816304}
// 编译实例 RVAs: 0x7bb4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140601be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1809536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e88063e4ff","size":5,"soft_fail":false,"target_va":5375007712,"va":5376817243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1735391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e82185e5ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376816874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1735883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e83583e5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376817382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3804444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e81c0d3a00","size":5,"soft_fail":false,"target_va":5380621856,"va":5376817407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5118010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e83a184e00","size":5,"soft_fail":false,"target_va":5381934560,"va":5376816545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35030259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e8f3841602","size":5,"soft_fail":false,"target_va":5411847168,"va":5376816904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34723827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e8f3d71102","size":5,"soft_fail":false,"target_va":5411540992,"va":5376817160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34724626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e812db1102","size":5,"soft_fail":false,"target_va":5411541776,"va":5376817145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34851359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e81fca1302","size":5,"soft_fail":false,"target_va":5411668497,"va":5376817133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1515332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e8441f1700","size":5,"soft_fail":false,"target_va":5378331728,"va":5376816391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2409804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e84cc52400","size":5,"soft_fail":false,"target_va":5379226224,"va":5376816415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_cuboid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1132915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e873491100","size":5,"soft_fail":false,"target_va":5377949680,"va":5376816760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3271385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e8d9ea3100","size":5,"soft_fail":false,"target_va":5380088032,"va":5376816642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5069319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376816304,"raw_hex":"e8075a4d00","size":5,"soft_fail":false,"target_va":5381885904,"va":5376816580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":428,"record_offset":21368,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":428,"record_offset":21052,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":21664,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":428,"record_offset":21228,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$102::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":428,"record_offset":21088,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":8499,"source_line":17,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":428,"record_offset":21200,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":10295,"source_line":74,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":428,"record_offset":21116,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":428,"record_offset":21172,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":10293,"source_line":89,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":428,"record_offset":21144,"parent_record_offset":20880,"ancestor_record_offset":20880,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":21496,"parent_record_offset":21472,"ancestor_record_offset":20880,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":21520,"parent_record_offset":21496,"ancestor_record_offset":20880,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":21848,"parent_record_offset":21824,"ancestor_record_offset":20880,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":21440,"parent_record_offset":21404,"ancestor_record_offset":20880,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":21472,"parent_record_offset":21440,"ancestor_record_offset":20880,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":21824,"parent_record_offset":21736,"ancestor_record_offset":20880,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":428,"record_offset":21404,"parent_record_offset":21368,"ancestor_record_offset":20880,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":21764,"parent_record_offset":21736,"ancestor_record_offset":20880,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":21736,"parent_record_offset":21712,"ancestor_record_offset":20880,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":428,"record_offset":21712,"parent_record_offset":21688,"ancestor_record_offset":20880,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":428,"record_offset":21920,"parent_record_offset":21896,"ancestor_record_offset":20880,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":428,"record_offset":21792,"parent_record_offset":21764,"ancestor_record_offset":20880,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":21592,"parent_record_offset":21404,"ancestor_record_offset":20880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":21548,"parent_record_offset":21520,"ancestor_record_offset":20880,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":428,"record_offset":21956,"parent_record_offset":21920,"ancestor_record_offset":20880,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":428,"record_offset":21980,"parent_record_offset":21956,"ancestor_record_offset":20880,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":428,"record_offset":22012,"parent_record_offset":21920,"ancestor_record_offset":20880,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00428.jsonl.gz"}
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
// 完整结构及原始类型记录: load_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_collision_as_bbx
// 信息 ID: fd672b85b46ed7af6a124b80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators
// 信息 ID: c6b34feec31cf69f94965628
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 112204e236791f18e171d314: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> > @ 0x3fdbe -> 0x7b66e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> > > @ 0x47abc9 -> 0x7b66e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$0
// 信息 ID: 9b490d4c05ac0b56ace94294
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b6c8f -> 0x7baad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b8ed1 -> 0x7baad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$0::closure$0
// 信息 ID: 856deae9de94dd507bde97d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators::closure$0; module=428, inline_record=17288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$0::closure$1
// 信息 ID: 00406c7a8512f7ece1d4b752
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators::closure$0; module=428, inline_record=17404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$1
// 信息 ID: 61dcbaba299ac433e5a588be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b6d37 -> 0x7baf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b8ef7 -> 0x7baf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b7443 -> 0x7baf00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b7421 -> 0x7baf00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$2
// 信息 ID: 3a7fd6e123ca96aa59f1ca1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=428, inline_record=7232; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=428, inline_record=7292; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=428, inline_record=7536; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=428, inline_record=8028; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=428, inline_record=10100; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=428, inline_record=10696; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=428, inline_record=11292; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=428, inline_record=15536; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$3
// 信息 ID: ce10213bf79f948676eb5ded
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$4
// 信息 ID: c365cf21e3d99069a416b8b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=428, inline_record=4476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
