// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/load_decorators.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 35613c6a3e62bb9e63a5b336533c6f7dae88f5facb206ef8bf291c988ad9a819
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 445
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 首条位置: {"module":445,"record_offset":560,"rva":8856912,"va":5377566032}
// 编译实例 RVAs: 0x872550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-1887963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e82531e3ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377566646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=17
//     evidence={"assembly":"\tcallq\t-1887667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e84d32e3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377566366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=8
//     target_aliases=1699 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8844444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8640b79ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377566631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140104100
//     owner=unresolved_namespace; count=1
//     target_aliases=583 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7792826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8461789ff","size":5,"soft_fail":false,"target_va":5369774336,"va":5377567157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=19
//     target_aliases=20 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t559641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8198a0800","size":5,"soft_fail":false,"target_va":5378126944,"va":5377567298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14094d680
//     owner=unresolved_namespace; count=1
//     target_aliases=24 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t896106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e86aac0d00","size":5,"soft_fail":false,"target_va":5378463360,"va":5377567249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140971300
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1028837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8e5b20f00","size":5,"soft_fail":false,"target_va":5378609920,"va":5377581078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a6640
//     owner=unresolved_namespace; count=31
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1251684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e864191300","size":5,"soft_fail":false,"target_va":5378827840,"va":5377576151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c2eef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3916929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e881c43b00","size":5,"soft_fail":false,"target_va":5381484272,"va":5377567338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c915a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4311379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e853c94100","size":5,"soft_fail":false,"target_va":5381887392,"va":5377576008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414aa360
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12810361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e87978c300","size":5,"soft_fail":false,"target_va":5390377824,"va":5377567458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414deb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13026721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8a1c5c600","size":5,"soft_fail":false,"target_va":5390592832,"va":5377566106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414df050
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13027627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e82bc9c600","size":5,"soft_fail":false,"target_va":5390594128,"va":5377566496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7227971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8434a6e00","size":5,"soft_fail":false,"target_va":5384794864,"va":5377566888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t39045468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e85cc95302","size":5,"soft_fail":false,"target_va":5416620794,"va":5377575321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h8c4d9339f82f42d5 (.llvm.10701964462552764965)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39174956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e82cc35502","size":5,"soft_fail":false,"target_va":5416749728,"va":5377574767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7043331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e803796b00","size":5,"soft_fail":false,"target_va":5384624080,"va":5377580744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t39046767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e86fce5302","size":5,"soft_fail":false,"target_va":5416621600,"va":5377574828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t39047351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8b7d05302","size":5,"soft_fail":false,"target_va":5416622208,"va":5377574852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >
//     owner=rust_standard_library; count=15
//     evidence={"assembly":"\tcallq\t10730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8ea290000","size":5,"soft_fail":false,"target_va":5377586592,"va":5377575857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39046817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8a1ce5302","size":5,"soft_fail":false,"target_va":5416621632,"va":5377574810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39046078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8becb5302","size":5,"soft_fail":false,"target_va":5416621376,"va":5377575293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1696395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e88be21900","size":5,"soft_fail":false,"target_va":5379265744,"va":5377569344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::sphere::Sphere::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1647535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8af231900","size":5,"soft_fail":false,"target_va":5379214864,"va":5377567324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::add_color
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3899695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e82f813b00","size":5,"soft_fail":false,"target_va":5381466448,"va":5377566748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::max_rank
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t559481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e879890800","size":5,"soft_fail":false,"target_va":5378127056,"va":5377567570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorCollision::empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4081050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e89a453e00","size":5,"soft_fail":false,"target_va":5381650896,"va":5377569841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=19
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4083549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e85d4f3e00","size":5,"soft_fail":false,"target_va":5381650864,"va":5377567310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert
//     owner=package_namespace_candidate; count=15
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4084997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e805553e00","size":5,"soft_fail":false,"target_va":5381653072,"va":5377568070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert_collision_and_interaction_as_same
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4083718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e806503e00","size":5,"soft_fail":false,"target_va":5381652144,"va":5377568421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library_replacing
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2083512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8b8ca1f00","size":5,"soft_fail":false,"target_va":5379650480,"va":5377566963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_collision_as_bbx
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8e8430000","size":5,"soft_fail":false,"target_va":5377585216,"va":5377567827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_decorators::closure$0
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8323a0000","size":5,"soft_fail":false,"target_va":5377582352,"va":5377567449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_decorators::closure$1
//     owner=package_namespace_candidate; count=26
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8b5410000","size":5,"soft_fail":false,"target_va":5377584352,"va":5377567526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::load_decorators::load_decorators::closure$3
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377566032,"raw_hex":"e8f5340000","size":5,"soft_fail":false,"target_va":5377581264,"va":5377567702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":445,"record_offset":3088,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":445,"record_offset":2208,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$91::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":445,"record_offset":2676,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":21551,"source_line":2666,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":864,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":445,"record_offset":920,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":445,"record_offset":764,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":445,"record_offset":3200,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":445,"record_offset":7408,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":445,"record_offset":3172,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":445,"record_offset":5184,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":11646,"source_line":1259,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":1304,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":445,"record_offset":1980,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":445,"record_offset":1080,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":7233,"source_line":2471,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2052,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":445,"record_offset":4652,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":10540,"source_line":100,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":445,"record_offset":4680,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":10549,"source_line":221,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::load_decorators::load_decorators::closure$2
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":445,"record_offset":6340,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":27030,"source_line":109,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::load_decorators::load_decorators::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":445,"record_offset":4800,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":27029,"source_line":128,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":560,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":560,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":560,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":560,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":560,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":560,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":2472,"parent_record_offset":2444,"ancestor_record_offset":560,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":1528,"parent_record_offset":1472,"ancestor_record_offset":560,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":445,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":560,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":2384,"parent_record_offset":2356,"ancestor_record_offset":560,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":1500,"parent_record_offset":1472,"ancestor_record_offset":560,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":560,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":2444,"parent_record_offset":2412,"ancestor_record_offset":560,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":2412,"parent_record_offset":2384,"ancestor_record_offset":560,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":560,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":445,"record_offset":2264,"parent_record_offset":2236,"ancestor_record_offset":560,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":445,"record_offset":2292,"parent_record_offset":2264,"ancestor_record_offset":560,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":445,"record_offset":2320,"parent_record_offset":2292,"ancestor_record_offset":560,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":445,"record_offset":2236,"parent_record_offset":2208,"ancestor_record_offset":560,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":445,"record_offset":7464,"parent_record_offset":7436,"ancestor_record_offset":560,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":445,"record_offset":7436,"parent_record_offset":7408,"ancestor_record_offset":560,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":445,"record_offset":14820,"parent_record_offset":14784,"ancestor_record_offset":560,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":445,"record_offset":14784,"parent_record_offset":14756,"ancestor_record_offset":560,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":2356,"parent_record_offset":2320,"ancestor_record_offset":560,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":445,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":560,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":445,"record_offset":4352,"parent_record_offset":4328,"ancestor_record_offset":560,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":445,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":560,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":445,"record_offset":796,"parent_record_offset":764,"ancestor_record_offset":560,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":445,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":560,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":445,"record_offset":5280,"parent_record_offset":5212,"ancestor_record_offset":560,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":445,"record_offset":1192,"parent_record_offset":1164,"ancestor_record_offset":560,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":445,"record_offset":5212,"parent_record_offset":5184,"ancestor_record_offset":560,"inlinee":11648,"source_line":1164,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":4480,"parent_record_offset":4452,"ancestor_record_offset":560,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":445,"record_offset":2164,"parent_record_offset":2136,"ancestor_record_offset":560,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":445,"record_offset":5248,"parent_record_offset":5212,"ancestor_record_offset":560,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":445,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":560,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":445,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":560,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":445,"record_offset":4712,"parent_record_offset":4680,"ancestor_record_offset":560,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":445,"record_offset":2600,"parent_record_offset":2320,"ancestor_record_offset":560,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":560,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=62
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":445,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":560,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":984,"parent_record_offset":920,"ancestor_record_offset":560,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":14852,"parent_record_offset":14820,"ancestor_record_offset":560,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":1012,"parent_record_offset":920,"ancestor_record_offset":560,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":956,"parent_record_offset":920,"ancestor_record_offset":560,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":1040,"parent_record_offset":1012,"ancestor_record_offset":560,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":445,"record_offset":4848,"parent_record_offset":4800,"ancestor_record_offset":560,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":445,"record_offset":1220,"parent_record_offset":1192,"ancestor_record_offset":560,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":445,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":560,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":445,"record_offset":1164,"parent_record_offset":1136,"ancestor_record_offset":560,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":445,"record_offset":2136,"parent_record_offset":2112,"ancestor_record_offset":560,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":445,"record_offset":1136,"parent_record_offset":1108,"ancestor_record_offset":560,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":445,"record_offset":1108,"parent_record_offset":1080,"ancestor_record_offset":560,"inlinee":7235,"source_line":1037,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":560,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2112,"parent_record_offset":2080,"ancestor_record_offset":560,"inlinee":15555,"source_line":161,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2080,"parent_record_offset":2052,"ancestor_record_offset":560,"inlinee":15548,"source_line":237,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":14880,"parent_record_offset":14820,"ancestor_record_offset":560,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":14912,"parent_record_offset":14784,"ancestor_record_offset":560,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::reload_decorators_after_mod_swap
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":445,"record_offset":30704,"rva":8877392,"va":5377586512}
// 编译实例 RVAs: 0x877550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] system_decorator::load_decorators::load_decorators
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-20544","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377586512,"raw_hex":"e9c0afffff","size":5,"soft_fail":false,"target_va":5377566032,"va":5377586571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":445,"record_offset":31064,"parent_record_offset":30704,"ancestor_record_offset":30704,"inlinee":27039,"source_line":112,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":445,"record_offset":30860,"parent_record_offset":30704,"ancestor_record_offset":30704,"inlinee":27034,"source_line":102,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":30968,"parent_record_offset":30944,"ancestor_record_offset":30704,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":445,"record_offset":31088,"parent_record_offset":31064,"ancestor_record_offset":30704,"inlinee":27040,"source_line":144,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":445,"record_offset":30916,"parent_record_offset":30880,"ancestor_record_offset":30704,"inlinee":27037,"source_line":120,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":445,"record_offset":30944,"parent_record_offset":30916,"ancestor_record_offset":30704,"inlinee":27038,"source_line":265,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":445,"record_offset":30880,"parent_record_offset":30860,"ancestor_record_offset":30704,"inlinee":27036,"source_line":1057,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":445,"record_offset":31024,"parent_record_offset":30916,"ancestor_record_offset":30704,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":30996,"parent_record_offset":30916,"ancestor_record_offset":30704,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":445,"record_offset":21820,"rva":8873232,"va":5377582352}
// 编译实例 RVAs: 0x876510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-1903838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e822f3e2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377582521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-1905097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e837eee2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377583796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8860315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e865cd78ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377582502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ba370
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8112028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e8643884ff","size":5,"soft_fail":false,"target_va":5369471856,"va":5377583879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39473422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e80e515a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5377583629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39037045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e875a85302","size":5,"soft_fail":false,"target_va":5416620769,"va":5377583719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39038306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e862ad5302","size":5,"soft_fail":false,"target_va":5416620794,"va":5377582483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::arrayed_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3881352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e888393b00","size":5,"soft_fail":false,"target_va":5381463760,"va":5377582403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::as_triangles
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3879728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e830333b00","size":5,"soft_fail":false,"target_va":5381463200,"va":5377583467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3880089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e899343b00","size":5,"soft_fail":false,"target_va":5381463584,"va":5377583490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::normals_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3882390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e8963d3b00","size":5,"soft_fail":false,"target_va":5381465136,"va":5377582741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3882985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e8e93f3b00","size":5,"soft_fail":false,"target_va":5381465408,"va":5377582418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::uvs_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3881814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e8563b3b00","size":5,"soft_fail":false,"target_va":5381464880,"va":5377583061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_indexed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1584455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377582352,"raw_hex":"e8472d1800","size":5,"soft_fail":false,"target_va":5379167904,"va":5377583444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":445,"record_offset":25728,"parent_record_offset":21820,"ancestor_record_offset":21820,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":21980,"parent_record_offset":21820,"ancestor_record_offset":21820,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":445,"record_offset":23132,"parent_record_offset":21820,"ancestor_record_offset":21820,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":25988,"parent_record_offset":21820,"ancestor_record_offset":21820,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":22564,"parent_record_offset":22540,"ancestor_record_offset":21820,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":22588,"parent_record_offset":22564,"ancestor_record_offset":21820,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":26176,"parent_record_offset":26152,"ancestor_record_offset":21820,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":26200,"parent_record_offset":26176,"ancestor_record_offset":21820,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":22612,"parent_record_offset":22588,"ancestor_record_offset":21820,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":26224,"parent_record_offset":26200,"ancestor_record_offset":21820,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":22540,"parent_record_offset":22384,"ancestor_record_offset":21820,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":26152,"parent_record_offset":26060,"ancestor_record_offset":21820,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":445,"record_offset":25800,"parent_record_offset":25760,"ancestor_record_offset":21820,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":445,"record_offset":25760,"parent_record_offset":25728,"ancestor_record_offset":21820,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":22320,"parent_record_offset":22292,"ancestor_record_offset":21820,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":26088,"parent_record_offset":26060,"ancestor_record_offset":21820,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":26060,"parent_record_offset":26036,"ancestor_record_offset":21820,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":22384,"parent_record_offset":22348,"ancestor_record_offset":21820,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":22348,"parent_record_offset":22320,"ancestor_record_offset":21820,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":26036,"parent_record_offset":26012,"ancestor_record_offset":21820,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":22428,"parent_record_offset":22384,"ancestor_record_offset":21820,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":22700,"parent_record_offset":22672,"ancestor_record_offset":21820,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":22264,"parent_record_offset":22052,"ancestor_record_offset":21820,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":22292,"parent_record_offset":22264,"ancestor_record_offset":21820,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":22004,"parent_record_offset":21980,"ancestor_record_offset":21820,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":22944,"parent_record_offset":22912,"ancestor_record_offset":21820,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":445,"record_offset":23020,"parent_record_offset":22944,"ancestor_record_offset":21820,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":445,"record_offset":22672,"parent_record_offset":22052,"ancestor_record_offset":21820,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":445,"record_offset":22028,"parent_record_offset":22004,"ancestor_record_offset":21820,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":445,"record_offset":22052,"parent_record_offset":22028,"ancestor_record_offset":21820,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":445,"record_offset":22452,"parent_record_offset":22428,"ancestor_record_offset":21820,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":445,"record_offset":22500,"parent_record_offset":22476,"ancestor_record_offset":21820,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":22852,"parent_record_offset":22812,"ancestor_record_offset":21820,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":22784,"parent_record_offset":22756,"ancestor_record_offset":21820,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":22120,"parent_record_offset":22096,"ancestor_record_offset":21820,"inlinee":10972,"source_line":64,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":22756,"parent_record_offset":22728,"ancestor_record_offset":21820,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":22096,"parent_record_offset":22052,"ancestor_record_offset":21820,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":22880,"parent_record_offset":22852,"ancestor_record_offset":21820,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":23196,"parent_record_offset":23168,"ancestor_record_offset":21820,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":22728,"parent_record_offset":22700,"ancestor_record_offset":21820,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":22912,"parent_record_offset":22880,"ancestor_record_offset":21820,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":22476,"parent_record_offset":22452,"ancestor_record_offset":21820,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":26120,"parent_record_offset":26088,"ancestor_record_offset":21820,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":445,"record_offset":22216,"parent_record_offset":22192,"ancestor_record_offset":21820,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":25952,"parent_record_offset":25760,"ancestor_record_offset":21820,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":445,"record_offset":22192,"parent_record_offset":22168,"ancestor_record_offset":21820,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":23064,"parent_record_offset":22812,"ancestor_record_offset":21820,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":22168,"parent_record_offset":22144,"ancestor_record_offset":21820,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":22984,"parent_record_offset":22944,"ancestor_record_offset":21820,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":22812,"parent_record_offset":22784,"ancestor_record_offset":21820,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":22144,"parent_record_offset":22120,"ancestor_record_offset":21820,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::load_decorators::load_decorators::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":445,"record_offset":23168,"parent_record_offset":23132,"ancestor_record_offset":21820,"inlinee":27032,"source_line":89,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::load_decorators::load_decorators::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/load_decorators.rs
//     evidence={"module":445,"record_offset":24424,"parent_record_offset":24388,"ancestor_record_offset":21820,"inlinee":27033,"source_line":90,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6998
// 首条位置: {"module":445,"inlinee":27032}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":23196,"parent_record_offset":23168,"ancestor_record_offset":23168,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":23812,"parent_record_offset":23788,"ancestor_record_offset":23168,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":23836,"parent_record_offset":23812,"ancestor_record_offset":23168,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":23860,"parent_record_offset":23836,"ancestor_record_offset":23168,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":23788,"parent_record_offset":23632,"ancestor_record_offset":23168,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":23560,"parent_record_offset":23528,"ancestor_record_offset":23168,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":23632,"parent_record_offset":23592,"ancestor_record_offset":23168,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":23592,"parent_record_offset":23560,"ancestor_record_offset":23168,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":23676,"parent_record_offset":23632,"ancestor_record_offset":23168,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":23948,"parent_record_offset":23920,"ancestor_record_offset":23168,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":23496,"parent_record_offset":23280,"ancestor_record_offset":23168,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":23528,"parent_record_offset":23496,"ancestor_record_offset":23168,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":23224,"parent_record_offset":23196,"ancestor_record_offset":23168,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":24192,"parent_record_offset":24160,"ancestor_record_offset":23168,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":445,"record_offset":24268,"parent_record_offset":24192,"ancestor_record_offset":23168,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":445,"record_offset":23920,"parent_record_offset":23280,"ancestor_record_offset":23168,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":445,"record_offset":23252,"parent_record_offset":23224,"ancestor_record_offset":23168,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":445,"record_offset":23280,"parent_record_offset":23252,"ancestor_record_offset":23168,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":445,"record_offset":23700,"parent_record_offset":23676,"ancestor_record_offset":23168,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":445,"record_offset":23748,"parent_record_offset":23724,"ancestor_record_offset":23168,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":24100,"parent_record_offset":24060,"ancestor_record_offset":23168,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":24032,"parent_record_offset":24004,"ancestor_record_offset":23168,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":23352,"parent_record_offset":23328,"ancestor_record_offset":23168,"inlinee":10972,"source_line":64,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":24004,"parent_record_offset":23976,"ancestor_record_offset":23168,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":23328,"parent_record_offset":23280,"ancestor_record_offset":23168,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":24128,"parent_record_offset":24100,"ancestor_record_offset":23168,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":23976,"parent_record_offset":23948,"ancestor_record_offset":23168,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":24160,"parent_record_offset":24128,"ancestor_record_offset":23168,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":23724,"parent_record_offset":23700,"ancestor_record_offset":23168,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":445,"record_offset":23448,"parent_record_offset":23424,"ancestor_record_offset":23168,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":445,"record_offset":23424,"parent_record_offset":23400,"ancestor_record_offset":23168,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":24312,"parent_record_offset":24060,"ancestor_record_offset":23168,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":23400,"parent_record_offset":23376,"ancestor_record_offset":23168,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":24232,"parent_record_offset":24192,"ancestor_record_offset":23168,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":24060,"parent_record_offset":24032,"ancestor_record_offset":23168,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":23376,"parent_record_offset":23352,"ancestor_record_offset":23168,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6999
// 首条位置: {"module":445,"inlinee":27033}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":24452,"parent_record_offset":24424,"ancestor_record_offset":24424,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":25068,"parent_record_offset":25044,"ancestor_record_offset":24424,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":25092,"parent_record_offset":25068,"ancestor_record_offset":24424,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":25116,"parent_record_offset":25092,"ancestor_record_offset":24424,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":25044,"parent_record_offset":24888,"ancestor_record_offset":24424,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":24816,"parent_record_offset":24784,"ancestor_record_offset":24424,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":24888,"parent_record_offset":24848,"ancestor_record_offset":24424,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":24848,"parent_record_offset":24816,"ancestor_record_offset":24424,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":24932,"parent_record_offset":24888,"ancestor_record_offset":24424,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":25204,"parent_record_offset":25176,"ancestor_record_offset":24424,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":24752,"parent_record_offset":24536,"ancestor_record_offset":24424,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":24784,"parent_record_offset":24752,"ancestor_record_offset":24424,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":24480,"parent_record_offset":24452,"ancestor_record_offset":24424,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":25500,"parent_record_offset":25456,"ancestor_record_offset":24424,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":445,"record_offset":25604,"parent_record_offset":25500,"ancestor_record_offset":24424,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":445,"record_offset":25176,"parent_record_offset":24536,"ancestor_record_offset":24424,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":445,"record_offset":24508,"parent_record_offset":24480,"ancestor_record_offset":24424,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":445,"record_offset":24536,"parent_record_offset":24508,"ancestor_record_offset":24424,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":445,"record_offset":24956,"parent_record_offset":24932,"ancestor_record_offset":24424,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":445,"record_offset":25004,"parent_record_offset":24980,"ancestor_record_offset":24424,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":25368,"parent_record_offset":25316,"ancestor_record_offset":24424,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":25288,"parent_record_offset":25260,"ancestor_record_offset":24424,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":24608,"parent_record_offset":24584,"ancestor_record_offset":24424,"inlinee":10972,"source_line":64,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":25260,"parent_record_offset":25232,"ancestor_record_offset":24424,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":24584,"parent_record_offset":24536,"ancestor_record_offset":24424,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":25412,"parent_record_offset":25368,"ancestor_record_offset":24424,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":25232,"parent_record_offset":25204,"ancestor_record_offset":24424,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":25456,"parent_record_offset":25412,"ancestor_record_offset":24424,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":24980,"parent_record_offset":24956,"ancestor_record_offset":24424,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":445,"record_offset":24704,"parent_record_offset":24680,"ancestor_record_offset":24424,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":445,"record_offset":24680,"parent_record_offset":24656,"ancestor_record_offset":24424,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":25652,"parent_record_offset":25316,"ancestor_record_offset":24424,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":24656,"parent_record_offset":24632,"ancestor_record_offset":24424,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":25556,"parent_record_offset":25500,"ancestor_record_offset":24424,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":25316,"parent_record_offset":25288,"ancestor_record_offset":24424,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":24632,"parent_record_offset":24608,"ancestor_record_offset":24424,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":445,"record_offset":27180,"rva":8875232,"va":5377584352}
// 编译实例 RVAs: 0x876ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1905964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e8d4eae2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377584647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1906426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e806e9e2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377585125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8862441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e817c578ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377584628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414aa360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12792982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e89634c300","size":5,"soft_fail":false,"target_va":5390377824,"va":5377584837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414df050
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13009666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e80283c600","size":5,"soft_fail":false,"target_va":5390594128,"va":5377584457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38767190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e8568a4f02","size":5,"soft_fail":false,"target_va":5416352064,"va":5377584869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7210414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e8ae056e00","size":5,"soft_fail":false,"target_va":5384794864,"va":5377584445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39036180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e814a55302","size":5,"soft_fail":false,"target_va":5416620794,"va":5377584609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7038893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e8ad676b00","size":5,"soft_fail":false,"target_va":5384624080,"va":5377585182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39036761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e859a75302","size":5,"soft_fail":false,"target_va":5416621762,"va":5377584996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39036650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e8eaa65302","size":5,"soft_fail":false,"target_va":5416621632,"va":5377584977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3880852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377584352,"raw_hex":"e894373b00","size":5,"soft_fail":false,"target_va":5381465408,"va":5377584551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":445,"record_offset":27348,"parent_record_offset":27180,"ancestor_record_offset":27180,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":28748,"parent_record_offset":27180,"ancestor_record_offset":27180,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":28652,"parent_record_offset":27180,"ancestor_record_offset":27180,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":27500,"parent_record_offset":27180,"ancestor_record_offset":27180,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":28960,"parent_record_offset":27180,"ancestor_record_offset":27180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":445,"record_offset":27428,"parent_record_offset":27180,"ancestor_record_offset":27180,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":28084,"parent_record_offset":28060,"ancestor_record_offset":27180,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":28108,"parent_record_offset":28084,"ancestor_record_offset":27180,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":29148,"parent_record_offset":29124,"ancestor_record_offset":27180,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":29172,"parent_record_offset":29148,"ancestor_record_offset":27180,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":28132,"parent_record_offset":28108,"ancestor_record_offset":27180,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":29196,"parent_record_offset":29172,"ancestor_record_offset":27180,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":28060,"parent_record_offset":27904,"ancestor_record_offset":27180,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":29124,"parent_record_offset":29032,"ancestor_record_offset":27180,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":445,"record_offset":27396,"parent_record_offset":27372,"ancestor_record_offset":27180,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":28836,"parent_record_offset":28804,"ancestor_record_offset":27180,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":27840,"parent_record_offset":27812,"ancestor_record_offset":27180,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":29060,"parent_record_offset":29032,"ancestor_record_offset":27180,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":29032,"parent_record_offset":29008,"ancestor_record_offset":27180,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":28884,"parent_record_offset":28860,"ancestor_record_offset":27180,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":28860,"parent_record_offset":28836,"ancestor_record_offset":27180,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":27904,"parent_record_offset":27868,"ancestor_record_offset":27180,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":27868,"parent_record_offset":27840,"ancestor_record_offset":27180,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":29008,"parent_record_offset":28984,"ancestor_record_offset":27180,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":27948,"parent_record_offset":27904,"ancestor_record_offset":27180,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":28804,"parent_record_offset":28772,"ancestor_record_offset":27180,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":28220,"parent_record_offset":28192,"ancestor_record_offset":27180,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":28772,"parent_record_offset":28748,"ancestor_record_offset":27180,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":27784,"parent_record_offset":27572,"ancestor_record_offset":27180,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":27812,"parent_record_offset":27784,"ancestor_record_offset":27180,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":27524,"parent_record_offset":27500,"ancestor_record_offset":27180,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":28464,"parent_record_offset":28432,"ancestor_record_offset":27180,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":445,"record_offset":28540,"parent_record_offset":28464,"ancestor_record_offset":27180,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":445,"record_offset":28192,"parent_record_offset":27572,"ancestor_record_offset":27180,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":445,"record_offset":27548,"parent_record_offset":27524,"ancestor_record_offset":27180,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":445,"record_offset":27572,"parent_record_offset":27548,"ancestor_record_offset":27180,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":445,"record_offset":27972,"parent_record_offset":27948,"ancestor_record_offset":27180,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":445,"record_offset":28020,"parent_record_offset":27996,"ancestor_record_offset":27180,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":28372,"parent_record_offset":28332,"ancestor_record_offset":27180,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":28304,"parent_record_offset":28276,"ancestor_record_offset":27180,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":27640,"parent_record_offset":27616,"ancestor_record_offset":27180,"inlinee":10972,"source_line":64,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":28276,"parent_record_offset":28248,"ancestor_record_offset":27180,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":27616,"parent_record_offset":27572,"ancestor_record_offset":27180,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":28400,"parent_record_offset":28372,"ancestor_record_offset":27180,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":28248,"parent_record_offset":28220,"ancestor_record_offset":27180,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":28432,"parent_record_offset":28400,"ancestor_record_offset":27180,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":27996,"parent_record_offset":27972,"ancestor_record_offset":27180,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":29092,"parent_record_offset":29060,"ancestor_record_offset":27180,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":445,"record_offset":27372,"parent_record_offset":27348,"ancestor_record_offset":27180,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":445,"record_offset":27736,"parent_record_offset":27712,"ancestor_record_offset":27180,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":27468,"parent_record_offset":27428,"ancestor_record_offset":27180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":445,"record_offset":27712,"parent_record_offset":27688,"ancestor_record_offset":27180,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":28584,"parent_record_offset":28332,"ancestor_record_offset":27180,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":27688,"parent_record_offset":27664,"ancestor_record_offset":27180,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":28504,"parent_record_offset":28464,"ancestor_record_offset":27180,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":445,"record_offset":28680,"parent_record_offset":28652,"ancestor_record_offset":27180,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":445,"record_offset":28708,"parent_record_offset":28680,"ancestor_record_offset":27180,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":28332,"parent_record_offset":28304,"ancestor_record_offset":27180,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":27664,"parent_record_offset":27640,"ancestor_record_offset":27180,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6996
// 首条位置: {"module":445,"inlinee":27030}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":445,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":6340,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":445,"record_offset":19304,"rva":8872144,"va":5377581264}
// 编译实例 RVAs: 0x8760d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1902738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e86ef7e2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377581421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-1903039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e841f6e2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377581738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8859215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e8b1d178ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377581402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ba370
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8110410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e8b63e84ff","size":5,"soft_fail":false,"target_va":5369471856,"va":5377582261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39475129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e8b9575a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5377581922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39038728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e808af5302","size":5,"soft_fail":false,"target_va":5416620769,"va":5377582036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39039400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e8a8b15302","size":5,"soft_fail":false,"target_va":5416620794,"va":5377581389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39039513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e819b25302","size":5,"soft_fail":false,"target_va":5416621536,"va":5377582018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39039602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e872b25302","size":5,"soft_fail":false,"target_va":5416621600,"va":5377581993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1684427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e8cbb31900","size":5,"soft_fail":false,"target_va":5379265744,"va":5377581312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::as_triangles
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3881435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e8db393b00","size":5,"soft_fail":false,"target_va":5381463200,"va":5377581760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3881799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e8473b3b00","size":5,"soft_fail":false,"target_va":5381463584,"va":5377581780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3884071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e827443b00","size":5,"soft_fail":false,"target_va":5381465408,"va":5377581332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_convex
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1585151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377581264,"raw_hex":"e8ff2f1800","size":5,"soft_fail":false,"target_va":5379166768,"va":5377581612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":445,"record_offset":20976,"parent_record_offset":19304,"ancestor_record_offset":19304,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":19504,"parent_record_offset":19304,"ancestor_record_offset":19304,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":445,"record_offset":20652,"parent_record_offset":19304,"ancestor_record_offset":19304,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":445,"record_offset":19468,"parent_record_offset":19304,"ancestor_record_offset":19304,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":20688,"parent_record_offset":19304,"ancestor_record_offset":19304,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":20084,"parent_record_offset":20060,"ancestor_record_offset":19304,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":20108,"parent_record_offset":20084,"ancestor_record_offset":19304,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":20872,"parent_record_offset":20848,"ancestor_record_offset":19304,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":20896,"parent_record_offset":20872,"ancestor_record_offset":19304,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":20132,"parent_record_offset":20108,"ancestor_record_offset":19304,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":20920,"parent_record_offset":20896,"ancestor_record_offset":19304,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":20060,"parent_record_offset":19904,"ancestor_record_offset":19304,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":20848,"parent_record_offset":20760,"ancestor_record_offset":19304,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":445,"record_offset":21048,"parent_record_offset":21008,"ancestor_record_offset":19304,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":445,"record_offset":21008,"parent_record_offset":20976,"ancestor_record_offset":19304,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":19840,"parent_record_offset":19812,"ancestor_record_offset":19304,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":20788,"parent_record_offset":20760,"ancestor_record_offset":19304,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":20760,"parent_record_offset":20736,"ancestor_record_offset":19304,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":19904,"parent_record_offset":19868,"ancestor_record_offset":19304,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":19868,"parent_record_offset":19840,"ancestor_record_offset":19304,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":20736,"parent_record_offset":20712,"ancestor_record_offset":19304,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":19948,"parent_record_offset":19904,"ancestor_record_offset":19304,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":20220,"parent_record_offset":20192,"ancestor_record_offset":19304,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":19784,"parent_record_offset":19576,"ancestor_record_offset":19304,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":19812,"parent_record_offset":19784,"ancestor_record_offset":19304,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":19528,"parent_record_offset":19504,"ancestor_record_offset":19304,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":20464,"parent_record_offset":20432,"ancestor_record_offset":19304,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":445,"record_offset":20540,"parent_record_offset":20464,"ancestor_record_offset":19304,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":445,"record_offset":20192,"parent_record_offset":19576,"ancestor_record_offset":19304,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":445,"record_offset":19552,"parent_record_offset":19528,"ancestor_record_offset":19304,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":445,"record_offset":19576,"parent_record_offset":19552,"ancestor_record_offset":19304,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":445,"record_offset":19972,"parent_record_offset":19948,"ancestor_record_offset":19304,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":445,"record_offset":20020,"parent_record_offset":19996,"ancestor_record_offset":19304,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":20372,"parent_record_offset":20332,"ancestor_record_offset":19304,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":20304,"parent_record_offset":20276,"ancestor_record_offset":19304,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":445,"record_offset":19640,"parent_record_offset":19616,"ancestor_record_offset":19304,"inlinee":10972,"source_line":64,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":20276,"parent_record_offset":20248,"ancestor_record_offset":19304,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":19616,"parent_record_offset":19576,"ancestor_record_offset":19304,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":20400,"parent_record_offset":20372,"ancestor_record_offset":19304,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":20248,"parent_record_offset":20220,"ancestor_record_offset":19304,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":20432,"parent_record_offset":20400,"ancestor_record_offset":19304,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":19996,"parent_record_offset":19972,"ancestor_record_offset":19304,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":20816,"parent_record_offset":20788,"ancestor_record_offset":19304,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":445,"record_offset":19736,"parent_record_offset":19712,"ancestor_record_offset":19304,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":20712,"parent_record_offset":20688,"ancestor_record_offset":19304,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":445,"record_offset":19712,"parent_record_offset":19688,"ancestor_record_offset":19304,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":20584,"parent_record_offset":20332,"ancestor_record_offset":19304,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":19688,"parent_record_offset":19664,"ancestor_record_offset":19304,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":20504,"parent_record_offset":20464,"ancestor_record_offset":19304,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":20332,"parent_record_offset":20304,"ancestor_record_offset":19304,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":19664,"parent_record_offset":19640,"ancestor_record_offset":19304,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_decorators::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6995
// 首条位置: {"module":445,"inlinee":27029}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":445,"record_offset":4848,"parent_record_offset":4800,"ancestor_record_offset":4800,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":5068,"parent_record_offset":5044,"ancestor_record_offset":4800,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":4800,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":4800,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":5044,"parent_record_offset":4988,"ancestor_record_offset":4800,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":5016,"parent_record_offset":4988,"ancestor_record_offset":4800,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":4800,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":4800,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":4892,"parent_record_offset":4848,"ancestor_record_offset":4800,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::load_decorators::load_collision_as_bbx
// 记录形式: compiled_procedure; instances=1
// 源码行: 669
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":445,"record_offset":29256,"rva":8876096,"va":5377585216}
// 编译实例 RVAs: 0x877040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1907059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e88de6e2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377585742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1907565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e893e4e2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377586264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8863540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e8ccc078ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377585727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ba370
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8114263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e8a92f84ff","size":5,"soft_fail":false,"target_va":5369471856,"va":5377586114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c915a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4301098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e82aa14100","size":5,"soft_fail":false,"target_va":5381887392,"va":5377586289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5739588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e844945700","size":5,"soft_fail":false,"target_va":5383325056,"va":5377585463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39471279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e8af485a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5377585772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39034747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e87b9f5302","size":5,"soft_fail":false,"target_va":5416620769,"va":5377586017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39035630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e8eea25302","size":5,"soft_fail":false,"target_va":5416621600,"va":5377585965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39164366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e8ce995502","size":5,"soft_fail":false,"target_va":5416750372,"va":5377586001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1680430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e82ea41900","size":5,"soft_fail":false,"target_va":5379265744,"va":5377585309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::collision_bbx
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3878246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e8662d3b00","size":5,"soft_fail":false,"target_va":5381463584,"va":5377585333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_cuboid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1581694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e87e221800","size":5,"soft_fail":false,"target_va":5379167328,"va":5377585629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2924842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e82aa12c00","size":5,"soft_fail":false,"target_va":5380510400,"va":5377585553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5690215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377585216,"raw_hex":"e867d35600","size":5,"soft_fail":false,"target_va":5383275712,"va":5377585492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":445,"record_offset":29664,"parent_record_offset":29256,"ancestor_record_offset":29256,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":445,"record_offset":29428,"parent_record_offset":29256,"ancestor_record_offset":29256,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":29968,"parent_record_offset":29256,"ancestor_record_offset":29256,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":29608,"parent_record_offset":29256,"ancestor_record_offset":29256,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":29464,"parent_record_offset":29256,"ancestor_record_offset":29256,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":445,"record_offset":29576,"parent_record_offset":29256,"ancestor_record_offset":29256,"inlinee":11331,"source_line":74,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":445,"record_offset":29492,"parent_record_offset":29256,"ancestor_record_offset":29256,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":445,"record_offset":29548,"parent_record_offset":29256,"ancestor_record_offset":29256,"inlinee":11329,"source_line":89,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":445,"record_offset":29520,"parent_record_offset":29256,"ancestor_record_offset":29256,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":29800,"parent_record_offset":29776,"ancestor_record_offset":29256,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":29824,"parent_record_offset":29800,"ancestor_record_offset":29256,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":30152,"parent_record_offset":30128,"ancestor_record_offset":29256,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":30176,"parent_record_offset":30152,"ancestor_record_offset":29256,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":29848,"parent_record_offset":29824,"ancestor_record_offset":29256,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":30200,"parent_record_offset":30176,"ancestor_record_offset":29256,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":29776,"parent_record_offset":29744,"ancestor_record_offset":29256,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":30128,"parent_record_offset":30040,"ancestor_record_offset":29256,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":445,"record_offset":29744,"parent_record_offset":29700,"ancestor_record_offset":29256,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":445,"record_offset":29700,"parent_record_offset":29664,"ancestor_record_offset":29256,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":30068,"parent_record_offset":30040,"ancestor_record_offset":29256,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":30040,"parent_record_offset":30016,"ancestor_record_offset":29256,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":445,"record_offset":30016,"parent_record_offset":29992,"ancestor_record_offset":29256,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":445,"record_offset":30280,"parent_record_offset":30256,"ancestor_record_offset":29256,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":30096,"parent_record_offset":30068,"ancestor_record_offset":29256,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":29896,"parent_record_offset":29700,"ancestor_record_offset":29256,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":30316,"parent_record_offset":30280,"ancestor_record_offset":29256,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":30340,"parent_record_offset":30316,"ancestor_record_offset":29256,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":30372,"parent_record_offset":30280,"ancestor_record_offset":29256,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00445.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_collision_as_bbx
// 信息 ID: db150fef9af618cf59720dae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators
// 信息 ID: e9cb61cee0c3adc1f17e413d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9d6c68b144fbee7ac2a249f9: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> > @ 0x287dbf -> 0x872550 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> @ 0x5f630b -> 0x872550 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::reload_decorators_after_mod_swap @ 0x87758b -> 0x872550 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$0
// 信息 ID: 9d933bea804c17f0203fbfb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x872ad9 -> 0x876510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x87469d -> 0x876510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$0::closure$0
// 信息 ID: 800e435410c76a64746d40e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators::closure$0; module=445, inline_record=23168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$0::closure$1
// 信息 ID: 341803e2064a04a7a04a3c1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators::closure$0; module=445, inline_record=24424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$1
// 信息 ID: 1c0108673e6fe622e84848ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x872b26 -> 0x876ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x8746c3 -> 0x876ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x8731ba -> 0x876ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x873197 -> 0x876ce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$2
// 信息 ID: 4be65ee01306385531237f20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=445, inline_record=6340; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=445, inline_record=6400; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=445, inline_record=6644; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=445, inline_record=7136; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=445, inline_record=9192; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=445, inline_record=13824; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=445, inline_record=13884; 不是独立调用边。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=445, inline_record=14260; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$3
// 信息 ID: 05b36b7c0c9de38d9f4b607a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::load_decorators::closure$4
// 信息 ID: 05ec52d9e4478c2336df8b30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::load_decorators::load_decorators", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::load_decorators::load_decorators; module=445, inline_record=4800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::load_decorators::reload_decorators_after_mod_swap
// 信息 ID: cc08636052f7637a75556c61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1a2113deb3d76cfc00532a3e: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> > @ 0x2ac3a3 -> 0x877550 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> @ 0x5f6624 -> 0x877550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
