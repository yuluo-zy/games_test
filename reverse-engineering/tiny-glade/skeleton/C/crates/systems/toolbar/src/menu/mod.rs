// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/menu/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 49678c378536e8ff2581838c771f898e2999dd16d8d7309c101454b41a6c45a1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 353, 1304, 2872
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":353,"record_offset":1804,"rva":7538064,"va":5376247184}
// 编译实例 RVAs: 0x730590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$bevy_ecs..system..system..System$u2b$In$u20$$u3d$$u20$$LP$$RP$$u2b$Out$u20$$u3d$$u20$$LP$$RP$$GT$$u20$as$u20$bevy_ecs..schedule..config..Schedulable$GT$::into_config
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t7971419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e85ba27900","size":5,"soft_fail":false,"target_va":5384218864,"va":5376247440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-568699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e88552f7ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5376247382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=12
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7525180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e8c42c8dff","size":5,"soft_fail":false,"target_va":5368722192,"va":5376247367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=11
//     target_aliases=118 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7045154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e8de7f94ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5376250813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140090990
//     owner=unresolved_namespace; count=1
//     target_aliases=31 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6949378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e8fef595ff","size":5,"soft_fail":false,"target_va":5369301392,"va":5376250765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5651752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e8d8c2a9ff","size":5,"soft_fail":false,"target_va":5370597904,"va":5376249651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141181fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10816414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e89e0ba500","size":5,"soft_fail":false,"target_va":5387067328,"va":5376250909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b3490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11021947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e87b2ea800","size":5,"soft_fail":false,"target_va":5387269264,"va":5376247312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b3a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11023368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e80834a800","size":5,"soft_fail":false,"target_va":5387270672,"va":5376247299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b3f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11024812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e8ac39a800","size":5,"soft_fail":false,"target_va":5387272080,"va":5376247263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b4b10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11027730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e81245a800","size":5,"soft_fail":false,"target_va":5387275024,"va":5376247289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b50d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11029166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e8ae4aa800","size":5,"soft_fail":false,"target_va":5387276496,"va":5376247325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b5650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11030662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e88650a800","size":5,"soft_fail":false,"target_va":5387277904,"va":5376247237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ba530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11048492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e82c96a800","size":5,"soft_fail":false,"target_va":5387298096,"va":5376249599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ba710
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11051113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e869a0a800","size":5,"soft_fail":false,"target_va":5387298576,"va":5376247458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ba920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11049598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e87e9aa800","size":5,"soft_fail":false,"target_va":5387299104,"va":5376249501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40809636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e8a4b46e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5376247415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t40370249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e849006802","size":5,"soft_fail":false,"target_va":5416620769,"va":5376250515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11034451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e8535fa800","size":5,"soft_fail":false,"target_va":5387283968,"va":5376249512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11045787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e89b8ba800","size":5,"soft_fail":false,"target_va":5387295424,"va":5376249632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11128774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376247184,"raw_hex":"e8c6cfa900","size":5,"soft_fail":false,"target_va":5387378784,"va":5376250005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":353,"record_offset":5412,"parent_record_offset":1804,"ancestor_record_offset":1804,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":353,"record_offset":5020,"parent_record_offset":1804,"ancestor_record_offset":1804,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":353,"record_offset":2016,"parent_record_offset":1804,"ancestor_record_offset":1804,"inlinee":4638,"source_line":448,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":353,"record_offset":5312,"parent_record_offset":1804,"ancestor_record_offset":1804,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":353,"record_offset":1940,"parent_record_offset":1804,"ancestor_record_offset":1804,"inlinee":22091,"source_line":110,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$21::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":353,"record_offset":1968,"parent_record_offset":1804,"ancestor_record_offset":1804,"inlinee":22093,"source_line":132,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$24::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":353,"record_offset":1992,"parent_record_offset":1804,"ancestor_record_offset":1804,"inlinee":22095,"source_line":139,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":1804,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":1804,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":1804,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":2200,"parent_record_offset":2168,"ancestor_record_offset":1804,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":353,"record_offset":2168,"parent_record_offset":2120,"ancestor_record_offset":1804,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":353,"record_offset":2120,"parent_record_offset":2052,"ancestor_record_offset":1804,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":353,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":1804,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":353,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":1804,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":353,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":1804,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":353,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":1804,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":353,"record_offset":6760,"parent_record_offset":6720,"ancestor_record_offset":1804,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":353,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":1804,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":353,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":1804,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":353,"record_offset":5496,"parent_record_offset":5464,"ancestor_record_offset":1804,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::into_configs
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":353,"record_offset":2052,"parent_record_offset":2016,"ancestor_record_offset":1804,"inlinee":4259,"source_line":565,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":353,"record_offset":5352,"parent_record_offset":5312,"ancestor_record_offset":1804,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":353,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":1804,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":353,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":1804,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":353,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":1804,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":353,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":1804,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":353,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":1804,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":353,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":1804,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::function_system::impl$5::into_system
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs
//     evidence={"module":353,"record_offset":2092,"parent_record_offset":2052,"ancestor_record_offset":1804,"inlinee":5856,"source_line":595,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":353,"record_offset":5464,"parent_record_offset":5436,"ancestor_record_offset":1804,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":353,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":1804,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":353,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":1804,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":353,"record_offset":6168,"parent_record_offset":6144,"ancestor_record_offset":1804,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":353,"record_offset":6196,"parent_record_offset":6168,"ancestor_record_offset":1804,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":353,"record_offset":5736,"parent_record_offset":5712,"ancestor_record_offset":1804,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":353,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":1804,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":353,"record_offset":5968,"parent_record_offset":5940,"ancestor_record_offset":1804,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":353,"record_offset":5940,"parent_record_offset":5912,"ancestor_record_offset":1804,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":353,"record_offset":6608,"parent_record_offset":6544,"ancestor_record_offset":1804,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":353,"record_offset":6420,"parent_record_offset":5464,"ancestor_record_offset":1804,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":353,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":1804,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":353,"record_offset":6956,"parent_record_offset":5464,"ancestor_record_offset":1804,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":353,"record_offset":6672,"parent_record_offset":6492,"ancestor_record_offset":1804,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":353,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":1804,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":353,"record_offset":6084,"parent_record_offset":6060,"ancestor_record_offset":1804,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":353,"record_offset":6544,"parent_record_offset":6516,"ancestor_record_offset":1804,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":353,"record_offset":6580,"parent_record_offset":6544,"ancestor_record_offset":1804,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":353,"record_offset":6900,"parent_record_offset":6720,"ancestor_record_offset":1804,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":353,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":1804,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":353,"record_offset":5880,"parent_record_offset":5852,"ancestor_record_offset":1804,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":353,"record_offset":5912,"parent_record_offset":5880,"ancestor_record_offset":1804,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":353,"record_offset":5852,"parent_record_offset":5632,"ancestor_record_offset":1804,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":353,"record_offset":5712,"parent_record_offset":5632,"ancestor_record_offset":1804,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":353,"record_offset":6272,"parent_record_offset":5632,"ancestor_record_offset":1804,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":353,"record_offset":5796,"parent_record_offset":5632,"ancestor_record_offset":1804,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":353,"record_offset":5556,"parent_record_offset":5520,"ancestor_record_offset":1804,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":353,"record_offset":6120,"parent_record_offset":6012,"ancestor_record_offset":1804,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":353,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":1804,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":353,"record_offset":6364,"parent_record_offset":5632,"ancestor_record_offset":1804,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":353,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":1804,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":353,"record_offset":5608,"parent_record_offset":5584,"ancestor_record_offset":1804,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":353,"record_offset":5584,"parent_record_offset":5556,"ancestor_record_offset":1804,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":353,"record_offset":5632,"parent_record_offset":5608,"ancestor_record_offset":1804,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":353,"record_offset":5684,"parent_record_offset":5632,"ancestor_record_offset":1804,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":353,"record_offset":6012,"parent_record_offset":5632,"ancestor_record_offset":1804,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":353,"record_offset":6244,"parent_record_offset":5632,"ancestor_record_offset":1804,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::impl$1::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":353,"record_offset":10420,"rva":7542240,"va":5376251360}
// 编译实例 RVAs: 0x7315e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11032563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251360,"raw_hex":"e8f357a800","size":5,"soft_fail":false,"target_va":5387283968,"va":5376251400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11231006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251360,"raw_hex":"e81e5fab00","size":5,"soft_fail":false,"target_va":5387482400,"va":5376251389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":353,"record_offset":10532,"parent_record_offset":10420,"ancestor_record_offset":10420,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::ellipsize
// 记录形式: compiled_procedure; instances=1
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":353,"record_offset":10564,"rva":7542304,"va":5376251424}
// 编译实例 RVAs: 0x731620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..iter..traits..collect..FromIterator$LT$char$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11110565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251424,"raw_hex":"e8a588a900","size":5,"soft_fail":false,"target_va":5387362112,"va":5376251542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-572962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251424,"raw_hex":"e8de41f7ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5376251645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-572903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251424,"raw_hex":"e81942f7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5376251602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7529439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251424,"raw_hex":"e8211c8dff","size":5,"soft_fail":false,"target_va":5368722192,"va":5376251626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40805380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251424,"raw_hex":"e804a46e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5376251671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8543282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251424,"raw_hex":"e8325c8200","size":5,"soft_fail":false,"target_va":5384794864,"va":5376251577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40369288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251424,"raw_hex":"e888fc6702","size":5,"soft_fail":false,"target_va":5416620794,"va":5376251501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::count::char_count_general_case
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8578410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251424,"raw_hex":"e86ae58200","size":5,"soft_fail":false,"target_va":5384829888,"va":5376251473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::count::do_count_chars
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8576809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376251424,"raw_hex":"e829df8200","size":5,"soft_fail":false,"target_va":5384828320,"va":5376251506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":353,"record_offset":11540,"parent_record_offset":10564,"ancestor_record_offset":10564,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":353,"record_offset":10764,"parent_record_offset":10564,"ancestor_record_offset":10564,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":353,"record_offset":11908,"parent_record_offset":10564,"ancestor_record_offset":10564,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":353,"record_offset":11512,"parent_record_offset":10564,"ancestor_record_offset":10564,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":353,"record_offset":11456,"parent_record_offset":10564,"ancestor_record_offset":10564,"inlinee":17297,"source_line":1414,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":353,"record_offset":11624,"parent_record_offset":10564,"ancestor_record_offset":10564,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$0::count
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":353,"record_offset":10696,"parent_record_offset":10564,"ancestor_record_offset":10564,"inlinee":17284,"source_line":46,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":10564,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":11288,"parent_record_offset":11264,"ancestor_record_offset":10564,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":11800,"parent_record_offset":11776,"ancestor_record_offset":10564,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":10564,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":11312,"parent_record_offset":11288,"ancestor_record_offset":10564,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":11848,"parent_record_offset":11824,"ancestor_record_offset":10564,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":11240,"parent_record_offset":11088,"ancestor_record_offset":10564,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":353,"record_offset":11776,"parent_record_offset":11720,"ancestor_record_offset":10564,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":353,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":10564,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":353,"record_offset":11028,"parent_record_offset":11000,"ancestor_record_offset":10564,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":353,"record_offset":11748,"parent_record_offset":11720,"ancestor_record_offset":10564,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":353,"record_offset":11720,"parent_record_offset":11696,"ancestor_record_offset":10564,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":353,"record_offset":11088,"parent_record_offset":11056,"ancestor_record_offset":10564,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":353,"record_offset":11056,"parent_record_offset":11028,"ancestor_record_offset":10564,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":353,"record_offset":11696,"parent_record_offset":11672,"ancestor_record_offset":10564,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":353,"record_offset":11128,"parent_record_offset":11088,"ancestor_record_offset":10564,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":353,"record_offset":10912,"parent_record_offset":10876,"ancestor_record_offset":10564,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":353,"record_offset":10940,"parent_record_offset":10912,"ancestor_record_offset":10564,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":353,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10564,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":353,"record_offset":10876,"parent_record_offset":10848,"ancestor_record_offset":10564,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":353,"record_offset":10848,"parent_record_offset":10820,"ancestor_record_offset":10564,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$127::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":353,"record_offset":10792,"parent_record_offset":10764,"ancestor_record_offset":10564,"inlinee":4518,"source_line":3043,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":353,"record_offset":10820,"parent_record_offset":10792,"ancestor_record_offset":10564,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":353,"record_offset":11000,"parent_record_offset":10968,"ancestor_record_offset":10564,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":353,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":10564,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":353,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":10564,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::Take::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":353,"record_offset":11480,"parent_record_offset":11456,"ancestor_record_offset":10564,"inlinee":17299,"source_line":23,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":353,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":10564,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":353,"record_offset":11564,"parent_record_offset":11540,"ancestor_record_offset":10564,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":353,"record_offset":11368,"parent_record_offset":10968,"ancestor_record_offset":10564,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":353,"record_offset":11392,"parent_record_offset":11368,"ancestor_record_offset":10564,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":353,"record_offset":11648,"parent_record_offset":11624,"ancestor_record_offset":10564,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   [pdb_inline_transitive] core::str::count::count_chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\count.rs
//     evidence={"module":353,"record_offset":10724,"parent_record_offset":10696,"ancestor_record_offset":10564,"inlinee":17286,"source_line":27,"archive":"_pdb/C/modules/00353.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::advance_menu_state
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5655
// 首条位置: {"module":1304,"inlinee":22101}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1304,"record_offset":9988,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1304,"record_offset":10112,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1304,"record_offset":10624,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1304,"record_offset":10044,"parent_record_offset":9952,"ancestor_record_offset":9952,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1304,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":9952,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1304,"record_offset":10420,"parent_record_offset":10396,"ancestor_record_offset":9952,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1304,"record_offset":10396,"parent_record_offset":10372,"ancestor_record_offset":9952,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1304,"record_offset":10348,"parent_record_offset":10308,"ancestor_record_offset":9952,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1304,"record_offset":10276,"parent_record_offset":10240,"ancestor_record_offset":9952,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1304,"record_offset":10308,"parent_record_offset":10276,"ancestor_record_offset":9952,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1304,"record_offset":10012,"parent_record_offset":9988,"ancestor_record_offset":9952,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1304,"record_offset":10136,"parent_record_offset":10112,"ancestor_record_offset":9952,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1304,"record_offset":10216,"parent_record_offset":10192,"ancestor_record_offset":9952,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1304,"record_offset":10192,"parent_record_offset":10112,"ancestor_record_offset":9952,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1304,"record_offset":10240,"parent_record_offset":10216,"ancestor_record_offset":9952,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1304,"record_offset":10072,"parent_record_offset":10044,"ancestor_record_offset":9952,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01304.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1304,"record_offset":10460,"parent_record_offset":10308,"ancestor_record_offset":9952,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01304.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::MenuFlowPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::MenuFlowPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<toolbar::menu::MenuFlowPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<toolbar::menu::MenuFlowPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<toolbar::menu::MenuFlowPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<toolbar::menu::MenuFlowPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<toolbar::menu::MenuFlowPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<toolbar::menu::MenuFlowPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["toolbar::menu::MenuFlowPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::ReflectionListingPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::tgsr_capture::TgsrCapturePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::run_game::ReflectionListingPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::tgsr_capture::TgsrCapturePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::SplashScreenPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::SplashScreenPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<toolbar::menu::SplashScreenPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<toolbar::menu::SplashScreenPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<toolbar::menu::SplashScreenPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$9::add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::startup::MinimalStartupPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::logo_animation::AnimatedLogo,bevy_app::plugin::sealed::PluginMarker,country_core::resources::loading_screen::LoadingScreenPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::sealed::impl$9::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "country_core::startup::MinimalStartupPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::logo_animation::AnimatedLogo", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::loading_screen::LoadingScreenPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 66; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::system::system_param::Local<toolbar::demo_stuff::demo_show_full_glade_size::TextAnchors>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::system::system_param::Local<toolbar::demo_stuff::demo_show_full_glade_size::TextAnchors>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::demo_stuff::demo_show_full_glade_size::TextAnchors>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers> >", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>", "bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::DisplayRollercoasterPrompt>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>", "bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::grading_lut::LutGalleryState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::photomode::lut_gallery::LutGalleryUiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref_mut$<toolbar::resouces_and_components::UiSlider>,ref$<country_core::components::UiInteractionState> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::system::system_param::Local<toolbar::photomode::ui_slider::GrabOffset>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::system::system_param::Local<bool>,bevy_ecs::system::system_param::Local<enum2$<country_core::systems::ui::resources::widget_params::UiFocus> > >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::ResMut<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref_mut$<toolbar::resouces_and_components::UiSlider>,ref$<country_core::components::UiInteractionState> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::system_param::Local<toolbar::photomode::ui_slider::GrabOffset>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::system::system_param::Local<bool>,bevy_ecs::system::system_param::Local<enum2$<country_core::systems::ui::resources::widget_params::UiFocus> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::EraserSizeMultiplier>", "bevy_ecs::change_detection::ResMut<country_core::resources::brush_sizes::BrushSizes>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<toolbar::ui_interaction::UiDragEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref_mut$<toolbar::resouces_and_components::UiSlider>,ref$<country_core::components::UiInteractionState> >,tuple$<> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::photomode::ui_slider::GrabOffset>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<bool>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<country_core::systems::ui::resources::widget_params::UiFocus> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::SteamdeckKeyboardRequested>,bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History> >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::message::message_writer::MessageWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::message::message_reader::MessageReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::SteamdeckKeyboardRequested>", "bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History> >", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::message::message_writer::MessageWriter<feedback_reporter::ReportPlayerFeedbackCmd>", "bevy_ecs::message::message_reader::MessageReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::SteamdeckKeyboardRequested>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<feedback_reporter::ReportPlayerFeedbackCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>", "bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::UiMouseInteractable>,ref$<toolbar::resouces_and_components::TaffyNode>,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<country_core::components::HistoryButton> > >,enum2$<core::option::Option<ref$<country_core::components::UiNav> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::UiDirectFocus>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<toolbar::ui_interaction::UiDragEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::popup_bounds::UiPopupBounds>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::system::system_param::Local<toolbar::menu::welcome::SteamdeckWelcomeMsg>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::system::system_param::Local<toolbar::menu::welcome::SteamdeckWelcomeMsg>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::welcome::SteamdeckWelcomeMsg>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: toolbar::menu::advance_menu_state
// 信息 ID: fb2c5f79ab40c27cffee5b3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f094d9ecd5c6e6be86f4b5ae: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustClosed>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustClosed>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustClosed>)>; module=1304, inline_record=9952; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustClosed>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustClosed>)> >; module=2872, inline_record=985932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::ellipsize
// 信息 ID: 427cc59ac95b64fd81e0998f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items<toolbar::menu::mod_checklist::replacement_mod_checklist::closure_env$0> @ 0x231b4e3 -> 0x731620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items_undecorated<toolbar::menu::missing_mods::ui::closure$3::closure_env$0> @ 0x231e155 -> 0x731620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::mod_row::mod_row<toolbar::menu::mods_mine_tab::list_slot::closure$1::closure_env$3> @ 0x2971749 -> 0x731620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::mod_row::mod_row<toolbar::menu::mods_workshop_tab::list_slot::closure$3::closure_env$2> @ 0x2972106 -> 0x731620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::mod_row::mod_row<toolbar::menu::mods_mine_tab::list_slot::closure$1::closure_env$1> @ 0x2972c19 -> 0x731620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::impl$0::build
// 信息 ID: 45353061f15f073f2a29e296
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::impl$1::build
// 信息 ID: 4743a564e34f382bdff0e768
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
