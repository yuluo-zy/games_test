// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/swap_replacement_mods.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 80225f5919af2f4caa4ad22c02e6e4ba74b1a223f8d713f2d3e0ad2a8d6fdce9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: swap_replacement_mods.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 111, 503, 1673, 1698, 1770, 2738
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use async_lock; // namespace candidate
//   package identities: [{"name":"async-lock","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.4.2"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::preactivate_boot_replacement_mods
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":78492,"rva":9473232,"va":5378182352}
// 编译实例 RVAs: 0x908cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6425668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e8440c6200","size":5,"soft_fail":false,"target_va":5384609408,"va":5378183735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t-2503825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e86fcbd9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378182524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099f90
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8842697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e8371279ff","size":5,"soft_fail":false,"target_va":5369339792,"va":5378182484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=2
//     target_aliases=26 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8753560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e8686e7aff","size":5,"soft_fail":false,"target_va":5369431360,"va":5378184915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409904b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t553788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e83c730800","size":5,"soft_fail":false,"target_va":5378737328,"va":5378183535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14595919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e84fb7de00","size":5,"soft_fail":false,"target_va":5392780768,"va":5378184844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38200880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e830e64602","size":5,"soft_fail":false,"target_va":5416385024,"va":5378184139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38437224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e868814a02","size":5,"soft_fail":false,"target_va":5416621408,"va":5378184179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e8832d0000","size":5,"soft_fail":false,"target_va":5378194560,"va":5378182904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38437045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e8b5804a02","size":5,"soft_fail":false,"target_va":5416621376,"va":5378184326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::enumerate_saved_sessions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2084260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e8a4cd1f00","size":5,"soft_fail":false,"target_va":5380266864,"va":5378182599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::startup_session::resolve_next_session
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t15737699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e86323f000","size":5,"soft_fail":false,"target_va":5393920400,"va":5378182696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::is_session_ok_to_load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2265782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e8b6922200","size":5,"soft_fail":false,"target_va":5380448448,"va":5378182661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::mark_session_loading
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2263901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e85d8b2200","size":5,"soft_fail":false,"target_va":5380447408,"va":5378183502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::saves
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4956932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e804a34b00","size":5,"soft_fail":false,"target_va":5383139520,"va":5378182583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5780334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e86e335800","size":5,"soft_fail":false,"target_va":5383963792,"va":5378183453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5788842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e8aa545800","size":5,"soft_fail":false,"target_va":5383972224,"va":5378183377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5781005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e80d365800","size":5,"soft_fail":false,"target_va":5383964912,"va":5378183902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5779951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e8ef315800","size":5,"soft_fail":false,"target_va":5383962816,"va":5378182860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38435194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e87a794a02","size":5,"soft_fail":false,"target_va":5416619360,"va":5378184161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::active_mod_names
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4723308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e86c124800","size":5,"soft_fail":false,"target_va":5382906864,"va":5378183551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::installed_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4721483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e84b0b4800","size":5,"soft_fail":false,"target_va":5382903888,"va":5378182400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::set_active_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4721894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378182352,"raw_hex":"e8e60c4800","size":5,"soft_fail":false,"target_va":5382905424,"va":5378183525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":79316,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":503,"record_offset":83892,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":78696,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::mod_status::ModStatus::set_replacement_mods_active
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/mod_status.rs
//     evidence={"module":503,"record_offset":83720,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":28632,"source_line":36,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$45::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":503,"record_offset":79344,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":13693,"source_line":436,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::load_saved_session_list
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":503,"record_offset":79372,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":28646,"source_line":475,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":503,"record_offset":83948,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":503,"record_offset":83808,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":82856,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":80416,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":80804,"parent_record_offset":78492,"ancestor_record_offset":78492,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":78952,"parent_record_offset":78928,"ancestor_record_offset":78492,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":78976,"parent_record_offset":78952,"ancestor_record_offset":78492,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":79000,"parent_record_offset":78976,"ancestor_record_offset":78492,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":78928,"parent_record_offset":78840,"ancestor_record_offset":78492,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":78868,"parent_record_offset":78840,"ancestor_record_offset":78492,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":78840,"parent_record_offset":78816,"ancestor_record_offset":78492,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":78816,"parent_record_offset":78792,"ancestor_record_offset":78492,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":78728,"parent_record_offset":78696,"ancestor_record_offset":78492,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":83212,"parent_record_offset":83184,"ancestor_record_offset":78492,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":83240,"parent_record_offset":83212,"ancestor_record_offset":78492,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":83152,"parent_record_offset":83120,"ancestor_record_offset":78492,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":83184,"parent_record_offset":83152,"ancestor_record_offset":78492,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":83300,"parent_record_offset":83184,"ancestor_record_offset":78492,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":83412,"parent_record_offset":83388,"ancestor_record_offset":78492,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":503,"record_offset":83264,"parent_record_offset":83240,"ancestor_record_offset":78492,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":78896,"parent_record_offset":78868,"ancestor_record_offset":78492,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":503,"record_offset":83020,"parent_record_offset":82972,"ancestor_record_offset":78492,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":83916,"parent_record_offset":83892,"ancestor_record_offset":78492,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":80752,"parent_record_offset":80724,"ancestor_record_offset":78492,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":83648,"parent_record_offset":82972,"ancestor_record_offset":78492,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":78756,"parent_record_offset":78728,"ancestor_record_offset":78492,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":79504,"parent_record_offset":79436,"ancestor_record_offset":78492,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":81228,"parent_record_offset":81204,"ancestor_record_offset":78492,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":81204,"parent_record_offset":81180,"ancestor_record_offset":78492,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":79472,"parent_record_offset":79436,"ancestor_record_offset":78492,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":503,"record_offset":81512,"parent_record_offset":81484,"ancestor_record_offset":78492,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":81484,"parent_record_offset":81456,"ancestor_record_offset":78492,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":81156,"parent_record_offset":81132,"ancestor_record_offset":78492,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":81180,"parent_record_offset":81156,"ancestor_record_offset":78492,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":81540,"parent_record_offset":81512,"ancestor_record_offset":78492,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":81132,"parent_record_offset":81016,"ancestor_record_offset":78492,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":79436,"parent_record_offset":79372,"ancestor_record_offset":78492,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":503,"record_offset":80544,"parent_record_offset":80520,"ancestor_record_offset":78492,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":503,"record_offset":80840,"parent_record_offset":80804,"ancestor_record_offset":78492,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":503,"record_offset":80984,"parent_record_offset":80952,"ancestor_record_offset":78492,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":503,"record_offset":80952,"parent_record_offset":80912,"ancestor_record_offset":78492,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":80888,"parent_record_offset":80864,"ancestor_record_offset":78492,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":80912,"parent_record_offset":80888,"ancestor_record_offset":78492,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":80864,"parent_record_offset":80840,"ancestor_record_offset":78492,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":81456,"parent_record_offset":80912,"ancestor_record_offset":78492,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":80520,"parent_record_offset":80488,"ancestor_record_offset":78492,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":80488,"parent_record_offset":80460,"ancestor_record_offset":78492,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":80644,"parent_record_offset":80612,"ancestor_record_offset":78492,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":80700,"parent_record_offset":80644,"ancestor_record_offset":78492,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":80612,"parent_record_offset":80584,"ancestor_record_offset":78492,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":80460,"parent_record_offset":80416,"ancestor_record_offset":78492,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":80724,"parent_record_offset":80700,"ancestor_record_offset":78492,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":81016,"parent_record_offset":80984,"ancestor_record_offset":78492,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":81300,"parent_record_offset":81016,"ancestor_record_offset":78492,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":81272,"parent_record_offset":81016,"ancestor_record_offset":78492,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":81076,"parent_record_offset":81016,"ancestor_record_offset":78492,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":83748,"parent_record_offset":83720,"ancestor_record_offset":78492,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":83832,"parent_record_offset":83808,"ancestor_record_offset":78492,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":83856,"parent_record_offset":83832,"ancestor_record_offset":78492,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":83772,"parent_record_offset":83748,"ancestor_record_offset":78492,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::saves_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":503,"record_offset":79408,"parent_record_offset":79372,"ancestor_record_offset":78492,"inlinee":28647,"source_line":442,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":503,"record_offset":83996,"parent_record_offset":83972,"ancestor_record_offset":78492,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":503,"record_offset":83972,"parent_record_offset":83948,"ancestor_record_offset":78492,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":503,"record_offset":84020,"parent_record_offset":83996,"ancestor_record_offset":78492,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":82892,"parent_record_offset":82856,"ancestor_record_offset":78492,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":84776,"parent_record_offset":84744,"ancestor_record_offset":78492,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":85256,"parent_record_offset":85040,"ancestor_record_offset":78492,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":503,"record_offset":83048,"parent_record_offset":83020,"ancestor_record_offset":78492,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":83120,"parent_record_offset":82972,"ancestor_record_offset":78492,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":83336,"parent_record_offset":83120,"ancestor_record_offset":78492,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":503,"record_offset":83076,"parent_record_offset":83048,"ancestor_record_offset":78492,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":503,"record_offset":82972,"parent_record_offset":82928,"ancestor_record_offset":78492,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":503,"record_offset":82928,"parent_record_offset":82892,"ancestor_record_offset":78492,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::swap_replacement_mods
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":69684,"rva":9470704,"va":5378179824}
// 编译实例 RVAs: 0x9082f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2501303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378179824,"raw_hex":"e849d5d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378180002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8748929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378179824,"raw_hex":"e87f807aff","size":5,"soft_fail":false,"target_va":5369431360,"va":5378180284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140907470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378179824,"raw_hex":"e87bf0ffff","size":5,"soft_fail":false,"target_va":5378176112,"va":5378180080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::startup_session::resolve_next_session
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t15740506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378179824,"raw_hex":"e85a2ef000","size":5,"soft_fail":false,"target_va":5393920400,"va":5378179889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":69836,"parent_record_offset":69684,"ancestor_record_offset":69684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":69992,"parent_record_offset":69968,"ancestor_record_offset":69684,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":70016,"parent_record_offset":69992,"ancestor_record_offset":69684,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":70040,"parent_record_offset":70016,"ancestor_record_offset":69684,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":69968,"parent_record_offset":69912,"ancestor_record_offset":69684,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":69940,"parent_record_offset":69912,"ancestor_record_offset":69684,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":69912,"parent_record_offset":69888,"ancestor_record_offset":69684,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":69888,"parent_record_offset":69864,"ancestor_record_offset":69684,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":70708,"parent_record_offset":70684,"ancestor_record_offset":69684,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":71208,"parent_record_offset":71180,"ancestor_record_offset":69684,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":69864,"parent_record_offset":69836,"ancestor_record_offset":69684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::_::impl$0::dyn_clone
// 记录形式: compiled_procedure; instances=3
// 源码行: 32, 39, 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":65308,"rva":9469648,"va":5378178768}
// 编译实例 RVAs: 0x907ed0, 0x907ee0, 0x907ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::impl$4::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":121156,"rva":9487280,"va":5378196400}
// 编译实例 RVAs: 0x90c3b0, 0x17f1890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t6627659","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378196400,"raw_hex":"e94b216500","size":5,"soft_fail":false,"target_va":5384824080,"va":5378196416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::impl$12::fmt
// 记录形式: compiled_procedure; instances=3
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":111,"record_offset":384,"rva":3816080,"va":5372525200}
// 编译实例 RVAs: 0x3a3a90, 0x9029b0, 0x17ebab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t12298859","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372525200,"raw_hex":"e96baabb00","size":5,"soft_fail":false,"target_va":5384824080,"va":5372525216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::impl$20::fmt
// 记录形式: compiled_procedure; instances=4
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":111,"record_offset":248,"rva":3816048,"va":5372525168}
// 编译实例 RVAs: 0x3a3a70, 0x902990, 0x17eba90, 0x21a7b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t12298891","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372525168,"raw_hex":"e98baabb00","size":5,"soft_fail":false,"target_va":5384824080,"va":5372525184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::apply_preactivated_mod_textures
// 记录形式: compiled_procedure; instances=1
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":72344,"rva":9471520,"va":5378180640}
// 编译实例 RVAs: 0x908620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6428426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e80a176200","size":5,"soft_fail":false,"target_va":5384609408,"va":5378180977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-2502048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e860d2d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378180747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t551773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e85d6b0800","size":5,"soft_fail":false,"target_va":5378733248,"va":5378181470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38201437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e85de84602","size":5,"soft_fail":false,"target_va":5416382848,"va":5378181406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7af50
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38217503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e81f274702","size":5,"soft_fail":false,"target_va":5416398672,"va":5378181164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5024913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e891ac4c00","size":5,"soft_fail":false,"target_va":5383206064,"va":5378181146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6443088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e850506200","size":5,"soft_fail":false,"target_va":5384624080,"va":5378180987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::store::locked_store
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t15721821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e85de5ef00","size":5,"soft_fail":false,"target_va":5393902624,"va":5378180798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mod_textures::reload_mod_textures
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t896517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e805ae0d00","size":5,"soft_fail":false,"target_va":5379077584,"va":5378181062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::active_mod_names
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4726176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e8a01d4800","size":5,"soft_fail":false,"target_va":5382906864,"va":5378180683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::shadowed_paths
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4723641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180640,"raw_hex":"e8b9134800","size":5,"soft_fail":false,"target_va":5382904688,"va":5378181042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":73156,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":503,"record_offset":73468,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":503,"record_offset":73272,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":72528,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":503,"record_offset":73328,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":503,"record_offset":73188,"parent_record_offset":72344,"ancestor_record_offset":72344,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":72780,"parent_record_offset":72756,"ancestor_record_offset":72344,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":72804,"parent_record_offset":72780,"ancestor_record_offset":72344,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":72828,"parent_record_offset":72804,"ancestor_record_offset":72344,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":72756,"parent_record_offset":72696,"ancestor_record_offset":72344,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":72724,"parent_record_offset":72696,"ancestor_record_offset":72344,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":72696,"parent_record_offset":72668,"ancestor_record_offset":72344,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":72668,"parent_record_offset":72644,"ancestor_record_offset":72344,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":75176,"parent_record_offset":75140,"ancestor_record_offset":72344,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":75140,"parent_record_offset":75116,"ancestor_record_offset":72344,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":72552,"parent_record_offset":72528,"ancestor_record_offset":72344,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::Mutex::unlock_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":503,"record_offset":75352,"parent_record_offset":75328,"ancestor_record_offset":72344,"inlinee":28636,"source_line":203,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::impl$21::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":503,"record_offset":75328,"parent_record_offset":75304,"ancestor_record_offset":72344,"inlinee":28634,"source_line":662,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":503,"record_offset":73496,"parent_record_offset":73468,"ancestor_record_offset":72344,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":503,"record_offset":73904,"parent_record_offset":73880,"ancestor_record_offset":72344,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":74832,"parent_record_offset":74804,"ancestor_record_offset":72344,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":74860,"parent_record_offset":74832,"ancestor_record_offset":72344,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":72996,"parent_record_offset":72968,"ancestor_record_offset":72344,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":503,"record_offset":74124,"parent_record_offset":74100,"ancestor_record_offset":72344,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":73296,"parent_record_offset":73272,"ancestor_record_offset":72344,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":73936,"parent_record_offset":73852,"ancestor_record_offset":72344,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":72576,"parent_record_offset":72552,"ancestor_record_offset":72344,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":503,"record_offset":74392,"parent_record_offset":74368,"ancestor_record_offset":72344,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":503,"record_offset":74244,"parent_record_offset":74220,"ancestor_record_offset":72344,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":75208,"parent_record_offset":75176,"ancestor_record_offset":72344,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":75380,"parent_record_offset":75352,"ancestor_record_offset":72344,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":73212,"parent_record_offset":73188,"ancestor_record_offset":72344,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":73236,"parent_record_offset":73212,"ancestor_record_offset":72344,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":75236,"parent_record_offset":75176,"ancestor_record_offset":72344,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":75268,"parent_record_offset":75140,"ancestor_record_offset":72344,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":503,"record_offset":74068,"parent_record_offset":74040,"ancestor_record_offset":72344,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":503,"record_offset":74100,"parent_record_offset":74068,"ancestor_record_offset":72344,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":503,"record_offset":74040,"parent_record_offset":74000,"ancestor_record_offset":72344,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":503,"record_offset":73880,"parent_record_offset":73852,"ancestor_record_offset":72344,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":74368,"parent_record_offset":73760,"ancestor_record_offset":72344,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":74220,"parent_record_offset":74000,"ancestor_record_offset":72344,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":73852,"parent_record_offset":73796,"ancestor_record_offset":72344,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":74000,"parent_record_offset":73972,"ancestor_record_offset":72344,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":74772,"parent_record_offset":74740,"ancestor_record_offset":72344,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":73824,"parent_record_offset":73796,"ancestor_record_offset":72344,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":73760,"parent_record_offset":73696,"ancestor_record_offset":72344,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":73696,"parent_record_offset":73668,"ancestor_record_offset":72344,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":74740,"parent_record_offset":73696,"ancestor_record_offset":72344,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":73732,"parent_record_offset":73696,"ancestor_record_offset":72344,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":73796,"parent_record_offset":73760,"ancestor_record_offset":72344,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":74804,"parent_record_offset":74772,"ancestor_record_offset":72344,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":73668,"parent_record_offset":73640,"ancestor_record_offset":72344,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":73972,"parent_record_offset":73760,"ancestor_record_offset":72344,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":503,"record_offset":73376,"parent_record_offset":73352,"ancestor_record_offset":72344,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":503,"record_offset":73352,"parent_record_offset":73328,"ancestor_record_offset":72344,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":503,"record_offset":73400,"parent_record_offset":73376,"ancestor_record_offset":72344,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::_::impl$0::get_param
// 记录形式: compiled_procedure; instances=2
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":112844,"rva":25098656,"va":5393807776}
// 编译实例 RVAs: 0x17ef9a0, 0x1903440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t22813459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393807776,"raw_hex":"e8131b5c01","size":5,"soft_fail":false,"target_va":5416622240,"va":5393808776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$118::get_param
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1673,"record_offset":112996,"parent_record_offset":112844,"ancestor_record_offset":112844,"inlinee":4583,"source_line":2160,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1673,"record_offset":113452,"parent_record_offset":113428,"ancestor_record_offset":112844,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1673,"record_offset":113500,"parent_record_offset":113476,"ancestor_record_offset":112844,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1673,"record_offset":113476,"parent_record_offset":113452,"ancestor_record_offset":112844,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":113428,"parent_record_offset":113404,"ancestor_record_offset":112844,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":113404,"parent_record_offset":113364,"ancestor_record_offset":112844,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":113364,"parent_record_offset":113340,"ancestor_record_offset":112844,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::_::impl$0::get_param
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1673,"record_offset":118412,"parent_record_offset":112996,"ancestor_record_offset":112844,"inlinee":4207,"source_line":56,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1673,"record_offset":114808,"parent_record_offset":114784,"ancestor_record_offset":112844,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1673,"record_offset":114784,"parent_record_offset":114752,"ancestor_record_offset":112844,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1673,"record_offset":113656,"parent_record_offset":113620,"ancestor_record_offset":112844,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1673,"record_offset":113592,"parent_record_offset":113088,"ancestor_record_offset":112844,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1673,"record_offset":113620,"parent_record_offset":113592,"ancestor_record_offset":112844,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1673,"record_offset":113120,"parent_record_offset":113088,"ancestor_record_offset":112844,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1673,"record_offset":113684,"parent_record_offset":113620,"ancestor_record_offset":112844,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1673,"record_offset":113168,"parent_record_offset":113144,"ancestor_record_offset":112844,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1673,"record_offset":113144,"parent_record_offset":113120,"ancestor_record_offset":112844,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1673,"record_offset":113340,"parent_record_offset":113316,"ancestor_record_offset":112844,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$104::get_param
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1673,"record_offset":118448,"parent_record_offset":118412,"ancestor_record_offset":112844,"inlinee":4208,"source_line":2160,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::get_param
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1673,"record_offset":113060,"parent_record_offset":112996,"ancestor_record_offset":112844,"inlinee":4156,"source_line":803,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$8::get_param
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1673,"record_offset":114252,"parent_record_offset":112996,"ancestor_record_offset":112844,"inlinee":4194,"source_line":882,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":1673,"record_offset":114876,"parent_record_offset":114852,"ancestor_record_offset":112844,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1673,"record_offset":114852,"parent_record_offset":114284,"ancestor_record_offset":112844,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1673,"record_offset":114284,"parent_record_offset":114252,"ancestor_record_offset":112844,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1673,"record_offset":113088,"parent_record_offset":113060,"ancestor_record_offset":112844,"inlinee":4157,"source_line":641,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1673,"record_offset":113256,"parent_record_offset":113196,"ancestor_record_offset":112844,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1673,"record_offset":113276,"parent_record_offset":113256,"ancestor_record_offset":112844,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1673,"record_offset":113316,"parent_record_offset":113144,"ancestor_record_offset":112844,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1673,"record_offset":113196,"parent_record_offset":113168,"ancestor_record_offset":112844,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1673,"record_offset":113228,"parent_record_offset":113196,"ancestor_record_offset":112844,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1673,"record_offset":113544,"parent_record_offset":113364,"ancestor_record_offset":112844,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1673,"record_offset":114900,"parent_record_offset":114876,"ancestor_record_offset":112844,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1673,"record_offset":114924,"parent_record_offset":114900,"ancestor_record_offset":112844,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1673,"record_offset":114032,"parent_record_offset":114008,"ancestor_record_offset":112844,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::_::impl$0::init_access
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7042
// 首条位置: {"module":1673,"inlinee":28738}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::_::impl$0::init_state
// 记录形式: compiled_procedure; instances=1
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":65740,"rva":9469696,"va":5378178816}
// 编译实例 RVAs: 0x907f00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-2500274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378178816,"raw_hex":"e84ed9d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378178973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14168c190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14172767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378178816,"raw_hex":"e85f42d800","size":5,"soft_fail":false,"target_va":5392351632,"va":5378178860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141691140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14193156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378178816,"raw_hex":"e80492d800","size":5,"soft_fail":false,"target_va":5392372032,"va":5378178871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416a3890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14268283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378178816,"raw_hex":"e87bb7d900","size":5,"soft_fail":false,"target_va":5392447632,"va":5378179344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416cf8f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14448731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378178816,"raw_hex":"e85b78dc00","size":5,"soft_fail":false,"target_va":5392627952,"va":5378179216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416d15f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14456342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378178816,"raw_hex":"e81696dc00","size":5,"soft_fail":false,"target_va":5392635376,"va":5378179029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416d4ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14471278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378178816,"raw_hex":"e86ed0dc00","size":5,"soft_fail":false,"target_va":5392650224,"va":5378178941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416d8650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14484835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378178816,"raw_hex":"e86305dd00","size":5,"soft_fail":false,"target_va":5392664144,"va":5378179304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416e8ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14552758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378178816,"raw_hex":"e8b60ede00","size":5,"soft_fail":false,"target_va":5392731888,"va":5378179125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$118::init_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":503,"record_offset":65892,"parent_record_offset":65740,"ancestor_record_offset":65740,"inlinee":4581,"source_line":2123,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":66228,"parent_record_offset":66204,"ancestor_record_offset":65740,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":66252,"parent_record_offset":66228,"ancestor_record_offset":65740,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":66276,"parent_record_offset":66252,"ancestor_record_offset":65740,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":66204,"parent_record_offset":66112,"ancestor_record_offset":65740,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":66140,"parent_record_offset":66112,"ancestor_record_offset":65740,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":66112,"parent_record_offset":66088,"ancestor_record_offset":65740,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":66088,"parent_record_offset":66064,"ancestor_record_offset":65740,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::register::ComponentsRegistrator::new
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/register.rs
//     evidence={"module":503,"record_offset":65984,"parent_record_offset":65952,"ancestor_record_offset":65740,"inlinee":4111,"source_line":85,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::_::impl$0::init_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":503,"record_offset":69620,"parent_record_offset":65892,"ancestor_record_offset":65740,"inlinee":4139,"source_line":56,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$104::init_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":503,"record_offset":69644,"parent_record_offset":69620,"ancestor_record_offset":65740,"inlinee":4141,"source_line":2123,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$8::init_state
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":503,"record_offset":65920,"parent_record_offset":65892,"ancestor_record_offset":65740,"inlinee":4129,"source_line":838,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::components_registrator
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":503,"record_offset":65952,"parent_record_offset":65920,"ancestor_record_offset":65740,"inlinee":4109,"source_line":238,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":66172,"parent_record_offset":66140,"ancestor_record_offset":65740,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":66016,"parent_record_offset":65920,"ancestor_record_offset":65740,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::_::impl$0::validate_param
// 记录形式: compiled_procedure; instances=2
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":108720,"rva":25098128,"va":5393807248}
// 编译实例 RVAs: 0x17ef790, 0x1903230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::message::message_writer::_::impl$0::validate_param
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1673,"record_offset":112312,"parent_record_offset":108720,"ancestor_record_offset":108720,"inlinee":4240,"source_line":56,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$7::validate_param
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1673,"record_offset":108888,"parent_record_offset":108720,"ancestor_record_offset":108720,"inlinee":4222,"source_line":783,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$8::validate_param
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1673,"record_offset":109952,"parent_record_offset":108720,"ancestor_record_offset":108720,"inlinee":4225,"source_line":862,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1673,"record_offset":112812,"parent_record_offset":108720,"ancestor_record_offset":108720,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1673,"record_offset":109240,"parent_record_offset":109216,"ancestor_record_offset":108720,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1673,"record_offset":109288,"parent_record_offset":109264,"ancestor_record_offset":108720,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1673,"record_offset":109264,"parent_record_offset":109240,"ancestor_record_offset":108720,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":109216,"parent_record_offset":109192,"ancestor_record_offset":108720,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":109192,"parent_record_offset":109152,"ancestor_record_offset":108720,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1673,"record_offset":109152,"parent_record_offset":109128,"ancestor_record_offset":108720,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":1673,"record_offset":109448,"parent_record_offset":109424,"ancestor_record_offset":108720,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1673,"record_offset":109424,"parent_record_offset":109404,"ancestor_record_offset":108720,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":1673,"record_offset":108916,"parent_record_offset":108888,"ancestor_record_offset":108720,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1673,"record_offset":108956,"parent_record_offset":108936,"ancestor_record_offset":108720,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1673,"record_offset":108936,"parent_record_offset":108916,"ancestor_record_offset":108720,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1673,"record_offset":109128,"parent_record_offset":109104,"ancestor_record_offset":108720,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$8::validate_param
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1673,"record_offset":112336,"parent_record_offset":112312,"ancestor_record_offset":108720,"inlinee":4225,"source_line":862,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1673,"record_offset":109044,"parent_record_offset":108984,"ancestor_record_offset":108720,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1673,"record_offset":109064,"parent_record_offset":109044,"ancestor_record_offset":108720,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1673,"record_offset":109380,"parent_record_offset":108888,"ancestor_record_offset":108720,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1673,"record_offset":109104,"parent_record_offset":108936,"ancestor_record_offset":108720,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1673,"record_offset":108984,"parent_record_offset":108956,"ancestor_record_offset":108720,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1673,"record_offset":109016,"parent_record_offset":108984,"ancestor_record_offset":108720,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1673,"record_offset":109332,"parent_record_offset":109152,"ancestor_record_offset":108720,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1673,"record_offset":109760,"parent_record_offset":109736,"ancestor_record_offset":108720,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::ModSwapReloads::apply
// 记录形式: compiled_procedure; instances=1
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":58088,"rva":9466992,"va":5378176112}
// 编译实例 RVAs: 0x907470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6470617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8d9bb6200","size":5,"soft_fail":false,"target_va":5384648448,"va":5378177826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6471064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e898bd6200","size":5,"soft_fail":false,"target_va":5384648032,"va":5378176963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t6433000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8e8286200","size":5,"soft_fail":false,"target_va":5384609408,"va":5378176403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-2497792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e800e3d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378176491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8747121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e88f877aff","size":5,"soft_fail":false,"target_va":5369431360,"va":5378178476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409904b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t559239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e887880800","size":5,"soft_fail":false,"target_va":5378737328,"va":5378178084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14602337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e861d0de00","size":5,"soft_fail":false,"target_va":5392780768,"va":5378178426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38172931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e803794602","size":5,"soft_fail":false,"target_va":5416350720,"va":5378177784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38204316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e89cf34602","size":5,"soft_fail":false,"target_va":5416382848,"va":5378178527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38206908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8bcfd4602","size":5,"soft_fail":false,"target_va":5416385024,"va":5378178111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7af50
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38221256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8c8354702","size":5,"soft_fail":false,"target_va":5416398672,"va":5378177411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5028669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e83dbb4c00","size":5,"soft_fail":false,"target_va":5383206064,"va":5378177390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6446447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e86f5d6200","size":5,"soft_fail":false,"target_va":5384624080,"va":5378177628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::store::locked_store
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t15725315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e803f3ef00","size":5,"soft_fail":false,"target_va":5393902624,"va":5378177304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38443256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8f8984a02","size":5,"soft_fail":false,"target_va":5416621408,"va":5378178147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e83b460000","size":5,"soft_fail":false,"target_va":5378194560,"va":5378176576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38443133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e87d984a02","size":5,"soft_fail":false,"target_va":5416621376,"va":5378178238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mod_textures::reload_mod_textures
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t900215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e877bc0d00","size":5,"soft_fail":false,"target_va":5379077584,"va":5378177364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::swap_replacement_mods::reload_assets_for_changed_paths
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8f10e0000","size":5,"soft_fail":false,"target_va":5378181568,"va":5378177738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::mark_session_loading
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2271226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8faa72200","size":5,"soft_fail":false,"target_va":5380447408,"va":5378176177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5786853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8e54c5800","size":5,"soft_fail":false,"target_va":5383963792,"va":5378176934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5795361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8216e5800","size":5,"soft_fail":false,"target_va":5383972224,"va":5378176858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5786843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8db4c5800","size":5,"soft_fail":false,"target_va":5383964912,"va":5378178064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5786269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e89d4a5800","size":5,"soft_fail":false,"target_va":5383962816,"va":5378176542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t5553472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e840bd5400","size":5,"soft_fail":false,"target_va":5383730768,"va":5378177291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38441226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e80a914a02","size":5,"soft_fail":false,"target_va":5416619360,"va":5378178129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::active_mod_names
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4730439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e8472e4800","size":5,"soft_fail":false,"target_va":5382906864,"va":5378176420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::set_active_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4729224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378176112,"raw_hex":"e888294800","size":5,"soft_fail":false,"target_va":5382905424,"va":5378176195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":62496,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":58316,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":503,"record_offset":62524,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":503,"record_offset":63664,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":503,"record_offset":58428,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":58712,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":58288,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::mod_status::ModStatus::set_replacement_mods_active
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/mod_status.rs
//     evidence={"module":503,"record_offset":58624,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":28632,"source_line":36,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":503,"record_offset":58484,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":503,"record_offset":58344,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":60764,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":59364,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":59752,"parent_record_offset":58088,"ancestor_record_offset":58088,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":58984,"parent_record_offset":58960,"ancestor_record_offset":58088,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":59008,"parent_record_offset":58984,"ancestor_record_offset":58088,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":59032,"parent_record_offset":59008,"ancestor_record_offset":58088,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":58960,"parent_record_offset":58896,"ancestor_record_offset":58088,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":63884,"parent_record_offset":63860,"ancestor_record_offset":58088,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":58928,"parent_record_offset":58896,"ancestor_record_offset":58088,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":58896,"parent_record_offset":58868,"ancestor_record_offset":58088,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":63932,"parent_record_offset":63908,"ancestor_record_offset":58088,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":63908,"parent_record_offset":63884,"ancestor_record_offset":58088,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":58868,"parent_record_offset":58840,"ancestor_record_offset":58088,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":62672,"parent_record_offset":62636,"ancestor_record_offset":58088,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":62636,"parent_record_offset":62612,"ancestor_record_offset":58088,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":63860,"parent_record_offset":63820,"ancestor_record_offset":58088,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":63788,"parent_record_offset":63752,"ancestor_record_offset":58088,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":63820,"parent_record_offset":63788,"ancestor_record_offset":58088,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":58736,"parent_record_offset":58712,"ancestor_record_offset":58088,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::Mutex::unlock_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":503,"record_offset":62848,"parent_record_offset":62824,"ancestor_record_offset":58088,"inlinee":28636,"source_line":203,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] async_lock::mutex::impl$21::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"async-lock","version":"3.4.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/async-lock-3.4.2/src/mutex.rs
//     evidence={"module":503,"record_offset":62824,"parent_record_offset":62800,"ancestor_record_offset":58088,"inlinee":28634,"source_line":662,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":503,"record_offset":62552,"parent_record_offset":62524,"ancestor_record_offset":58088,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":503,"record_offset":63688,"parent_record_offset":63664,"ancestor_record_offset":58088,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":503,"record_offset":63752,"parent_record_offset":63688,"ancestor_record_offset":58088,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":61112,"parent_record_offset":61084,"ancestor_record_offset":58088,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":61140,"parent_record_offset":61112,"ancestor_record_offset":58088,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":61052,"parent_record_offset":61020,"ancestor_record_offset":58088,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":61084,"parent_record_offset":61052,"ancestor_record_offset":58088,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":61200,"parent_record_offset":61084,"ancestor_record_offset":58088,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":503,"record_offset":61312,"parent_record_offset":61288,"ancestor_record_offset":58088,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":503,"record_offset":61164,"parent_record_offset":61140,"ancestor_record_offset":58088,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":59204,"parent_record_offset":59176,"ancestor_record_offset":58088,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":503,"record_offset":60920,"parent_record_offset":60876,"ancestor_record_offset":58088,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":58452,"parent_record_offset":58428,"ancestor_record_offset":58088,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":59700,"parent_record_offset":59672,"ancestor_record_offset":58088,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":61548,"parent_record_offset":60876,"ancestor_record_offset":58088,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":58760,"parent_record_offset":58736,"ancestor_record_offset":58088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":62704,"parent_record_offset":62672,"ancestor_record_offset":58088,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":60172,"parent_record_offset":60148,"ancestor_record_offset":58088,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":60148,"parent_record_offset":60124,"ancestor_record_offset":58088,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":503,"record_offset":60356,"parent_record_offset":60328,"ancestor_record_offset":58088,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":63972,"parent_record_offset":63820,"ancestor_record_offset":58088,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":503,"record_offset":60456,"parent_record_offset":60428,"ancestor_record_offset":58088,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":60428,"parent_record_offset":60400,"ancestor_record_offset":58088,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":60100,"parent_record_offset":60076,"ancestor_record_offset":58088,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":60124,"parent_record_offset":60100,"ancestor_record_offset":58088,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":60484,"parent_record_offset":60456,"ancestor_record_offset":58088,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":60076,"parent_record_offset":59960,"ancestor_record_offset":58088,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":503,"record_offset":59492,"parent_record_offset":59468,"ancestor_record_offset":58088,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":503,"record_offset":59788,"parent_record_offset":59752,"ancestor_record_offset":58088,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":503,"record_offset":59928,"parent_record_offset":59896,"ancestor_record_offset":58088,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":503,"record_offset":59896,"parent_record_offset":59860,"ancestor_record_offset":58088,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":59836,"parent_record_offset":59812,"ancestor_record_offset":58088,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":59860,"parent_record_offset":59836,"ancestor_record_offset":58088,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":59812,"parent_record_offset":59788,"ancestor_record_offset":58088,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":503,"record_offset":60400,"parent_record_offset":59860,"ancestor_record_offset":58088,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":59468,"parent_record_offset":59436,"ancestor_record_offset":58088,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":59436,"parent_record_offset":59408,"ancestor_record_offset":58088,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":59592,"parent_record_offset":59560,"ancestor_record_offset":58088,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":59648,"parent_record_offset":59592,"ancestor_record_offset":58088,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":59560,"parent_record_offset":59532,"ancestor_record_offset":58088,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":59408,"parent_record_offset":59364,"ancestor_record_offset":58088,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":503,"record_offset":59672,"parent_record_offset":59648,"ancestor_record_offset":58088,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":59960,"parent_record_offset":59928,"ancestor_record_offset":58088,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":60244,"parent_record_offset":59960,"ancestor_record_offset":58088,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":60216,"parent_record_offset":59960,"ancestor_record_offset":58088,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":503,"record_offset":60020,"parent_record_offset":59960,"ancestor_record_offset":58088,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":58652,"parent_record_offset":58624,"ancestor_record_offset":58088,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":62876,"parent_record_offset":62848,"ancestor_record_offset":58088,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":58368,"parent_record_offset":58344,"ancestor_record_offset":58088,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":58392,"parent_record_offset":58368,"ancestor_record_offset":58088,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":58676,"parent_record_offset":58652,"ancestor_record_offset":58088,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":62732,"parent_record_offset":62672,"ancestor_record_offset":58088,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":62764,"parent_record_offset":62636,"ancestor_record_offset":58088,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":503,"record_offset":58532,"parent_record_offset":58508,"ancestor_record_offset":58088,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":503,"record_offset":58508,"parent_record_offset":58484,"ancestor_record_offset":58088,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":503,"record_offset":58556,"parent_record_offset":58532,"ancestor_record_offset":58088,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":60800,"parent_record_offset":60764,"ancestor_record_offset":58088,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":64336,"parent_record_offset":64304,"ancestor_record_offset":58088,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":64800,"parent_record_offset":64584,"ancestor_record_offset":58088,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":503,"record_offset":60948,"parent_record_offset":60920,"ancestor_record_offset":58088,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":61020,"parent_record_offset":60876,"ancestor_record_offset":58088,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":503,"record_offset":61236,"parent_record_offset":61020,"ancestor_record_offset":58088,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":503,"record_offset":60976,"parent_record_offset":60948,"ancestor_record_offset":58088,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":503,"record_offset":60876,"parent_record_offset":60836,"ancestor_record_offset":58088,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":503,"record_offset":60836,"parent_record_offset":60800,"ancestor_record_offset":58088,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::confirm_replacement_mods
// 记录形式: compiled_procedure; instances=1
// 源码行: 271
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":71372,"rva":9471184,"va":5378180304}
// 编译实例 RVAs: 0x9084d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2501824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180304,"raw_hex":"e840d3d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378180523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8749263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180304,"raw_hex":"e8317f7aff","size":5,"soft_fail":false,"target_va":5369431360,"va":5378180618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140907470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180304,"raw_hex":"e8f3eeffff","size":5,"soft_fail":false,"target_va":5378176112,"va":5378180472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f7ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13564379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378180304,"raw_hex":"e8dbf9ce00","size":5,"soft_fail":false,"target_va":5391744752,"va":5378180368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-2501866","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378180304,"raw_hex":"e916d3d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378180565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":71540,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":71568,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":71624,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":71656,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":71956,"parent_record_offset":71932,"ancestor_record_offset":71372,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":71980,"parent_record_offset":71956,"ancestor_record_offset":71372,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":72004,"parent_record_offset":71980,"ancestor_record_offset":71372,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":71932,"parent_record_offset":71868,"ancestor_record_offset":71372,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":71900,"parent_record_offset":71868,"ancestor_record_offset":71372,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":71868,"parent_record_offset":71840,"ancestor_record_offset":71372,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":71840,"parent_record_offset":71812,"ancestor_record_offset":71372,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":71680,"parent_record_offset":71656,"ancestor_record_offset":71372,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":72180,"parent_record_offset":72152,"ancestor_record_offset":71372,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":71704,"parent_record_offset":71680,"ancestor_record_offset":71372,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::swap_replacement_mods::confirm_replacement_mods::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/swap_replacement_mods.rs
//     evidence={"module":503,"record_offset":71596,"parent_record_offset":71568,"ancestor_record_offset":71372,"inlinee":28638,"source_line":277,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::confirm_replacement_mods::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FDE
// 首条位置: {"module":503,"inlinee":28638}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::swap_replacement_mods::reload_assets_for_changed_paths
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":75628,"rva":9472448,"va":5378181568}
// 编译实例 RVAs: 0x9089c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2503448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378181568,"raw_hex":"e8e8ccd9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378182147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409774a0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t452569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378181568,"raw_hex":"e8d9e70600","size":5,"soft_fail":false,"target_va":5378634912,"va":5378182338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15104760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378181568,"raw_hex":"e8f87ae600","size":5,"soft_fail":false,"target_va":5393286448,"va":5378181683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b2d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swap_replacement_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15376900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378181568,"raw_hex":"e804a2ea00","size":5,"soft_fail":false,"target_va":5393558928,"va":5378182023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38440120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378181568,"raw_hex":"e8b88c4a02","size":5,"soft_fail":false,"target_va":5416622240,"va":5378182115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::GpuWorld::update_mesh_atlas_slices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t885789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378181568,"raw_hex":"e81d840d00","size":5,"soft_fail":false,"target_va":5379067728,"va":5378181934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_atlases::reload_nani_meshes_after_mod_swap
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1586565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378181568,"raw_hex":"e885351800","size":5,"soft_fail":false,"target_va":5379768208,"va":5378181638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1459642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378181568,"raw_hex":"e8ba451600","size":5,"soft_fail":false,"target_va":5379641312,"va":5378181665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":77976,"parent_record_offset":75628,"ancestor_record_offset":75628,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":503,"record_offset":76384,"parent_record_offset":75628,"ancestor_record_offset":75628,"inlinee":28640,"source_line":104,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":503,"record_offset":75808,"parent_record_offset":75628,"ancestor_record_offset":75628,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":503,"record_offset":75836,"parent_record_offset":75628,"ancestor_record_offset":75628,"inlinee":12096,"source_line":1659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$27::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":503,"record_offset":77432,"parent_record_offset":75628,"ancestor_record_offset":75628,"inlinee":12100,"source_line":1718,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":78124,"parent_record_offset":78100,"ancestor_record_offset":75628,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":78148,"parent_record_offset":78124,"ancestor_record_offset":75628,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":78172,"parent_record_offset":78148,"ancestor_record_offset":75628,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":78100,"parent_record_offset":78072,"ancestor_record_offset":75628,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":503,"record_offset":77048,"parent_record_offset":77024,"ancestor_record_offset":75628,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":503,"record_offset":76096,"parent_record_offset":76072,"ancestor_record_offset":75628,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":503,"record_offset":76616,"parent_record_offset":76592,"ancestor_record_offset":75628,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":503,"record_offset":76700,"parent_record_offset":76676,"ancestor_record_offset":75628,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":77116,"parent_record_offset":76532,"ancestor_record_offset":75628,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":76272,"parent_record_offset":76244,"ancestor_record_offset":75628,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":503,"record_offset":76848,"parent_record_offset":76820,"ancestor_record_offset":75628,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":503,"record_offset":76820,"parent_record_offset":76792,"ancestor_record_offset":75628,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":77372,"parent_record_offset":76384,"ancestor_record_offset":75628,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":76128,"parent_record_offset":76040,"ancestor_record_offset":75628,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":76640,"parent_record_offset":76616,"ancestor_record_offset":75628,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":78000,"parent_record_offset":77976,"ancestor_record_offset":75628,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":503,"record_offset":76332,"parent_record_offset":76200,"ancestor_record_offset":75628,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":77924,"parent_record_offset":77864,"ancestor_record_offset":75628,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":503,"record_offset":77072,"parent_record_offset":77048,"ancestor_record_offset":75628,"inlinee":28642,"source_line":44,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::mesh_atlas_library::impl$7::get_by_world_subset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mesh_atlas_library.rs
//     evidence={"module":503,"record_offset":77396,"parent_record_offset":77372,"ancestor_record_offset":75628,"inlinee":28644,"source_line":105,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":503,"record_offset":76760,"parent_record_offset":76732,"ancestor_record_offset":75628,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":503,"record_offset":76792,"parent_record_offset":76760,"ancestor_record_offset":75628,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":503,"record_offset":76732,"parent_record_offset":76532,"ancestor_record_offset":75628,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":503,"record_offset":76592,"parent_record_offset":76532,"ancestor_record_offset":75628,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":503,"record_offset":76072,"parent_record_offset":76040,"ancestor_record_offset":75628,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":503,"record_offset":77144,"parent_record_offset":76532,"ancestor_record_offset":75628,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":503,"record_offset":76676,"parent_record_offset":76532,"ancestor_record_offset":75628,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":503,"record_offset":77268,"parent_record_offset":76532,"ancestor_record_offset":75628,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":503,"record_offset":77024,"parent_record_offset":77000,"ancestor_record_offset":75628,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":503,"record_offset":76440,"parent_record_offset":76412,"ancestor_record_offset":75628,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":503,"record_offset":77000,"parent_record_offset":76892,"ancestor_record_offset":75628,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":503,"record_offset":75908,"parent_record_offset":75872,"ancestor_record_offset":75628,"inlinee":11335,"source_line":3199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":503,"record_offset":77480,"parent_record_offset":77456,"ancestor_record_offset":75628,"inlinee":11343,"source_line":3323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":503,"record_offset":77336,"parent_record_offset":76440,"ancestor_record_offset":75628,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":77864,"parent_record_offset":77528,"ancestor_record_offset":75628,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76940,"parent_record_offset":76916,"ancestor_record_offset":75628,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":77776,"parent_record_offset":77560,"ancestor_record_offset":75628,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":77236,"parent_record_offset":76532,"ancestor_record_offset":75628,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76040,"parent_record_offset":76012,"ancestor_record_offset":75628,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":77560,"parent_record_offset":77528,"ancestor_record_offset":75628,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76916,"parent_record_offset":76892,"ancestor_record_offset":75628,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76504,"parent_record_offset":76476,"ancestor_record_offset":75628,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76476,"parent_record_offset":76440,"ancestor_record_offset":75628,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76200,"parent_record_offset":76168,"ancestor_record_offset":75628,"inlinee":11339,"source_line":1487,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76168,"parent_record_offset":75944,"ancestor_record_offset":75628,"inlinee":11338,"source_line":1473,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":75984,"parent_record_offset":75944,"ancestor_record_offset":75628,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76532,"parent_record_offset":76504,"ancestor_record_offset":75628,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76012,"parent_record_offset":75984,"ancestor_record_offset":75628,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":77296,"parent_record_offset":76532,"ancestor_record_offset":75628,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76244,"parent_record_offset":76200,"ancestor_record_offset":75628,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":75944,"parent_record_offset":75908,"ancestor_record_offset":75628,"inlinee":11337,"source_line":3530,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":77528,"parent_record_offset":77504,"ancestor_record_offset":75628,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$40::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":78072,"parent_record_offset":78048,"ancestor_record_offset":75628,"inlinee":4778,"source_line":4042,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$42::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":77504,"parent_record_offset":77480,"ancestor_record_offset":75628,"inlinee":11345,"source_line":4083,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":503,"record_offset":76892,"parent_record_offset":76532,"ancestor_record_offset":75628,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":503,"record_offset":75872,"parent_record_offset":75836,"ancestor_record_offset":75628,"inlinee":12098,"source_line":1771,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$30::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":503,"record_offset":77456,"parent_record_offset":77432,"ancestor_record_offset":75628,"inlinee":12102,"source_line":1840,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":503,"record_offset":76412,"parent_record_offset":76384,"ancestor_record_offset":75628,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00503.jsonl.gz"}
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
// 完整结构及原始类型记录: swap_replacement_mods.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::swap_replacement_mods::ModAssetReloadSystems
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 15
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::startup::swap_replacement_mods::ModAssetReloadSystems>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::startup::swap_replacement_mods::ModAssetReloadSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<country_core::startup::swap_replacement_mods::ModAssetReloadSystems>", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["country_core::startup::swap_replacement_mods::ModAssetReloadSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<country_core::startup::swap_replacement_mods::ModAssetReloadSystems>", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["country_core::startup::swap_replacement_mods::ModAssetReloadSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>,country_core::startup::swap_replacement_mods::ModAssetReloadSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "country_core::startup::swap_replacement_mods::ModAssetReloadSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>,country_core::startup::swap_replacement_mods::ModAssetReloadSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "country_core::startup::swap_replacement_mods::ModAssetReloadSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::after<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)>,tuple$<>,country_core::startup::swap_replacement_mods::ModAssetReloadSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::after", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)>", "tuple$<>", "country_core::startup::swap_replacement_mods::ModAssetReloadSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemSet::system_type<country_core::startup::swap_replacement_mods::ModAssetReloadSystems>", "constructor": "bevy_ecs::schedule::set::SystemSet::system_type", "arguments": ["country_core::startup::swap_replacement_mods::ModAssetReloadSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemSet::is_anonymous<country_core::startup::swap_replacement_mods::ModAssetReloadSystems>", "constructor": "bevy_ecs::schedule::set::SystemSet::is_anonymous", "arguments": ["country_core::startup::swap_replacement_mods::ModAssetReloadSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::swap_replacement_mods::ModPreactivationSystems
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::startup::swap_replacement_mods::ModPreactivationSystems>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::startup::swap_replacement_mods::ModPreactivationSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<country_core::startup::swap_replacement_mods::ModPreactivationSystems>", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["country_core::startup::swap_replacement_mods::ModPreactivationSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<country_core::startup::swap_replacement_mods::ModPreactivationSystems>", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["country_core::startup::swap_replacement_mods::ModPreactivationSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemSet::system_type<country_core::startup::swap_replacement_mods::ModPreactivationSystems>", "constructor": "bevy_ecs::schedule::set::SystemSet::system_type", "arguments": ["country_core::startup::swap_replacement_mods::ModPreactivationSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemSet::is_anonymous<country_core::startup::swap_replacement_mods::ModPreactivationSystems>", "constructor": "bevy_ecs::schedule::set::SystemSet::is_anonymous", "arguments": ["country_core::startup::swap_replacement_mods::ModPreactivationSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::after<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>,tuple$<>,country_core::startup::swap_replacement_mods::ModPreactivationSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::after", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "tuple$<>", "country_core::startup::swap_replacement_mods::ModPreactivationSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)>,country_core::startup::swap_replacement_mods::ModPreactivationSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)>", "country_core::startup::swap_replacement_mods::ModPreactivationSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::swap_replacement_mods::ModSwapReloads
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 48
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>,country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>,country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "tuple$<>", "void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$110::init_access<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads>", "constructor": "bevy_ecs::system::system_param::impl$110::init_access", "arguments": ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "country_core::startup::swap_replacement_mods::ModSwapReloads"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::swap_replacement_mods::ModSwapSystems
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemSet::system_type<country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "bevy_ecs::schedule::set::SystemSet::system_type", "arguments": ["country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemSet::is_anonymous<country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "bevy_ecs::schedule::set::SystemSet::is_anonymous", "arguments": ["country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::after<void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>,tuple$<>,country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::after", "arguments": ["void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::startup::mod_hot_reload::ModHotReload> > >)>", "tuple$<>", "country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>,country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>,country_core::startup::swap_replacement_mods::ModSwapSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)>", "country_core::startup::swap_replacement_mods::ModSwapSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 59
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::messages::Messages<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>", "constructor": "bevy_ecs::message::messages::Messages", "arguments": ["country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<bevy_ecs::message::messages::Messages<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd> >", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["bevy_ecs::message::messages::Messages<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>", "constructor": "bevy_ecs::message::message_reader::MessageReader", "arguments": ["country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$110::init_access<bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch> >", "constructor": "bevy_ecs::system::system_param::impl$110::init_access", "arguments": ["bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>,country_core::startup::swap_replacement_mods::ModAssetReloadSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>)>", "country_core::startup::swap_replacement_mods::ModAssetReloadSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>,country_core::startup::swap_replacement_mods::ModAssetReloadSystems>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "country_core::startup::swap_replacement_mods::ModAssetReloadSystems"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$8::init_state<bevy_ecs::message::messages::Messages<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd> >", "constructor": "bevy_ecs::system::system_param::impl$8::init_state", "arguments": ["bevy_ecs::message::messages::Messages<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>", "constructor": "bevy_ecs::message::message_writer::MessageWriter", "arguments": ["country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$118::init_access<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd> >", "constructor": "bevy_ecs::system::system_param::impl$118::init_access", "arguments": ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::message_registry::MessageRegistry::register_message<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>", "constructor": "bevy_ecs::message::message_registry::MessageRegistry::register_message", "arguments": ["country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$112::init_access<bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands>", "constructor": "bevy_ecs::system::system_param::impl$112::init_access", "arguments": ["bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::system::commands::Commands"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)", "tuple$<>", "void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::startup::swap_replacement_mods::ReplacementModsSwappedCmd>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::system::commands::Commands)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: swap_replacement_mods.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: swap_replacement_mods.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 284; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::ModSwapReloads::apply
// 信息 ID: 47c889b652feb3a3aeb60404
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::swap_replacement_mods::swap_replacement_mods @ 0x9083f0 -> 0x907470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::swap_replacement_mods::confirm_replacement_mods @ 0x908578 -> 0x907470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::_::impl$0::dyn_clone
// 信息 ID: 18164c2c228201596741c872
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::_::impl$0::get_param
// 信息 ID: ecadc9d6df5ad576ed596281
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0fc47f8b8e48a0b9fa0f1be6: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)
// [泛型宿主/闭包/直接引用方参数候选] 6423aa9a8a6bc6aed74fec54: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> @ 0x17edaf0 -> 0x17ef9a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> @ 0x17ee6f7 -> 0x17ef9a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> > @ 0x192ab15 -> 0x1903440 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> > @ 0x193ad54 -> 0x1903440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::_::impl$0::init_access
// 信息 ID: 1f121840ce445e9715642a27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0fc47f8b8e48a0b9fa0f1be6: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::initialize<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)>; module=1673, inline_record=12024; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system_param::impl$110::init_access<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads>; module=1770, inline_record=94560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::_::impl$0::init_state
// 信息 ID: c9ec67b6bcd60cf94eab53b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0fc47f8b8e48a0b9fa0f1be6: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)
// [泛型宿主/闭包/直接引用方参数候选] 6423aa9a8a6bc6aed74fec54: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::initialize<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> @ 0x17ec311 -> 0x907f00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::initialize<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> @ 0x17ed55e -> 0x907f00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::_::impl$0::validate_param
// 信息 ID: 58d24c817d0b7a8f19562e80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0fc47f8b8e48a0b9fa0f1be6: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)
// [泛型宿主/闭包/直接引用方参数候选] 6423aa9a8a6bc6aed74fec54: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> @ 0x17eee83 -> 0x17ef790 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> @ 0x17eef97 -> 0x17ef790 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> @ 0x18f8fe7 -> 0x1903230 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> > @ 0x190a9e3 -> 0x1903230 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> > @ 0x192a9ea -> 0x1903230 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> > @ 0x193ab73 -> 0x1903230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::apply_preactivated_mod_textures
// 信息 ID: e701119675feb531a052adc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 40b2e8d4f9c822da1e1b90cf: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)> @ 0x17eed04 -> 0x908620 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)> > @ 0x1949021 -> 0x908620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::confirm_replacement_mods
// 信息 ID: b3c17b419e2853cc4720d232
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0fc47f8b8e48a0b9fa0f1be6: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> @ 0x17ee775 -> 0x9084d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)> > @ 0x192aba4 -> 0x9084d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::confirm_replacement_mods::closure$0
// 信息 ID: cbd22bf9d9601ac299ff2629
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::swap_replacement_mods::confirm_replacement_mods", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::swap_replacement_mods::confirm_replacement_mods; module=503, inline_record=71596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::impl$12::fmt
// 信息 ID: d265f542b82e16a357f756c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::impl$20::fmt
// 信息 ID: 7374ee26f0e069849c30b222
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::impl$4::fmt
// 信息 ID: 6fc1ae07370f4669cdcd048a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::preactivate_boot_replacement_mods
// 信息 ID: d823d4a0c0a52e5e7c9f8ec0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 01e5ad06aacd15ca43210285: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)> @ 0x17ed8f4 -> 0x908cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)> > @ 0x190ea10 -> 0x908cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::reload_assets_for_changed_paths
// 信息 ID: dfd1ecddeff36544ab34e290
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::swap_replacement_mods::ModSwapReloads::apply @ 0x907aca -> 0x9089c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2d275 -> 0x9089c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::swap_replacement_mods::swap_replacement_mods
// 信息 ID: e1c3009d8ec82fce6a1e6444
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6423aa9a8a6bc6aed74fec54: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> @ 0x17edc50 -> 0x9082f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)> > @ 0x193aec7 -> 0x9082f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
