// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/backend/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 62288b050a91e35fae032aaa2690346900db8eb5f43f3299ab370f57b3b51a41
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 511
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
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
// 函数符号: country_core::systems::backend::impl$0::add_backend_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":185944,"rva":9733728,"va":5378442848}
// 编译实例 RVAs: 0x948660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$bevy_ecs..system..system..System$u2b$In$u20$$u3d$$u20$$LP$$RP$$u2b$Out$u20$$u3d$$u20$$LP$$RP$$GT$$u20$as$u20$bevy_ecs..schedule..config..Schedulable$GT$::into_config
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5775571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8d3205800","size":5,"soft_fail":false,"target_va":5384218864,"va":5378443288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2764560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8f0d0d5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378443243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9721041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e82fab6bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378443228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=2
//     target_aliases=118 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9237919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8610a73ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5378443578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7845040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8504b88ff","size":5,"soft_fail":false,"target_va":5370597904,"va":5378442939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14091c630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-180319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8a13ffdff","size":5,"soft_fail":false,"target_va":5378262576,"va":5378442890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14091c9b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-179876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e85c41fdff","size":5,"soft_fail":false,"target_va":5378263472,"va":5378443343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409279d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-134930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8eef0fdff","size":5,"soft_fail":false,"target_va":5378308560,"va":5378443485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t57008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8b0de0000","size":5,"soft_fail":false,"target_va":5378499936,"va":5378442923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd0440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2653402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8da7c2800","size":5,"soft_fail":false,"target_va":5381096512,"va":5378443105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c7bf40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3356710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e826383300","size":5,"soft_fail":false,"target_va":5381799744,"va":5378443029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14151ae90
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12396427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e88b27bd00","size":5,"soft_fail":false,"target_va":5390839440,"va":5378443008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417e48e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15319220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8b4c0e900","size":5,"soft_fail":false,"target_va":5393762528,"va":5378443303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417eb200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15346071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e89729ea00","size":5,"soft_fail":false,"target_va":5393789440,"va":5378443364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417eb500
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15346879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8bf2cea00","size":5,"soft_fail":false,"target_va":5393790208,"va":5378443324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14181c840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15548682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e80a41ed00","size":5,"soft_fail":false,"target_va":5393991744,"va":5378443057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419e2420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17407182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8ce9c0901","size":5,"soft_fail":false,"target_va":5395850272,"va":5378443085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38177236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378442848,"raw_hex":"e8d4894602","size":5,"soft_fail":false,"target_va":5416620769,"va":5378443528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":511,"record_offset":186168,"parent_record_offset":185944,"ancestor_record_offset":185944,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":511,"record_offset":186380,"parent_record_offset":185944,"ancestor_record_offset":185944,"inlinee":4638,"source_line":448,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":511,"record_offset":186096,"parent_record_offset":185944,"ancestor_record_offset":185944,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":186536,"parent_record_offset":186512,"ancestor_record_offset":185944,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":186560,"parent_record_offset":186536,"ancestor_record_offset":185944,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":186584,"parent_record_offset":186560,"ancestor_record_offset":185944,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":186512,"parent_record_offset":186480,"ancestor_record_offset":185944,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":186480,"parent_record_offset":186444,"ancestor_record_offset":185944,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":186444,"parent_record_offset":186408,"ancestor_record_offset":185944,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::into_configs
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":511,"record_offset":186408,"parent_record_offset":186380,"ancestor_record_offset":185944,"inlinee":4259,"source_line":565,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":511,"record_offset":186136,"parent_record_offset":186096,"ancestor_record_offset":185944,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::impl$0::insert_backend_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":186996,"rva":9734528,"va":5378443648}
// 编译实例 RVAs: 0x948980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2765692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e884ccd5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378444391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416076f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13364315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e85beccb00","size":5,"soft_fail":false,"target_va":5391808240,"va":5378443920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141852d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15769736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e888a0f000","size":5,"soft_fail":false,"target_va":5394214144,"va":5378444403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141854ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15778299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e8fbc1f000","size":5,"soft_fail":false,"target_va":5394222832,"va":5378444528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14185c280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15807394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e8a233f100","size":5,"soft_fail":false,"target_va":5394252416,"va":5378445017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14185f930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15821362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e8326af100","size":5,"soft_fail":false,"target_va":5394266416,"va":5378445049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141866510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15849000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e828d6f100","size":5,"soft_fail":false,"target_va":5394294032,"va":5378445027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38612534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e8362e4d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5378444517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::input_stream::InputRecordReplayState::try_load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1117441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e8010d1100","size":5,"soft_fail":false,"target_va":5379561728,"va":5378444282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::toolbar::Toolbar::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2109933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e8ed312000","size":5,"soft_fail":false,"target_va":5380553712,"va":5378443774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::backend::impl$0::insert_backend_resources::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t37937536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e880e14202","size":5,"soft_fail":false,"target_va":5416382624,"va":5378445083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::store_parameters
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t874801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378443648,"raw_hex":"e831590d00","size":5,"soft_fail":false,"target_va":5379318560,"va":5378443754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":188056,"parent_record_offset":186996,"ancestor_record_offset":186996,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":511,"record_offset":188020,"parent_record_offset":186996,"ancestor_record_offset":186996,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::impl$29::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":511,"record_offset":188544,"parent_record_offset":186996,"ancestor_record_offset":186996,"inlinee":13697,"source_line":293,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::impl$35::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":511,"record_offset":188424,"parent_record_offset":186996,"ancestor_record_offset":186996,"inlinee":29506,"source_line":308,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputRecordReplayState::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":511,"record_offset":187640,"parent_record_offset":186996,"ancestor_record_offset":186996,"inlinee":29504,"source_line":224,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputReplayInitialState::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":511,"record_offset":187172,"parent_record_offset":186996,"ancestor_record_offset":186996,"inlinee":29500,"source_line":62,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputStream::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":511,"record_offset":187232,"parent_record_offset":186996,"ancestor_record_offset":186996,"inlinee":29502,"source_line":108,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::backend::icc::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/backend/icc.rs
//     evidence={"module":511,"record_offset":188604,"parent_record_offset":186996,"ancestor_record_offset":186996,"inlinee":29510,"source_line":13,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":188304,"parent_record_offset":188280,"ancestor_record_offset":186996,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":188328,"parent_record_offset":188304,"ancestor_record_offset":186996,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":188352,"parent_record_offset":188328,"ancestor_record_offset":186996,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":188280,"parent_record_offset":188224,"ancestor_record_offset":186996,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":511,"record_offset":187544,"parent_record_offset":187520,"ancestor_record_offset":186996,"inlinee":11703,"source_line":781,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":511,"record_offset":187284,"parent_record_offset":187260,"ancestor_record_offset":186996,"inlinee":12130,"source_line":3641,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$23::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":511,"record_offset":187392,"parent_record_offset":187360,"ancestor_record_offset":186996,"inlinee":12134,"source_line":3746,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":511,"record_offset":187520,"parent_record_offset":187232,"ancestor_record_offset":186996,"inlinee":12059,"source_line":165,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::spec_from_iter::impl$0::spec_from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\spec_from_iter.rs
//     evidence={"module":511,"record_offset":187336,"parent_record_offset":187284,"ancestor_record_offset":186996,"inlinee":12132,"source_line":12,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":188252,"parent_record_offset":188224,"ancestor_record_offset":186996,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":188224,"parent_record_offset":188200,"ancestor_record_offset":186996,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":188200,"parent_record_offset":188176,"ancestor_record_offset":186996,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":187416,"parent_record_offset":187392,"ancestor_record_offset":186996,"inlinee":12135,"source_line":1339,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":187360,"parent_record_offset":187336,"ancestor_record_offset":186996,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":511,"record_offset":187308,"parent_record_offset":187284,"ancestor_record_offset":186996,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":511,"record_offset":187260,"parent_record_offset":187232,"ancestor_record_offset":186996,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":511,"record_offset":187440,"parent_record_offset":187416,"ancestor_record_offset":186996,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":511,"record_offset":187464,"parent_record_offset":187440,"ancestor_record_offset":186996,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":188080,"parent_record_offset":188056,"ancestor_record_offset":186996,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::action_set::impl$33::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":511,"record_offset":188452,"parent_record_offset":188424,"ancestor_record_offset":186996,"inlinee":29508,"source_line":301,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::action_set::impl$3::default
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":511,"record_offset":188480,"parent_record_offset":188452,"ancestor_record_offset":186996,"inlinee":13699,"source_line":17,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::input_stream::InputStream::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":511,"record_offset":187668,"parent_record_offset":187640,"ancestor_record_offset":186996,"inlinee":29502,"source_line":108,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::mode_manager::impl$3::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/mode_manager.rs
//     evidence={"module":511,"record_offset":187200,"parent_record_offset":187172,"ancestor_record_offset":186996,"inlinee":29449,"source_line":352,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::impl$0::insert_backend_resources::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 198
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":189372,"rva":47673504,"va":5416382624}
// 编译实例 RVAs: 0x2d770a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-31758663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416382624,"raw_hex":"e8b9661bfe","size":5,"soft_fail":false,"target_va":5384624080,"va":5416382738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t239537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416382624,"raw_hex":"e8b1a70300","size":5,"soft_fail":false,"target_va":5416622240,"va":5416382698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":189532,"parent_record_offset":189372,"ancestor_record_offset":189372,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::impl$0::add_backend_events
// 记录形式: compiled_procedure; instances=1
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":174124,"rva":9732016,"va":5378441136}
// 编译实例 RVAs: 0x947fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15607990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378441136,"raw_hex":"e8b628ee00","size":5,"soft_fail":false,"target_va":5394049376,"va":5378441381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":511,"record_offset":174268,"parent_record_offset":174124,"ancestor_record_offset":174124,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":175656,"parent_record_offset":175632,"ancestor_record_offset":174124,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":175704,"parent_record_offset":175680,"ancestor_record_offset":174124,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":511,"record_offset":175680,"parent_record_offset":175656,"ancestor_record_offset":174124,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":175632,"parent_record_offset":175608,"ancestor_record_offset":174124,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":175608,"parent_record_offset":175568,"ancestor_record_offset":174124,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":175568,"parent_record_offset":175544,"ancestor_record_offset":174124,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":511,"record_offset":174292,"parent_record_offset":174268,"ancestor_record_offset":174124,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":511,"record_offset":174352,"parent_record_offset":174320,"ancestor_record_offset":174124,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":511,"record_offset":175876,"parent_record_offset":175852,"ancestor_record_offset":174124,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":511,"record_offset":175852,"parent_record_offset":175828,"ancestor_record_offset":174124,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":511,"record_offset":175316,"parent_record_offset":175292,"ancestor_record_offset":174124,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":175364,"parent_record_offset":175340,"ancestor_record_offset":174124,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":175340,"parent_record_offset":175316,"ancestor_record_offset":174124,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":175544,"parent_record_offset":175520,"ancestor_record_offset":174124,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":511,"record_offset":174320,"parent_record_offset":174292,"ancestor_record_offset":174124,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":511,"record_offset":175292,"parent_record_offset":175268,"ancestor_record_offset":174124,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":511,"record_offset":174376,"parent_record_offset":174352,"ancestor_record_offset":174124,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":511,"record_offset":175016,"parent_record_offset":174992,"ancestor_record_offset":174124,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":511,"record_offset":175044,"parent_record_offset":175016,"ancestor_record_offset":174124,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":511,"record_offset":174592,"parent_record_offset":174568,"ancestor_record_offset":174124,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":511,"record_offset":174676,"parent_record_offset":174652,"ancestor_record_offset":174124,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":511,"record_offset":174824,"parent_record_offset":174796,"ancestor_record_offset":174124,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":511,"record_offset":174796,"parent_record_offset":174768,"ancestor_record_offset":174124,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":511,"record_offset":175456,"parent_record_offset":175392,"ancestor_record_offset":174124,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":175268,"parent_record_offset":174320,"ancestor_record_offset":174124,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":175480,"parent_record_offset":175456,"ancestor_record_offset":174124,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":175804,"parent_record_offset":174320,"ancestor_record_offset":174124,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":511,"record_offset":175520,"parent_record_offset":175340,"ancestor_record_offset":174124,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":174616,"parent_record_offset":174592,"ancestor_record_offset":174124,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":511,"record_offset":174936,"parent_record_offset":174912,"ancestor_record_offset":174124,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":511,"record_offset":175392,"parent_record_offset":175364,"ancestor_record_offset":174124,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":511,"record_offset":175428,"parent_record_offset":175392,"ancestor_record_offset":174124,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":511,"record_offset":175748,"parent_record_offset":175568,"ancestor_record_offset":174124,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":511,"record_offset":174992,"parent_record_offset":174972,"ancestor_record_offset":174124,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":174736,"parent_record_offset":174708,"ancestor_record_offset":174124,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":174768,"parent_record_offset":174736,"ancestor_record_offset":174124,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":511,"record_offset":174708,"parent_record_offset":174488,"ancestor_record_offset":174124,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":174568,"parent_record_offset":174488,"ancestor_record_offset":174124,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":175120,"parent_record_offset":174488,"ancestor_record_offset":174124,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":511,"record_offset":174652,"parent_record_offset":174488,"ancestor_record_offset":174124,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":511,"record_offset":174412,"parent_record_offset":174376,"ancestor_record_offset":174124,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":511,"record_offset":174972,"parent_record_offset":174868,"ancestor_record_offset":174124,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":174912,"parent_record_offset":174888,"ancestor_record_offset":174124,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":175212,"parent_record_offset":174488,"ancestor_record_offset":174124,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":174888,"parent_record_offset":174868,"ancestor_record_offset":174124,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":174464,"parent_record_offset":174440,"ancestor_record_offset":174124,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":174440,"parent_record_offset":174412,"ancestor_record_offset":174124,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":174488,"parent_record_offset":174464,"ancestor_record_offset":174124,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":174540,"parent_record_offset":174488,"ancestor_record_offset":174124,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":511,"record_offset":174868,"parent_record_offset":174488,"ancestor_record_offset":174124,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":511,"record_offset":175092,"parent_record_offset":174488,"ancestor_record_offset":174124,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00511.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::backend::TimedFrameEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::messages::Messages<country_core::systems::backend::TimedFrameEvent>", "constructor": "bevy_ecs::message::messages::Messages", "arguments": ["country_core::systems::backend::TimedFrameEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::message::messages::Messages<country_core::systems::backend::TimedFrameEvent> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::message::messages::Messages<country_core::systems::backend::TimedFrameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::message_registry::impl$0::register_message::closure_env$0<country_core::systems::backend::TimedFrameEvent>", "constructor": "bevy_ecs::message::message_registry::impl$0::register_message::closure_env$0", "arguments": ["country_core::systems::backend::TimedFrameEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::message::message_registry::impl$0::register_message::closure_env$0<country_core::systems::backend::TimedFrameEvent>,tuple$<bevy_ecs::change_detection::MutUntyped> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::message::message_registry::impl$0::register_message::closure_env$0<country_core::systems::backend::TimedFrameEvent>", "tuple$<bevy_ecs::change_detection::MutUntyped>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::message_registry::MessageRegistry::register_message<country_core::systems::backend::TimedFrameEvent>", "constructor": "bevy_ecs::message::message_registry::MessageRegistry::register_message", "arguments": ["country_core::systems::backend::TimedFrameEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr<bevy_ecs::message::messages::Messages<country_core::systems::backend::TimedFrameEvent> >", "constructor": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr", "arguments": ["bevy_ecs::message::messages::Messages<country_core::systems::backend::TimedFrameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 20; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::PlantDrawEvent>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenDrawEvent>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenEraseEvent>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::PlantDrawEvent>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenDrawEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::GardenEraseEvent>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::garden::PlantDrawEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>", "bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::impl$0::add_backend_events
// 信息 ID: 4622e7c2f707c62ed7b8e220
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41df69 -> 0x947fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::impl$0::add_backend_systems
// 信息 ID: d6df2e27407ac2057d0a1ccf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41df21 -> 0x948660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::impl$0::insert_backend_resources
// 信息 ID: 00b5ecac60f359c5fbd191fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41da3d -> 0x948980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::impl$0::insert_backend_resources::closure$0
// 信息 ID: bba4cf6744f11567c4c682ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::backend::impl$0::insert_backend_resources", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::impl$0::insert_backend_resources @ 0x948f1b -> 0x2d770a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
