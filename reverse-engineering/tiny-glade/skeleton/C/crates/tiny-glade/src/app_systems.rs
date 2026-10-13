// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/app_systems.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3d5b988f43b19473bc42fc23c4f401911a97e999a7c73ac75ccc0c8998140db4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: app_systems.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 77, 179, 200, 204
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
// use bevy_state; // namespace candidate
//   package identities: [{"name":"bevy_state","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::impl$0::add_wall_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":179,"record_offset":152028,"rva":5371232,"va":5374080352}
// 编译实例 RVAs: 0x51f560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$bevy_ecs..system..system..System$u2b$In$u20$$u3d$$u20$$LP$$RP$$u2b$Out$u20$$u3d$$u20$$LP$$RP$$GT$$u20$as$u20$bevy_ecs..schedule..config..Schedulable$GT$::into_config
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t10138349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e8edb29a00","size":5,"soft_fail":false,"target_va":5384218864,"va":5374080510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..schedule..config..ScheduleConfigs$LT$T$GT$$u20$as$u20$bevy_ecs..schedule..config..IntoScheduleConfigs$LT$T$C$$LP$$RP$$GT$$GT$::run_if
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-3501407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e8a192caff","size":5,"soft_fail":false,"target_va":5370579120,"va":5374080522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1598224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e810631800","size":5,"soft_fail":false,"target_va":5375678688,"va":5374080459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5358257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e84f3daeff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374080444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400793a0
//     owner=unresolved_namespace; count=8
//     target_aliases=118 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4876556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e8f496b5ff","size":5,"soft_fail":false,"target_va":5369205664,"va":5374082215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140095100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4762492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e88454b7ff","size":5,"soft_fail":false,"target_va":5369319680,"va":5374082167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ce80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4009033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e8b7d3c2ff","size":5,"soft_fail":false,"target_va":5370072704,"va":5374081732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401b9e10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3561377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e85fa8c9ff","size":5,"soft_fail":false,"target_va":5370519056,"va":5374080428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401be900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3543676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e884edc9ff","size":5,"soft_fail":false,"target_va":5370538240,"va":5374081911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd210
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3483353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e827d9caff","size":5,"soft_fail":false,"target_va":5370597904,"va":5374081252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401ce830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3478500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e81ceccaff","size":5,"soft_fail":false,"target_va":5370603568,"va":5374082063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cec80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3476722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e80ef3caff","size":5,"soft_fail":false,"target_va":5370604672,"va":5374081389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cf210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3475251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e8cdf8caff","size":5,"soft_fail":false,"target_va":5370606096,"va":5374081342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cfb40
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3471968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e8a005cbff","size":5,"soft_fail":false,"target_va":5370608448,"va":5374080411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cfe50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3472844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e83402cbff","size":5,"soft_fail":false,"target_va":5370609232,"va":5374082071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140245bb0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2989350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e8da62d2ff","size":5,"soft_fail":false,"target_va":5371091888,"va":5374081233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030e8c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2167464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e858eddeff","size":5,"soft_fail":false,"target_va":5371914432,"va":5374081891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035a990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1855077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e89bb1e3ff","size":5,"soft_fail":false,"target_va":5372225936,"va":5374081008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403a5e30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1546085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e89b68e8ff","size":5,"soft_fail":false,"target_va":5372534320,"va":5374080400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42976362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e86ac48f02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374080689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42538664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e8a8168902","size":5,"soft_fail":false,"target_va":5416620769,"va":5374082100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-82871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e849bcfeff","size":5,"soft_fail":false,"target_va":5373998512,"va":5374081378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3496664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e828a5caff","size":5,"soft_fail":false,"target_va":5370584336,"va":5374080995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::IntoScheduleConfigs::in_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1823205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374080352,"raw_hex":"e81b2ee4ff","size":5,"soft_fail":false,"target_va":5372258512,"va":5374081712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":179,"record_offset":152176,"parent_record_offset":152028,"ancestor_record_offset":152028,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":179,"record_offset":152232,"parent_record_offset":152028,"ancestor_record_offset":152028,"inlinee":4638,"source_line":448,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":179,"record_offset":153052,"parent_record_offset":152028,"ancestor_record_offset":152028,"inlinee":5917,"source_line":493,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/mod.rs
//     evidence={"module":179,"record_offset":153152,"parent_record_offset":152028,"ancestor_record_offset":152028,"inlinee":15452,"source_line":73,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":179,"record_offset":152388,"parent_record_offset":152364,"ancestor_record_offset":152028,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":179,"record_offset":152412,"parent_record_offset":152388,"ancestor_record_offset":152028,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":179,"record_offset":152436,"parent_record_offset":152412,"ancestor_record_offset":152028,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":179,"record_offset":152364,"parent_record_offset":152332,"ancestor_record_offset":152028,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":179,"record_offset":152332,"parent_record_offset":152296,"ancestor_record_offset":152028,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":179,"record_offset":152296,"parent_record_offset":152260,"ancestor_record_offset":152028,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::into_configs
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":179,"record_offset":152260,"parent_record_offset":152232,"ancestor_record_offset":152028,"inlinee":4259,"source_line":565,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/label.rs
//     evidence={"module":179,"record_offset":153092,"parent_record_offset":153052,"ancestor_record_offset":152028,"inlinee":4539,"source_line":119,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::function_system::impl$5::into_system
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs
//     evidence={"module":179,"record_offset":152820,"parent_record_offset":152784,"ancestor_record_offset":152028,"inlinee":5856,"source_line":595,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::impl$0::add_wall_systems::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 222
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2979
// 首条位置: {"module":77,"inlinee":10617}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":77,"record_offset":3472860,"parent_record_offset":3472832,"ancestor_record_offset":3472832,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::NextState::set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_state","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_state-0.17.3/src/state/resources.rs
//     evidence={"module":77,"record_offset":3472916,"parent_record_offset":3472832,"ancestor_record_offset":3472832,"inlinee":10613,"source_line":134,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":77,"record_offset":3472884,"parent_record_offset":3472860,"ancestor_record_offset":3472832,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::impl$0::add_wall_events
// 记录形式: compiled_procedure; instances=1
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":179,"record_offset":54844,"rva":5355856,"va":5374064976}
// 编译实例 RVAs: 0x51b950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t10577862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e8c667a100","size":5,"soft_fail":false,"target_va":5384644912,"va":5374067045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f3c0
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4849846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e84affb5ff","size":5,"soft_fail":false,"target_va":5369230272,"va":5374080113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087a30
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4815142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e8da86b6ff","size":5,"soft_fail":false,"target_va":5369264688,"va":5374079825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400915c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4775222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e8ca22b7ff","size":5,"soft_fail":false,"target_va":5369304512,"va":5374079729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140092dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4769654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e88a38b7ff","size":5,"soft_fail":false,"target_va":5369310656,"va":5374080305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140093b80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4765366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e84a49b7ff","size":5,"soft_fail":false,"target_va":5369314176,"va":5374079537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140093cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4765158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e81a4ab7ff","size":5,"soft_fail":false,"target_va":5369314480,"va":5374079633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140096550
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4755046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e89a71b7ff","size":5,"soft_fail":false,"target_va":5369324880,"va":5374079921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=2
//     target_aliases=34 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4410774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e86ab2bcff","size":5,"soft_fail":false,"target_va":5369660368,"va":5374071137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407eab40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2938906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e81ad82c00","size":5,"soft_fail":false,"target_va":5377010496,"va":5374071585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407f0ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2963430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e8e6372d00","size":5,"soft_fail":false,"target_va":5377034976,"va":5374071541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407f5900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2983408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e8f0852d00","size":5,"soft_fail":false,"target_va":5377054976,"va":5374071563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b8f4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6759551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e87f246700","size":5,"soft_fail":false,"target_va":5380830416,"va":5374070860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bf8650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7190554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e81ab86d00","size":5,"soft_fail":false,"target_va":5381260880,"va":5374070321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=35
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-213140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e86cbffcff","size":5,"soft_fail":false,"target_va":5373853600,"va":5374066735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::message::message_registry::MessageRegistry::register_message
//     owner=package_namespace_candidate; count=48
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1378838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e8160a1500","size":5,"soft_fail":false,"target_va":5375444112,"va":5374065269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7241107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e8937d6e00","size":5,"soft_fail":false,"target_va":5381309408,"va":5374068296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_render_slots::WallRenderSlots::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5576701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e8fd175500","size":5,"soft_fail":false,"target_va":5379645024,"va":5374068318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5926397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e8fd6d5a00","size":5,"soft_fail":false,"target_va":5379993152,"va":5374066750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=22
//     evidence={"assembly":"\tcallq\t10647841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e82179a200","size":5,"soft_fail":false,"target_va":5384726688,"va":5374078842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42539905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374064976,"raw_hex":"e8811b8902","size":5,"soft_fail":false,"target_va":5416619360,"va":5374079450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_message
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":179,"record_offset":55064,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":11549,"source_line":400,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::editing_highlight::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/editing_highlight.rs
//     evidence={"module":179,"record_offset":80764,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15436,"source_line":7,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":179,"record_offset":80736,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15434,"source_line":111,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::plaster::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/plaster.rs
//     evidence={"module":179,"record_offset":80016,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15432,"source_line":68,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":179,"record_offset":68464,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15412,"source_line":10,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":179,"record_offset":71352,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15416,"source_line":19,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":179,"record_offset":70632,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15414,"source_line":24,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::stone_foundation_cutouts::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/stone_foundation_cutouts.rs
//     evidence={"module":179,"record_offset":79296,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15430,"source_line":125,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::stone_foundation_surface::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/stone_foundation_surface.rs
//     evidence={"module":179,"record_offset":72072,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15418,"source_line":39,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_stone_columns::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_stone_columns.rs
//     evidence={"module":179,"record_offset":98996,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15448,"source_line":15,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::window_cuts::impl$31::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/window_cuts.rs
//     evidence={"module":179,"record_offset":73512,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15422,"source_line":418,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wooden_arch_frame::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wooden_arch_frame.rs
//     evidence={"module":179,"record_offset":72792,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15420,"source_line":55,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wooden_platform_stairs::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wooden_platform_stairs.rs
//     evidence={"module":179,"record_offset":76400,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15426,"source_line":29,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wooden_platform_stairs::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wooden_platform_stairs.rs
//     evidence={"module":179,"record_offset":77848,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15428,"source_line":35,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wooden_wall_boards::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wooden_wall_boards.rs
//     evidence={"module":179,"record_offset":74232,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15424,"source_line":38,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::terrain::deferred_terrain_history::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/deferred_terrain_history.rs
//     evidence={"module":179,"record_offset":97528,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15442,"source_line":10,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::deferred_wall_history::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/deferred_wall_history.rs
//     evidence={"module":179,"record_offset":97500,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15440,"source_line":14,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::draw_wall::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/draw_wall.rs
//     evidence={"module":179,"record_offset":97472,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15438,"source_line":39,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::finish_wall::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/finish_wall.rs
//     evidence={"module":179,"record_offset":66740,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15408,"source_line":19,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::ui_continue_wall::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/ui_continue_wall.rs
//     evidence={"module":179,"record_offset":68436,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15410,"source_line":51,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":179,"record_offset":97556,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15444,"source_line":56,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":179,"record_offset":98276,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15446,"source_line":60,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":179,"record_offset":100128,"parent_record_offset":54844,"ancestor_record_offset":54844,"inlinee":15450,"source_line":117,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":179,"record_offset":56460,"parent_record_offset":56436,"ancestor_record_offset":54844,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":179,"record_offset":56508,"parent_record_offset":56484,"ancestor_record_offset":54844,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":179,"record_offset":56484,"parent_record_offset":56460,"ancestor_record_offset":54844,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":179,"record_offset":56436,"parent_record_offset":56412,"ancestor_record_offset":54844,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":179,"record_offset":56412,"parent_record_offset":56372,"ancestor_record_offset":54844,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":179,"record_offset":56372,"parent_record_offset":56348,"ancestor_record_offset":54844,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_message
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/sub_app.rs
//     evidence={"module":179,"record_offset":55088,"parent_record_offset":55064,"ancestor_record_offset":54844,"inlinee":9392,"source_line":348,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::info::Components::get_valid_resource_id
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/info.rs
//     evidence={"module":179,"record_offset":55148,"parent_record_offset":55116,"ancestor_record_offset":54844,"inlinee":9045,"source_line":590,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":179,"record_offset":56680,"parent_record_offset":56656,"ancestor_record_offset":54844,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":179,"record_offset":56656,"parent_record_offset":56632,"ancestor_record_offset":54844,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":179,"record_offset":56120,"parent_record_offset":56096,"ancestor_record_offset":54844,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":179,"record_offset":56168,"parent_record_offset":56144,"ancestor_record_offset":54844,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":179,"record_offset":56144,"parent_record_offset":56120,"ancestor_record_offset":54844,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":179,"record_offset":56348,"parent_record_offset":56324,"ancestor_record_offset":54844,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":179,"record_offset":55116,"parent_record_offset":55088,"ancestor_record_offset":54844,"inlinee":9369,"source_line":1839,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":179,"record_offset":56096,"parent_record_offset":56072,"ancestor_record_offset":54844,"inlinee":9373,"source_line":1842,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_map.rs
//     evidence={"module":179,"record_offset":55172,"parent_record_offset":55148,"ancestor_record_offset":54844,"inlinee":4297,"source_line":938,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":179,"record_offset":55820,"parent_record_offset":55796,"ancestor_record_offset":54844,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":179,"record_offset":55848,"parent_record_offset":55820,"ancestor_record_offset":54844,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":179,"record_offset":69060,"parent_record_offset":69032,"ancestor_record_offset":54844,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":179,"record_offset":68960,"parent_record_offset":68756,"ancestor_record_offset":54844,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":179,"record_offset":100012,"parent_record_offset":99956,"ancestor_record_offset":54844,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":179,"record_offset":55388,"parent_record_offset":55364,"ancestor_record_offset":54844,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=96
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":179,"record_offset":55472,"parent_record_offset":55448,"ancestor_record_offset":54844,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=68
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":179,"record_offset":68924,"parent_record_offset":68900,"ancestor_record_offset":54844,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":179,"record_offset":69088,"parent_record_offset":69032,"ancestor_record_offset":54844,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":179,"record_offset":55620,"parent_record_offset":55592,"ancestor_record_offset":54844,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":179,"record_offset":55592,"parent_record_offset":55564,"ancestor_record_offset":54844,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=140
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":179,"record_offset":56260,"parent_record_offset":56196,"ancestor_record_offset":54844,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":179,"record_offset":56072,"parent_record_offset":55116,"ancestor_record_offset":54844,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":179,"record_offset":56284,"parent_record_offset":56260,"ancestor_record_offset":54844,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":179,"record_offset":56608,"parent_record_offset":55116,"ancestor_record_offset":54844,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":179,"record_offset":56324,"parent_record_offset":56144,"ancestor_record_offset":54844,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":179,"record_offset":68788,"parent_record_offset":68756,"ancestor_record_offset":54844,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":179,"record_offset":99928,"parent_record_offset":99900,"ancestor_record_offset":54844,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":179,"record_offset":55412,"parent_record_offset":55388,"ancestor_record_offset":54844,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":179,"record_offset":68876,"parent_record_offset":68756,"ancestor_record_offset":54844,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":179,"record_offset":55736,"parent_record_offset":55712,"ancestor_record_offset":54844,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":179,"record_offset":68900,"parent_record_offset":68876,"ancestor_record_offset":54844,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":179,"record_offset":56196,"parent_record_offset":56168,"ancestor_record_offset":54844,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":179,"record_offset":56232,"parent_record_offset":56196,"ancestor_record_offset":54844,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":179,"record_offset":56552,"parent_record_offset":56372,"ancestor_record_offset":54844,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$10::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":179,"record_offset":99756,"parent_record_offset":98996,"ancestor_record_offset":54844,"inlinee":11146,"source_line":286,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":179,"record_offset":55796,"parent_record_offset":55772,"ancestor_record_offset":54844,"inlinee":4329,"source_line":88,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":179,"record_offset":55532,"parent_record_offset":55504,"ancestor_record_offset":54844,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":179,"record_offset":55564,"parent_record_offset":55532,"ancestor_record_offset":54844,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":179,"record_offset":55504,"parent_record_offset":55284,"ancestor_record_offset":54844,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":179,"record_offset":55364,"parent_record_offset":55284,"ancestor_record_offset":54844,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":179,"record_offset":55924,"parent_record_offset":55284,"ancestor_record_offset":54844,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=96
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":179,"record_offset":55448,"parent_record_offset":55284,"ancestor_record_offset":54844,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":179,"record_offset":55208,"parent_record_offset":55172,"ancestor_record_offset":54844,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":179,"record_offset":55772,"parent_record_offset":55664,"ancestor_record_offset":54844,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":179,"record_offset":55712,"parent_record_offset":55688,"ancestor_record_offset":54844,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":179,"record_offset":56016,"parent_record_offset":55284,"ancestor_record_offset":54844,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":179,"record_offset":55688,"parent_record_offset":55664,"ancestor_record_offset":54844,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":179,"record_offset":55260,"parent_record_offset":55236,"ancestor_record_offset":54844,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":179,"record_offset":55236,"parent_record_offset":55208,"ancestor_record_offset":54844,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":179,"record_offset":55284,"parent_record_offset":55260,"ancestor_record_offset":54844,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":179,"record_offset":55336,"parent_record_offset":55284,"ancestor_record_offset":54844,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":179,"record_offset":55664,"parent_record_offset":55284,"ancestor_record_offset":54844,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::util::likely
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/util.rs
//     evidence={"module":179,"record_offset":55896,"parent_record_offset":55284,"ancestor_record_offset":54844,"inlinee":4335,"source_line":12,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":179,"record_offset":69200,"parent_record_offset":68524,"ancestor_record_offset":54844,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":179,"record_offset":68524,"parent_record_offset":68464,"ancestor_record_offset":54844,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":179,"record_offset":68580,"parent_record_offset":68552,"ancestor_record_offset":54844,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":179,"record_offset":68836,"parent_record_offset":68812,"ancestor_record_offset":54844,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":179,"record_offset":68696,"parent_record_offset":68668,"ancestor_record_offset":54844,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":179,"record_offset":69032,"parent_record_offset":68636,"ancestor_record_offset":54844,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":179,"record_offset":68552,"parent_record_offset":68524,"ancestor_record_offset":54844,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":179,"record_offset":68724,"parent_record_offset":68696,"ancestor_record_offset":54844,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":179,"record_offset":68756,"parent_record_offset":68724,"ancestor_record_offset":54844,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":179,"record_offset":68636,"parent_record_offset":68608,"ancestor_record_offset":54844,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":179,"record_offset":68608,"parent_record_offset":68580,"ancestor_record_offset":54844,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00179.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::app_systems::walls_on_glade_unload
// 记录形式: compiled_procedure; instances=1
// 源码行: 325
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":204,"record_offset":536,"rva":5723968,"va":5374433088}
// 编译实例 RVAs: 0x575740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=18
//     evidence={"assembly":"\tcallq\t10211506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8b2d09b00","size":5,"soft_fail":false,"target_va":5384644912,"va":5374433401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t1244783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e86ffe1200","size":5,"soft_fail":false,"target_va":5375678704,"va":5374433916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007dcf0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5210986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8967cb0ff","size":5,"soft_fail":false,"target_va":5369224432,"va":5374435413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f3c0
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5208642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8be85b0ff","size":5,"soft_fail":false,"target_va":5369230272,"va":5374438909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087a30
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5173602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e89e0eb1ff","size":5,"soft_fail":false,"target_va":5369264688,"va":5374438285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400915c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5133695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e881aab1ff","size":5,"soft_fail":false,"target_va":5369304512,"va":5374438202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140092dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5128994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8debcb1ff","size":5,"soft_fail":false,"target_va":5369310656,"va":5374439645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140093cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5123442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e88ed2b1ff","size":5,"soft_fail":false,"target_va":5369314480,"va":5374437917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140096550
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5113679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8b1f8b1ff","size":5,"soft_fail":false,"target_va":5369324880,"va":5374438554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b32a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4993350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8baceb3ff","size":5,"soft_fail":false,"target_va":5369442976,"va":5374436321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ba980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4960290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8de4fb4ff","size":5,"soft_fail":false,"target_va":5369473408,"va":5374433693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c5670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4918874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8a6f1b4ff","size":5,"soft_fail":false,"target_va":5369517680,"va":5374436549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c5f30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4915198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e80200b5ff","size":5,"soft_fail":false,"target_va":5369519920,"va":5374435113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4775822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e87220b7ff","size":5,"soft_fail":false,"target_va":5369660368,"va":5374436185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ca170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1753383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8d93ee5ff","size":5,"soft_fail":false,"target_va":5372682608,"va":5374435986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cce80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1741706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8766ce5ff","size":5,"soft_fail":false,"target_va":5372694144,"va":5374435845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ab0970
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5481674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8caa45300","size":5,"soft_fail":false,"target_va":5379918192,"va":5374436513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ba15e0
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: app_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6467753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8a9b06200","size":5,"soft_fail":false,"target_va":5380904416,"va":5374436658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::color_id::StoneFoundationColorIds::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6467712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e880b06200","size":5,"soft_fail":false,"target_va":5380904880,"va":5374437163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove_all
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6864582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8c6be6800","size":5,"soft_fail":false,"target_va":5381301072,"va":5374436485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::reset_max_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6865339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8bbc16800","size":5,"soft_fail":false,"target_va":5381301840,"va":5374436496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_render_slots::WallRenderSlots::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5208757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8b57a4f00","size":5,"soft_fail":false,"target_va":5379645408,"va":5374436646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5556622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e88ec95400","size":5,"soft_fail":false,"target_va":5379993152,"va":5374436525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t10289383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e8e7009d00","size":5,"soft_fail":false,"target_va":5384726688,"va":5374437300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42181499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374433088,"raw_hex":"e87ba38302","size":5,"soft_fail":false,"target_va":5416619360,"va":5374437856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":204,"record_offset":2888,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":3720,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":204,"record_offset":716,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15412,"source_line":10,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":204,"record_offset":4240,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15416,"source_line":19,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":204,"record_offset":2948,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15414,"source_line":24,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::stone_foundation_cutouts::impl$13::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/stone_foundation_cutouts.rs
//     evidence={"module":204,"record_offset":16316,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15430,"source_line":125,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::stone_foundation_surface::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/stone_foundation_surface.rs
//     evidence={"module":204,"record_offset":6528,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15418,"source_line":39,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::window_cuts::impl$31::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/window_cuts.rs
//     evidence={"module":204,"record_offset":17228,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15422,"source_line":418,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wooden_arch_frame::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wooden_arch_frame.rs
//     evidence={"module":204,"record_offset":8808,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15420,"source_line":55,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wooden_platform_stairs::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wooden_platform_stairs.rs
//     evidence={"module":204,"record_offset":13308,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15426,"source_line":29,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wooden_platform_stairs::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wooden_platform_stairs.rs
//     evidence={"module":204,"record_offset":14812,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15428,"source_line":35,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wooden_wall_boards::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wooden_wall_boards.rs
//     evidence={"module":204,"record_offset":11084,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15424,"source_line":38,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/lib.rs
//     evidence={"module":204,"record_offset":18200,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15450,"source_line":117,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":4112,"parent_record_offset":4084,"ancestor_record_offset":536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":4140,"parent_record_offset":4112,"ancestor_record_offset":536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":4172,"parent_record_offset":4140,"ancestor_record_offset":536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":4084,"parent_record_offset":3896,"ancestor_record_offset":536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":204,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":204,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":204,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":204,"record_offset":2916,"parent_record_offset":2888,"ancestor_record_offset":536,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":204,"record_offset":1316,"parent_record_offset":1288,"ancestor_record_offset":536,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":204,"record_offset":1216,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":204,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":536,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":204,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":536,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":204,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":536,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":204,"record_offset":4016,"parent_record_offset":3988,"ancestor_record_offset":536,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":204,"record_offset":6004,"parent_record_offset":5972,"ancestor_record_offset":536,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":204,"record_offset":1344,"parent_record_offset":1288,"ancestor_record_offset":536,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":204,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":536,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":204,"record_offset":924,"parent_record_offset":888,"ancestor_record_offset":536,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":204,"record_offset":1044,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":204,"record_offset":5368,"parent_record_offset":5284,"ancestor_record_offset":536,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":204,"record_offset":19852,"parent_record_offset":19824,"ancestor_record_offset":536,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=47
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":204,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":536,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":204,"record_offset":1132,"parent_record_offset":1012,"ancestor_record_offset":536,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":204,"record_offset":4052,"parent_record_offset":3928,"ancestor_record_offset":536,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":1156,"parent_record_offset":1132,"ancestor_record_offset":536,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":204,"record_offset":19680,"parent_record_offset":18200,"ancestor_record_offset":536,"inlinee":11146,"source_line":286,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":204,"record_offset":5500,"parent_record_offset":5472,"ancestor_record_offset":536,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":204,"record_offset":5532,"parent_record_offset":5500,"ancestor_record_offset":536,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":204,"record_offset":5472,"parent_record_offset":5432,"ancestor_record_offset":536,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":204,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":536,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":5800,"parent_record_offset":5196,"ancestor_record_offset":536,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":5652,"parent_record_offset":5432,"ancestor_record_offset":536,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":5284,"parent_record_offset":5228,"ancestor_record_offset":536,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":5432,"parent_record_offset":5404,"ancestor_record_offset":536,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":3928,"parent_record_offset":3896,"ancestor_record_offset":536,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":536,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":5196,"parent_record_offset":5136,"ancestor_record_offset":536,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":536,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":3896,"parent_record_offset":3840,"ancestor_record_offset":536,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":3868,"parent_record_offset":3840,"ancestor_record_offset":536,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":5228,"parent_record_offset":5196,"ancestor_record_offset":536,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":3960,"parent_record_offset":3928,"ancestor_record_offset":536,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":536,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":204,"record_offset":5404,"parent_record_offset":5196,"ancestor_record_offset":536,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":204,"record_offset":1456,"parent_record_offset":776,"ancestor_record_offset":536,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":204,"record_offset":776,"parent_record_offset":716,"ancestor_record_offset":536,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":204,"record_offset":832,"parent_record_offset":804,"ancestor_record_offset":536,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":204,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":536,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":204,"record_offset":952,"parent_record_offset":924,"ancestor_record_offset":536,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":204,"record_offset":1288,"parent_record_offset":888,"ancestor_record_offset":536,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":204,"record_offset":804,"parent_record_offset":776,"ancestor_record_offset":536,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":204,"record_offset":980,"parent_record_offset":952,"ancestor_record_offset":536,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":204,"record_offset":1012,"parent_record_offset":980,"ancestor_record_offset":536,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":204,"record_offset":888,"parent_record_offset":860,"ancestor_record_offset":536,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":204,"record_offset":860,"parent_record_offset":832,"ancestor_record_offset":536,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00204.jsonl.gz"}
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
// 完整结构及原始类型记录: app_systems.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: app_systems.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: app_systems.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 235; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_wall_boards::WoodenFreeformWallBoards>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenGateStepDrawables> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutBuffers>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutGpuBuffers>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_wall_boards::WoodenFreeformWallBoards>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenGateStepDrawables> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutBuffers>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutGpuBuffers>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_wall_boards::WoodenFreeformWallBoards>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenGateStepDrawables>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutBuffers>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutGpuBuffers>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::temp_eraser::TempEraser>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::temp_eraser::TempEraser>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::temp_eraser::TempEraser>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::PrevAppMode>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ModeJustChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::PrevAppMode>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ModeJustChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::PrevAppMode>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ModeJustChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::impl$0::add_wall_events
// 信息 ID: cb2359a471e37b262fc7afd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41ddb2 -> 0x51b950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::impl$0::add_wall_systems
// 信息 ID: cbf3fb8fc9f0bbb9eda4b804
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41ddfa -> 0x51f560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::impl$0::add_wall_systems::closure$0
// 信息 ID: 0890a2527bc33d6ad1f7102a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2244e3aa9ca98f1554fc8390: void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)
// [泛型宿主/闭包/直接引用方参数候选] c633f28c6e030952427defed: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::app_systems::impl$0::add_wall_systems", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::schedule::schedule::Schedules::add_systems<tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)> >,bevy_app::main_schedule::PostUpdate,tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)> >,bevy_app::main_schedule::PostUpdate,tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::system::commands::Commands),tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tuple$<>,tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tuple$<>,tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tuple$<>,tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tuple$<>,tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tuple$<>,tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0 <- bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tuple$<>,tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 26 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tuple$<>,tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0> >; module=77, inline_record=3472832; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >),tuple$<>,tiny_glade::app_systems::impl$0::add_wall_systems::closure_env$0>; module=200, inline_record=38528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::app_systems::walls_on_glade_unload
// 信息 ID: 995d244229034af30618a961
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2da46d11cc93f410a5e5feba: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_wall_boards::WoodenFreeformWallBoards>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenGateStepDrawables> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutBuffers>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutGpuBuffers>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_wall_boards::WoodenFreeformWallBoards>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenGateStepDrawables> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutBuffers>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutGpuBuffers>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_wall_boards::WoodenFreeformWallBoards>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenGateStepDrawables> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutBuffers>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutGpuBuffers>)> > @ 0x27ad8f -> 0x575740 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_wall_boards::WoodenFreeformWallBoards>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenGateStepDrawables> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutBuffers>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutGpuBuffers>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_wall_boards::WoodenFreeformWallBoards>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenGateStepDrawables> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_cutouts::StoneFoundationCutoutBuffers>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::gates::Gates>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::window_cuts::WindowCutGpuBuffers>)> @ 0x568f26 -> 0x575740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
