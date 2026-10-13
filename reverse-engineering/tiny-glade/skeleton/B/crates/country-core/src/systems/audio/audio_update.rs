// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/audio/audio_update.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1a14c7072adbccd47dc7bfd2216e6b48da4fb5c89f58a3bef880500aa68fbee2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: audio_update.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 530
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::audio_update::audio_update_system
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":530,"record_offset":61508,"rva":10259920,"va":5378969040}
// 编译实例 RVAs: 0x9c8dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a54810
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t571298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e8a2b70800","size":5,"soft_fail":false,"target_va":5379541008,"va":5378969705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32699853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e8cdf5f201","size":5,"soft_fail":false,"target_va":5411669968,"va":5378970110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32571411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e81300f101","size":5,"soft_fail":false,"target_va":5411541584,"va":5378970168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e84afbffff","size":5,"soft_fail":false,"target_va":5378967920,"va":5378969121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32574164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e8d40af101","size":5,"soft_fail":false,"target_va":5411543360,"va":5378969191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t567392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e860a80800","size":5,"soft_fail":false,"target_va":5379537408,"va":5378970011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2415898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e81add2400","size":5,"soft_fail":false,"target_va":5381385536,"va":5378969633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2421084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e85cf12400","size":5,"soft_fail":false,"target_va":5381390640,"va":5378969551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2416408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e818df2400","size":5,"soft_fail":false,"target_va":5381386480,"va":5378970067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2415611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e8fbdb2400","size":5,"soft_fail":false,"target_va":5381384704,"va":5378969088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9854922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e8ca5f9600","size":5,"soft_fail":false,"target_va":5388825296,"va":5378970369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32566943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378969040,"raw_hex":"e89feef001","size":5,"soft_fail":false,"target_va":5411537104,"va":5378970156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":65436,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":64228,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":30313,"source_line":354,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":64112,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":30310,"source_line":333,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":64344,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":30315,"source_line":339,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":64492,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":30318,"source_line":348,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::update
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":64608,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":30320,"source_line":308,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":530,"record_offset":64704,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":29550,"source_line":1151,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_vector3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":530,"record_offset":65060,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":30321,"source_line":1172,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":63128,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":61696,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":62152,"parent_record_offset":61508,"ancestor_record_offset":61508,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":530,"record_offset":64168,"parent_record_offset":64112,"ancestor_record_offset":61508,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":63464,"parent_record_offset":63436,"ancestor_record_offset":61508,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":63516,"parent_record_offset":63492,"ancestor_record_offset":61508,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":63492,"parent_record_offset":63464,"ancestor_record_offset":61508,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":63412,"parent_record_offset":63384,"ancestor_record_offset":61508,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":63436,"parent_record_offset":63412,"ancestor_record_offset":61508,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":63608,"parent_record_offset":63436,"ancestor_record_offset":61508,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":63720,"parent_record_offset":63696,"ancestor_record_offset":61508,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":530,"record_offset":64852,"parent_record_offset":64796,"ancestor_record_offset":61508,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":530,"record_offset":64764,"parent_record_offset":64740,"ancestor_record_offset":61508,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":530,"record_offset":63540,"parent_record_offset":63516,"ancestor_record_offset":61508,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":530,"record_offset":61852,"parent_record_offset":61820,"ancestor_record_offset":61508,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":530,"record_offset":63280,"parent_record_offset":63236,"ancestor_record_offset":61508,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":64140,"parent_record_offset":64112,"ancestor_record_offset":61508,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":530,"record_offset":62100,"parent_record_offset":62072,"ancestor_record_offset":61508,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":530,"record_offset":64068,"parent_record_offset":63236,"ancestor_record_offset":61508,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":63672,"parent_record_offset":63384,"ancestor_record_offset":61508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":62564,"parent_record_offset":62540,"ancestor_record_offset":61508,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":62540,"parent_record_offset":62516,"ancestor_record_offset":61508,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":62748,"parent_record_offset":62720,"ancestor_record_offset":61508,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":63564,"parent_record_offset":63540,"ancestor_record_offset":61508,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":62492,"parent_record_offset":62468,"ancestor_record_offset":61508,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":62516,"parent_record_offset":62492,"ancestor_record_offset":61508,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":62820,"parent_record_offset":62792,"ancestor_record_offset":61508,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":62468,"parent_record_offset":62352,"ancestor_record_offset":61508,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":530,"record_offset":61820,"parent_record_offset":61792,"ancestor_record_offset":61508,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":530,"record_offset":62180,"parent_record_offset":62152,"ancestor_record_offset":61508,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":530,"record_offset":62320,"parent_record_offset":62288,"ancestor_record_offset":61508,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":530,"record_offset":62288,"parent_record_offset":62252,"ancestor_record_offset":61508,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":62228,"parent_record_offset":62204,"ancestor_record_offset":61508,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":62252,"parent_record_offset":62228,"ancestor_record_offset":61508,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":62204,"parent_record_offset":62180,"ancestor_record_offset":61508,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":62792,"parent_record_offset":62252,"ancestor_record_offset":61508,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":61792,"parent_record_offset":61760,"ancestor_record_offset":61508,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":61760,"parent_record_offset":61732,"ancestor_record_offset":61508,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":61952,"parent_record_offset":61920,"ancestor_record_offset":61508,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":62048,"parent_record_offset":61952,"ancestor_record_offset":61508,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":61920,"parent_record_offset":61896,"ancestor_record_offset":61508,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":61732,"parent_record_offset":61696,"ancestor_record_offset":61508,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":62072,"parent_record_offset":62048,"ancestor_record_offset":61508,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":62352,"parent_record_offset":62320,"ancestor_record_offset":61508,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":62636,"parent_record_offset":62352,"ancestor_record_offset":61508,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":62608,"parent_record_offset":62352,"ancestor_record_offset":61508,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":62412,"parent_record_offset":62352,"ancestor_record_offset":61508,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":64312,"parent_record_offset":64228,"ancestor_record_offset":61508,"inlinee":30314,"source_line":249,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_dawn_jingle_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":64460,"parent_record_offset":64344,"ancestor_record_offset":61508,"inlinee":30317,"source_line":241,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":64196,"parent_record_offset":64112,"ancestor_record_offset":61508,"inlinee":30312,"source_line":233,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":64432,"parent_record_offset":64344,"ancestor_record_offset":61508,"inlinee":30316,"source_line":237,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":64576,"parent_record_offset":64492,"ancestor_record_offset":61508,"inlinee":30319,"source_line":245,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":530,"record_offset":64740,"parent_record_offset":64704,"ancestor_record_offset":61508,"inlinee":25141,"source_line":1167,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":530,"record_offset":64972,"parent_record_offset":64704,"ancestor_record_offset":61508,"inlinee":28544,"source_line":1245,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":530,"record_offset":65028,"parent_record_offset":64704,"ancestor_record_offset":61508,"inlinee":30322,"source_line":199,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":63164,"parent_record_offset":63128,"ancestor_record_offset":61508,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":65492,"parent_record_offset":65464,"ancestor_record_offset":61508,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":65980,"parent_record_offset":65720,"ancestor_record_offset":61508,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":530,"record_offset":63308,"parent_record_offset":63280,"ancestor_record_offset":61508,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":63384,"parent_record_offset":63236,"ancestor_record_offset":61508,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":63644,"parent_record_offset":63384,"ancestor_record_offset":61508,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":530,"record_offset":63336,"parent_record_offset":63308,"ancestor_record_offset":61508,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":530,"record_offset":63236,"parent_record_offset":63200,"ancestor_record_offset":61508,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":530,"record_offset":63200,"parent_record_offset":63164,"ancestor_record_offset":61508,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::audio_update::boot_audio_update_system
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":530,"record_offset":66572,"rva":10261280,"va":5378970400}
// 编译实例 RVAs: 0x9c9320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a54810
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t569934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e84eb20800","size":5,"soft_fail":false,"target_va":5379541008,"va":5378971069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32698563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e8c3f0f201","size":5,"soft_fail":false,"target_va":5411669968,"va":5378971400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32570121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e809fbf001","size":5,"soft_fail":false,"target_va":5411541584,"va":5378971458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e8fef5ffff","size":5,"soft_fail":false,"target_va":5378967920,"va":5378970477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32572808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e88805f101","size":5,"soft_fail":false,"target_va":5411543360,"va":5378970547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t566102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e856a30800","size":5,"soft_fail":false,"target_va":5379537408,"va":5378971301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2414538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e8cad72400","size":5,"soft_fail":false,"target_va":5381385536,"va":5378970993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2419724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e80cec2400","size":5,"soft_fail":false,"target_va":5381390640,"va":5378970911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2415118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e80eda2400","size":5,"soft_fail":false,"target_va":5381386480,"va":5378971357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2414254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e8aed62400","size":5,"soft_fail":false,"target_va":5381384704,"va":5378970445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9853626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e8ba5a9600","size":5,"soft_fail":false,"target_va":5388825296,"va":5378971665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32565653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378970400,"raw_hex":"e895e9f001","size":5,"soft_fail":false,"target_va":5411537104,"va":5378971446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":69764,"parent_record_offset":66572,"ancestor_record_offset":66572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":69288,"parent_record_offset":66572,"ancestor_record_offset":66572,"inlinee":30313,"source_line":354,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":69172,"parent_record_offset":66572,"ancestor_record_offset":66572,"inlinee":30310,"source_line":333,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":69404,"parent_record_offset":66572,"ancestor_record_offset":66572,"inlinee":30315,"source_line":339,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":69552,"parent_record_offset":66572,"ancestor_record_offset":66572,"inlinee":30318,"source_line":348,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::update
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":69668,"parent_record_offset":66572,"ancestor_record_offset":66572,"inlinee":30320,"source_line":308,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":68188,"parent_record_offset":66572,"ancestor_record_offset":66572,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":66756,"parent_record_offset":66572,"ancestor_record_offset":66572,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":67212,"parent_record_offset":66572,"ancestor_record_offset":66572,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":530,"record_offset":69228,"parent_record_offset":69172,"ancestor_record_offset":66572,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":68524,"parent_record_offset":68496,"ancestor_record_offset":66572,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":68576,"parent_record_offset":68552,"ancestor_record_offset":66572,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":68552,"parent_record_offset":68524,"ancestor_record_offset":66572,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":68472,"parent_record_offset":68444,"ancestor_record_offset":66572,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":68496,"parent_record_offset":68472,"ancestor_record_offset":66572,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":68668,"parent_record_offset":68496,"ancestor_record_offset":66572,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":530,"record_offset":68780,"parent_record_offset":68756,"ancestor_record_offset":66572,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":530,"record_offset":68600,"parent_record_offset":68576,"ancestor_record_offset":66572,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":530,"record_offset":66912,"parent_record_offset":66880,"ancestor_record_offset":66572,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":530,"record_offset":68340,"parent_record_offset":68296,"ancestor_record_offset":66572,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":530,"record_offset":69200,"parent_record_offset":69172,"ancestor_record_offset":66572,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":530,"record_offset":67160,"parent_record_offset":67132,"ancestor_record_offset":66572,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":530,"record_offset":69128,"parent_record_offset":68296,"ancestor_record_offset":66572,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":68732,"parent_record_offset":68444,"ancestor_record_offset":66572,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":67624,"parent_record_offset":67600,"ancestor_record_offset":66572,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":67600,"parent_record_offset":67576,"ancestor_record_offset":66572,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":530,"record_offset":67808,"parent_record_offset":67780,"ancestor_record_offset":66572,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":68624,"parent_record_offset":68600,"ancestor_record_offset":66572,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":67552,"parent_record_offset":67528,"ancestor_record_offset":66572,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":67576,"parent_record_offset":67552,"ancestor_record_offset":66572,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":67880,"parent_record_offset":67852,"ancestor_record_offset":66572,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":530,"record_offset":67528,"parent_record_offset":67412,"ancestor_record_offset":66572,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":530,"record_offset":66880,"parent_record_offset":66852,"ancestor_record_offset":66572,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":530,"record_offset":67240,"parent_record_offset":67212,"ancestor_record_offset":66572,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":530,"record_offset":67380,"parent_record_offset":67348,"ancestor_record_offset":66572,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":530,"record_offset":67348,"parent_record_offset":67312,"ancestor_record_offset":66572,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":67288,"parent_record_offset":67264,"ancestor_record_offset":66572,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":67312,"parent_record_offset":67288,"ancestor_record_offset":66572,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":67264,"parent_record_offset":67240,"ancestor_record_offset":66572,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":530,"record_offset":67852,"parent_record_offset":67312,"ancestor_record_offset":66572,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":66852,"parent_record_offset":66820,"ancestor_record_offset":66572,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":66820,"parent_record_offset":66792,"ancestor_record_offset":66572,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":67012,"parent_record_offset":66980,"ancestor_record_offset":66572,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":67108,"parent_record_offset":67012,"ancestor_record_offset":66572,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":66980,"parent_record_offset":66956,"ancestor_record_offset":66572,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":66792,"parent_record_offset":66756,"ancestor_record_offset":66572,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":67132,"parent_record_offset":67108,"ancestor_record_offset":66572,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":67412,"parent_record_offset":67380,"ancestor_record_offset":66572,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":67696,"parent_record_offset":67412,"ancestor_record_offset":66572,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":67668,"parent_record_offset":67412,"ancestor_record_offset":66572,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":530,"record_offset":67472,"parent_record_offset":67412,"ancestor_record_offset":66572,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":69372,"parent_record_offset":69288,"ancestor_record_offset":66572,"inlinee":30314,"source_line":249,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_dawn_jingle_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":69520,"parent_record_offset":69404,"ancestor_record_offset":66572,"inlinee":30317,"source_line":241,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":69256,"parent_record_offset":69172,"ancestor_record_offset":66572,"inlinee":30312,"source_line":233,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":69492,"parent_record_offset":69404,"ancestor_record_offset":66572,"inlinee":30316,"source_line":237,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":530,"record_offset":69636,"parent_record_offset":69552,"ancestor_record_offset":66572,"inlinee":30319,"source_line":245,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":68224,"parent_record_offset":68188,"ancestor_record_offset":66572,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":69820,"parent_record_offset":69792,"ancestor_record_offset":66572,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":70308,"parent_record_offset":70048,"ancestor_record_offset":66572,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":530,"record_offset":68368,"parent_record_offset":68340,"ancestor_record_offset":66572,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":68444,"parent_record_offset":68296,"ancestor_record_offset":66572,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":530,"record_offset":68704,"parent_record_offset":68444,"ancestor_record_offset":66572,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":530,"record_offset":68396,"parent_record_offset":68368,"ancestor_record_offset":66572,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":530,"record_offset":68296,"parent_record_offset":68260,"ancestor_record_offset":66572,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":530,"record_offset":68260,"parent_record_offset":68224,"ancestor_record_offset":66572,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00530.jsonl.gz"}
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
// 完整结构及原始类型记录: audio_update.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: audio_update.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: audio_update.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::audio_update::audio_update_system
// 信息 ID: 614863c2d95466c1c375333e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b1e21fd0d013a3dd5ca06110: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > > @ 0x14ce7c0 -> 0x9c8dd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > @ 0x1556785 -> 0x9c8dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::audio_update::boot_audio_update_system
// 信息 ID: 6c556fe8c4cfdb768e95a77d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cdfe10352a97cf17480ce5fb: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > @ 0x2aef8 -> 0x9c9320 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > > @ 0x4430aa -> 0x9c9320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
