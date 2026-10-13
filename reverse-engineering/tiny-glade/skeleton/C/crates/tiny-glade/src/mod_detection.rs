// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/mod_detection.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e5e4a139f680d9202bac681c52a76a863608869b98993be79d40ee1ca8f0fa05
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod_detection.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 186, 230
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::mod_detection::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":186,"record_offset":75180,"rva":5472896,"va":5374182016}
// 编译实例 RVAs: 0x538280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10466280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e8e8b39f00","size":5,"soft_fail":false,"target_va":5384648448,"va":5374182163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10465959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e8a7b29f00","size":5,"soft_fail":false,"target_va":5384648032,"va":5374182068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..boxed..Box$LT$dyn$u20$bevy_ecs..system..system..System$u2b$In$u20$$u3d$$u20$$LP$$RP$$u2b$Out$u20$$u3d$$u20$$LP$$RP$$GT$$u20$as$u20$bevy_ecs..schedule..config..Schedulable$GT$::into_config
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10036282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e83a249900","size":5,"soft_fail":false,"target_va":5384218864,"va":5374182577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..schedule..config..ScheduleConfigs$LT$T$GT$$u20$as$u20$bevy_ecs..schedule..config..IntoScheduleConfigs$LT$T$C$$LP$$RP$$GT$$GT$::run_if
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3605397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e86bfcc8ff","size":5,"soft_fail":false,"target_va":5370577200,"va":5374182592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10427052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e8ac1a9f00","size":5,"soft_fail":false,"target_va":5384609408,"va":5374182351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1496264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e8c8d41600","size":5,"soft_fail":false,"target_va":5375678688,"va":5374182419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: mod_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5460217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e807afacff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374182404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b05c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4751250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e86e80b7ff","size":5,"soft_fail":false,"target_va":5369431488,"va":5374182733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cfb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3574160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e87076c9ff","size":5,"soft_fail":false,"target_va":5370608448,"va":5374182603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404378e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1051818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e856f3efff","size":5,"soft_fail":false,"target_va":5373130976,"va":5374182789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42438133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e8f58d8702","size":5,"soft_fail":false,"target_va":5416620769,"va":5374182631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-361033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e8b77dfaff","size":5,"soft_fail":false,"target_va":5373821488,"va":5374182516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::discover_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8747190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e8b6788500","size":5,"soft_fail":false,"target_va":5382929296,"va":5374182101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::init_mod_catalog
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8724797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374182016,"raw_hex":"e83d218500","size":5,"soft_fail":false,"target_va":5382907200,"va":5374182398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":186,"record_offset":76684,"parent_record_offset":75180,"ancestor_record_offset":75180,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":186,"record_offset":76420,"parent_record_offset":75180,"ancestor_record_offset":75180,"inlinee":4638,"source_line":448,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::mod_status::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/mod_status.rs
//     evidence={"module":186,"record_offset":75660,"parent_record_offset":75180,"ancestor_record_offset":75180,"inlinee":15632,"source_line":17,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::mod_detection::init_mods
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/mod_detection.rs
//     evidence={"module":186,"record_offset":75316,"parent_record_offset":75180,"ancestor_record_offset":75180,"inlinee":15630,"source_line":46,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":75824,"parent_record_offset":75800,"ancestor_record_offset":75180,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":75848,"parent_record_offset":75824,"ancestor_record_offset":75180,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":75872,"parent_record_offset":75848,"ancestor_record_offset":75180,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":75800,"parent_record_offset":75768,"ancestor_record_offset":75180,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":186,"record_offset":75768,"parent_record_offset":75740,"ancestor_record_offset":75180,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":186,"record_offset":76484,"parent_record_offset":76448,"ancestor_record_offset":75180,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":186,"record_offset":75740,"parent_record_offset":75704,"ancestor_record_offset":75180,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$6::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":186,"record_offset":75924,"parent_record_offset":75704,"ancestor_record_offset":75180,"inlinee":7479,"source_line":1212,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":76288,"parent_record_offset":76264,"ancestor_record_offset":75180,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$62::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":75704,"parent_record_offset":75660,"ancestor_record_offset":75180,"inlinee":7477,"source_line":3726,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":75376,"parent_record_offset":75316,"ancestor_record_offset":75180,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::config::impl$4::into_configs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/config.rs
//     evidence={"module":186,"record_offset":76448,"parent_record_offset":76420,"ancestor_record_offset":75180,"inlinee":4259,"source_line":565,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":186,"record_offset":75488,"parent_record_offset":75316,"ancestor_record_offset":75180,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":186,"record_offset":75948,"parent_record_offset":75924,"ancestor_record_offset":75180,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":186,"record_offset":75512,"parent_record_offset":75488,"ancestor_record_offset":75180,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":76264,"parent_record_offset":75660,"ancestor_record_offset":75180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":76324,"parent_record_offset":76288,"ancestor_record_offset":75180,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":75428,"parent_record_offset":75404,"ancestor_record_offset":75180,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":75452,"parent_record_offset":75428,"ancestor_record_offset":75180,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":76348,"parent_record_offset":76324,"ancestor_record_offset":75180,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":76380,"parent_record_offset":76288,"ancestor_record_offset":75180,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":186,"record_offset":75592,"parent_record_offset":75568,"ancestor_record_offset":75180,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":186,"record_offset":75544,"parent_record_offset":75316,"ancestor_record_offset":75180,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":186,"record_offset":75568,"parent_record_offset":75544,"ancestor_record_offset":75180,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":186,"record_offset":75616,"parent_record_offset":75592,"ancestor_record_offset":75180,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":186,"record_offset":75404,"parent_record_offset":75316,"ancestor_record_offset":75180,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::mod_detection::spawn_package_integrity_check_after_reveal
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":186,"record_offset":3132,"rva":5465552,"va":5374174672}
// 编译实例 RVAs: 0x5365d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::lifecycle::Packet<()>>>::drop_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10528492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e8eca6a000","size":5,"soft_fail":false,"target_va":5384703584,"va":5374175087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10528836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e844a8a000","size":5,"soft_fail":false,"target_va":5384703904,"va":5374175063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::thread::builder::Builder>::name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10474348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e86cd39f00","size":5,"soft_fail":false,"target_va":5384649296,"va":5374174943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1503804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e83cf21600","size":5,"soft_fail":false,"target_va":5375678688,"va":5374174879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1503406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e8aef01600","size":5,"soft_fail":false,"target_va":5375678704,"va":5374175293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mod_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5452677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e87bccacff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374174864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4836560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e83033b6ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374175403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: mod_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4733114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e846c7b7ff","size":5,"soft_fail":false,"target_va":5369442256,"va":5374175365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42445675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e86bab8702","size":5,"soft_fail":false,"target_va":5416620794,"va":5374175114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42446466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e882ae8702","size":5,"soft_fail":false,"target_va":5416621632,"va":5374175161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::spawn_unchecked
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-824681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374174672,"raw_hex":"e8976af3ff","size":5,"soft_fail":false,"target_va":5373350336,"va":5374175012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*51986123(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374174672,"raw_hex":"ff15cb3e1903","size":6,"soft_fail":false,"va":5374175047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":186,"record_offset":3864,"parent_record_offset":3132,"ancestor_record_offset":3132,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":186,"record_offset":3300,"parent_record_offset":3132,"ancestor_record_offset":3132,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::mod_status::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/mod_status.rs
//     evidence={"module":186,"record_offset":4052,"parent_record_offset":3132,"ancestor_record_offset":3132,"inlinee":15625,"source_line":17,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/mod_detection.rs
//     evidence={"module":186,"record_offset":4260,"parent_record_offset":3132,"ancestor_record_offset":3132,"inlinee":15627,"source_line":68,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":3132,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":3132,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":3132,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":3132,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":3132,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":3132,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":4696,"parent_record_offset":4664,"ancestor_record_offset":3132,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":5964,"parent_record_offset":5936,"ancestor_record_offset":3132,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":186,"record_offset":4588,"parent_record_offset":4552,"ancestor_record_offset":3132,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":186,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":3132,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":186,"record_offset":4664,"parent_record_offset":4624,"ancestor_record_offset":3132,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":186,"record_offset":4624,"parent_record_offset":4588,"ancestor_record_offset":3132,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":186,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":3132,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":186,"record_offset":4440,"parent_record_offset":4392,"ancestor_record_offset":3132,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":186,"record_offset":4476,"parent_record_offset":4440,"ancestor_record_offset":3132,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":186,"record_offset":4512,"parent_record_offset":4476,"ancestor_record_offset":3132,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":186,"record_offset":4392,"parent_record_offset":4356,"ancestor_record_offset":3132,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":186,"record_offset":4356,"parent_record_offset":4260,"ancestor_record_offset":3132,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":5468,"parent_record_offset":5432,"ancestor_record_offset":3132,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":4080,"parent_record_offset":4052,"ancestor_record_offset":3132,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":3132,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":3536,"parent_record_offset":3512,"ancestor_record_offset":3132,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":4552,"parent_record_offset":4512,"ancestor_record_offset":3132,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":3512,"parent_record_offset":3412,"ancestor_record_offset":3132,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":186,"record_offset":3412,"parent_record_offset":3380,"ancestor_record_offset":3132,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":186,"record_offset":3896,"parent_record_offset":3864,"ancestor_record_offset":3132,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":186,"record_offset":3380,"parent_record_offset":3332,"ancestor_record_offset":3132,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":186,"record_offset":3332,"parent_record_offset":3300,"ancestor_record_offset":3132,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":186,"record_offset":3928,"parent_record_offset":3896,"ancestor_record_offset":3132,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":186,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":3132,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":186,"record_offset":3456,"parent_record_offset":3412,"ancestor_record_offset":3132,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":186,"record_offset":3956,"parent_record_offset":3928,"ancestor_record_offset":3132,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":186,"record_offset":4016,"parent_record_offset":3896,"ancestor_record_offset":3132,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":186,"record_offset":4824,"parent_record_offset":4512,"ancestor_record_offset":3132,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":3132,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":5032,"parent_record_offset":4260,"ancestor_record_offset":3132,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":186,"record_offset":5500,"parent_record_offset":5468,"ancestor_record_offset":3132,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":186,"record_offset":4992,"parent_record_offset":4260,"ancestor_record_offset":3132,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":186,"record_offset":4952,"parent_record_offset":4924,"ancestor_record_offset":3132,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":186,"record_offset":3568,"parent_record_offset":3412,"ancestor_record_offset":3132,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":186,"record_offset":3616,"parent_record_offset":3592,"ancestor_record_offset":3132,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":186,"record_offset":3592,"parent_record_offset":3568,"ancestor_record_offset":3132,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":4108,"parent_record_offset":4080,"ancestor_record_offset":3132,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":5308,"parent_record_offset":5272,"ancestor_record_offset":3132,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":4132,"parent_record_offset":4108,"ancestor_record_offset":3132,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":3132,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":5364,"parent_record_offset":5272,"ancestor_record_offset":3132,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":186,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":3132,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":186,"record_offset":4328,"parent_record_offset":4260,"ancestor_record_offset":3132,"inlinee":7549,"source_line":78,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":186,"record_offset":4900,"parent_record_offset":4260,"ancestor_record_offset":3132,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":186,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":3132,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::mod_detection::init_mods
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D0E
// 首条位置: {"module":186,"inlinee":15630}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":75376,"parent_record_offset":75316,"ancestor_record_offset":75316,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":186,"record_offset":75488,"parent_record_offset":75316,"ancestor_record_offset":75316,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":186,"record_offset":75544,"parent_record_offset":75316,"ancestor_record_offset":75316,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":186,"record_offset":75404,"parent_record_offset":75316,"ancestor_record_offset":75316,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":186,"record_offset":75512,"parent_record_offset":75488,"ancestor_record_offset":75316,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":75428,"parent_record_offset":75404,"ancestor_record_offset":75316,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":75452,"parent_record_offset":75428,"ancestor_record_offset":75316,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":186,"record_offset":75592,"parent_record_offset":75568,"ancestor_record_offset":75316,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":186,"record_offset":75568,"parent_record_offset":75544,"ancestor_record_offset":75316,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":186,"record_offset":75616,"parent_record_offset":75592,"ancestor_record_offset":75316,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D0B
// 首条位置: {"module":186,"inlinee":15627}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":186,"record_offset":4356,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":5032,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":186,"record_offset":4992,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":186,"record_offset":4328,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":7549,"source_line":78,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":186,"record_offset":4900,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":4260,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4260,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":4260,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":4260,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":4260,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":4260,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":4696,"parent_record_offset":4664,"ancestor_record_offset":4260,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":5964,"parent_record_offset":5936,"ancestor_record_offset":4260,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":186,"record_offset":4588,"parent_record_offset":4552,"ancestor_record_offset":4260,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":186,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":4260,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":186,"record_offset":4664,"parent_record_offset":4624,"ancestor_record_offset":4260,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":186,"record_offset":4624,"parent_record_offset":4588,"ancestor_record_offset":4260,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":186,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":4260,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":186,"record_offset":4440,"parent_record_offset":4392,"ancestor_record_offset":4260,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":186,"record_offset":4476,"parent_record_offset":4440,"ancestor_record_offset":4260,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":186,"record_offset":4512,"parent_record_offset":4476,"ancestor_record_offset":4260,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":186,"record_offset":4392,"parent_record_offset":4356,"ancestor_record_offset":4260,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":5468,"parent_record_offset":5432,"ancestor_record_offset":4260,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":4260,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":186,"record_offset":4552,"parent_record_offset":4512,"ancestor_record_offset":4260,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":186,"record_offset":4824,"parent_record_offset":4512,"ancestor_record_offset":4260,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":4260,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":5060,"parent_record_offset":5032,"ancestor_record_offset":4260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":186,"record_offset":5500,"parent_record_offset":5468,"ancestor_record_offset":4260,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":186,"record_offset":4952,"parent_record_offset":4924,"ancestor_record_offset":4260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":5308,"parent_record_offset":5272,"ancestor_record_offset":4260,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":4260,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":5364,"parent_record_offset":5272,"ancestor_record_offset":4260,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":186,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":4260,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":186,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":4260,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42ED
// 首条位置: {"module":230,"inlinee":17133}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":230,"record_offset":23716,"parent_record_offset":23420,"ancestor_record_offset":23420,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":23488,"parent_record_offset":23420,"ancestor_record_offset":23420,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":23884,"parent_record_offset":23420,"ancestor_record_offset":23420,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":230,"record_offset":23772,"parent_record_offset":23420,"ancestor_record_offset":23420,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":230,"record_offset":23632,"parent_record_offset":23420,"ancestor_record_offset":23420,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":24000,"parent_record_offset":23964,"ancestor_record_offset":23420,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":23964,"parent_record_offset":23940,"ancestor_record_offset":23420,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":230,"record_offset":23564,"parent_record_offset":23536,"ancestor_record_offset":23420,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":230,"record_offset":23512,"parent_record_offset":23488,"ancestor_record_offset":23420,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":23740,"parent_record_offset":23716,"ancestor_record_offset":23420,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":23912,"parent_record_offset":23884,"ancestor_record_offset":23420,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":230,"record_offset":24032,"parent_record_offset":24000,"ancestor_record_offset":23420,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":23656,"parent_record_offset":23632,"ancestor_record_offset":23420,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":23680,"parent_record_offset":23656,"ancestor_record_offset":23420,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":24060,"parent_record_offset":24000,"ancestor_record_offset":23420,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":24092,"parent_record_offset":23964,"ancestor_record_offset":23420,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":230,"record_offset":23820,"parent_record_offset":23796,"ancestor_record_offset":23420,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":230,"record_offset":23796,"parent_record_offset":23772,"ancestor_record_offset":23420,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":230,"record_offset":23844,"parent_record_offset":23820,"ancestor_record_offset":23420,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::mod_detection::indicates_tampering
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/mod_detection.rs
//     evidence={"module":230,"record_offset":23536,"parent_record_offset":23512,"ancestor_record_offset":23420,"inlinee":15635,"source_line":104,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::mod_detection::indicates_tampering
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D13
// 首条位置: {"module":230,"inlinee":15635}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":230,"record_offset":23564,"parent_record_offset":23536,"ancestor_record_offset":23536,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00230.jsonl.gz"}
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
// 完整结构及原始类型记录: mod_detection.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::mod_detection::ModDetectionPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$0::add_to_app<tiny_glade::mod_detection::ModDetectionPlugin>", "constructor": "bevy_app::plugin::sealed::impl$0::add_to_app", "arguments": ["tiny_glade::mod_detection::ModDetectionPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::mod_detection::ModDetectionPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::mod_detection::ModDetectionPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::mod_detection::ModDetectionPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::mod_detection::ModDetectionPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::mod_detection::ModDetectionPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::mod_detection::ModDetectionPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::mod_detection::ModDetectionPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::mod_detection::ModDetectionPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any<tiny_glade::mod_detection::ModDetectionPlugin>", "constructor": "downcast_rs::impl$0::into_any", "arguments": ["tiny_glade::mod_detection::ModDetectionPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::mod_detection::ModDetectionPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::mod_detection::ModDetectionPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::mod_detection::ModDetectionPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::mod_detection::ModDetectionPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::mod_detection::ModDetectionPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::mod_detection::ModDetectionPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<tiny_glade::mod_detection::ModDetectionPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["tiny_glade::mod_detection::ModDetectionPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod_detection.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod_detection.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::StartupRevealFinished>,bevy_ecs::system::system_param::Local<bool>,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::StartupRevealFinished>", "bevy_ecs::system::system_param::Local<bool>", "bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::StartupRevealFinished>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<bool>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>

// ------------------------------------------------------------------
// 函数: tiny_glade::mod_detection::impl$0::build
// 信息 ID: 8a3f61b059e2f9276e16bfb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::mod_detection::indicates_tampering
// 信息 ID: 2cbd33e6a179cee59da824a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0,tuple$<> >; module=230, inline_record=23536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::mod_detection::init_mods
// 信息 ID: 5ba04c8a2ce7b9ff305e150b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::mod_detection::impl$0::build; module=186, inline_record=75316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check
// 信息 ID: 24a8855c342390723a2316d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::mod_detection::spawn_package_integrity_check_after_reveal; module=186, inline_record=4260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure$0
// 信息 ID: f9969b97c6c9d68071adc879
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0 <- std::thread::lifecycle::spawn_unchecked<tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::mod_detection::spawn_informational_mods_label_integrity_check::closure_env$0,tuple$<> >; module=230, inline_record=23420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::mod_detection::spawn_package_integrity_check_after_reveal
// 信息 ID: 3fb7fd13115d814eb4e17f17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 91b4991100c4a4830d22fe7b: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::StartupRevealFinished>,bevy_ecs::system::system_param::Local<bool>,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::StartupRevealFinished>,bevy_ecs::system::system_param::Local<bool>,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::StartupRevealFinished>,bevy_ecs::system::system_param::Local<bool>,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)> @ 0x1aa057 -> 0x5365d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::StartupRevealFinished>,bevy_ecs::system::system_param::Local<bool>,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::StartupRevealFinished>,bevy_ecs::system::system_param::Local<bool>,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)> > @ 0x2ad051 -> 0x5365d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
