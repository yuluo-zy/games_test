// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/audio/audio_update.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1a14c7072adbccd47dc7bfd2216e6b48da4fb5c89f58a3bef880500aa68fbee2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: audio_update.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 567
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 首条位置: {"module":567,"record_offset":67780,"rva":11810816,"va":5380519936}
// 编译实例 RVAs: 0xb43800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a777c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-836176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e8b03df3ff","size":5,"soft_fail":false,"target_va":5379684288,"va":5380520459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12259546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e8da10bb00","size":5,"soft_fail":false,"target_va":5392780768,"va":5380521217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35864148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e8543e2302","size":5,"soft_fail":false,"target_va":5416385024,"va":5380520871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36100542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e8bed92602","size":5,"soft_fail":false,"target_va":5416621408,"va":5380520861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e8f6110000","size":5,"soft_fail":false,"target_va":5380524624,"va":5380520021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36100427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e84bd92602","size":5,"soft_fail":false,"target_va":5416621376,"va":5380520944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-835415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e8a940f3ff","size":5,"soft_fail":false,"target_va":5379685360,"va":5380520770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3443401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e8c98a3400","size":5,"soft_fail":false,"target_va":5383963792,"va":5380520386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3451905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e801ac3400","size":5,"soft_fail":false,"target_va":5383972224,"va":5380520314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3444090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e87a8d3400","size":5,"soft_fail":false,"target_va":5383964912,"va":5380520817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3442827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e88b883400","size":5,"soft_fail":false,"target_va":5383962816,"va":5380519984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36098446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380519936,"raw_hex":"e88ed12602","size":5,"soft_fail":false,"target_va":5416619360,"va":5380520909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":71500,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":70292,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":33534,"source_line":354,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":70176,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":33533,"source_line":333,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":70408,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":33531,"source_line":339,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":70556,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":33530,"source_line":348,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::update
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":70672,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":33536,"source_line":308,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":567,"record_offset":70768,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":7186,"source_line":1281,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_vector3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":567,"record_offset":71152,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":35778,"source_line":1302,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":69368,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":67968,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":68356,"parent_record_offset":67780,"ancestor_record_offset":67780,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":567,"record_offset":70232,"parent_record_offset":70176,"ancestor_record_offset":67780,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":69696,"parent_record_offset":69668,"ancestor_record_offset":67780,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":69724,"parent_record_offset":69696,"ancestor_record_offset":67780,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":69644,"parent_record_offset":69616,"ancestor_record_offset":67780,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":69668,"parent_record_offset":69644,"ancestor_record_offset":67780,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":69784,"parent_record_offset":69668,"ancestor_record_offset":67780,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":69896,"parent_record_offset":69872,"ancestor_record_offset":67780,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":567,"record_offset":70944,"parent_record_offset":70868,"ancestor_record_offset":67780,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":567,"record_offset":70836,"parent_record_offset":70812,"ancestor_record_offset":67780,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":567,"record_offset":69748,"parent_record_offset":69724,"ancestor_record_offset":67780,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":567,"record_offset":69516,"parent_record_offset":69476,"ancestor_record_offset":67780,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":567,"record_offset":70204,"parent_record_offset":70176,"ancestor_record_offset":67780,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":68304,"parent_record_offset":68276,"ancestor_record_offset":67780,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":70132,"parent_record_offset":69476,"ancestor_record_offset":67780,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":69848,"parent_record_offset":69616,"ancestor_record_offset":67780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":68776,"parent_record_offset":68752,"ancestor_record_offset":67780,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":68752,"parent_record_offset":68728,"ancestor_record_offset":67780,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":68960,"parent_record_offset":68932,"ancestor_record_offset":67780,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":567,"record_offset":69060,"parent_record_offset":69032,"ancestor_record_offset":67780,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":69032,"parent_record_offset":69004,"ancestor_record_offset":67780,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":68704,"parent_record_offset":68680,"ancestor_record_offset":67780,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":68728,"parent_record_offset":68704,"ancestor_record_offset":67780,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":69088,"parent_record_offset":69060,"ancestor_record_offset":67780,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":68680,"parent_record_offset":68564,"ancestor_record_offset":67780,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":567,"record_offset":68096,"parent_record_offset":68072,"ancestor_record_offset":67780,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":567,"record_offset":68392,"parent_record_offset":68356,"ancestor_record_offset":67780,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":567,"record_offset":68532,"parent_record_offset":68500,"ancestor_record_offset":67780,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":567,"record_offset":68500,"parent_record_offset":68464,"ancestor_record_offset":67780,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":68440,"parent_record_offset":68416,"ancestor_record_offset":67780,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":68464,"parent_record_offset":68440,"ancestor_record_offset":67780,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":68416,"parent_record_offset":68392,"ancestor_record_offset":67780,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":69004,"parent_record_offset":68464,"ancestor_record_offset":67780,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":68072,"parent_record_offset":68040,"ancestor_record_offset":67780,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":68040,"parent_record_offset":68012,"ancestor_record_offset":67780,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":68196,"parent_record_offset":68164,"ancestor_record_offset":67780,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":68252,"parent_record_offset":68196,"ancestor_record_offset":67780,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":68164,"parent_record_offset":68136,"ancestor_record_offset":67780,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":68012,"parent_record_offset":67968,"ancestor_record_offset":67780,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":68276,"parent_record_offset":68252,"ancestor_record_offset":67780,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":68564,"parent_record_offset":68532,"ancestor_record_offset":67780,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":68848,"parent_record_offset":68564,"ancestor_record_offset":67780,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":68820,"parent_record_offset":68564,"ancestor_record_offset":67780,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":68624,"parent_record_offset":68564,"ancestor_record_offset":67780,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":70376,"parent_record_offset":70292,"ancestor_record_offset":67780,"inlinee":33420,"source_line":249,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_dawn_jingle_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":70524,"parent_record_offset":70408,"ancestor_record_offset":67780,"inlinee":33414,"source_line":241,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":70260,"parent_record_offset":70176,"ancestor_record_offset":67780,"inlinee":33419,"source_line":233,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":70496,"parent_record_offset":70408,"ancestor_record_offset":67780,"inlinee":33413,"source_line":237,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":70640,"parent_record_offset":70556,"ancestor_record_offset":67780,"inlinee":33412,"source_line":245,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":567,"record_offset":70812,"parent_record_offset":70768,"ancestor_record_offset":67780,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":567,"record_offset":71064,"parent_record_offset":70768,"ancestor_record_offset":67780,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":567,"record_offset":71120,"parent_record_offset":70768,"ancestor_record_offset":67780,"inlinee":7195,"source_line":236,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":69404,"parent_record_offset":69368,"ancestor_record_offset":67780,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":71564,"parent_record_offset":71532,"ancestor_record_offset":67780,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":72004,"parent_record_offset":71800,"ancestor_record_offset":67780,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":567,"record_offset":69544,"parent_record_offset":69516,"ancestor_record_offset":67780,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":69616,"parent_record_offset":69476,"ancestor_record_offset":67780,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":69820,"parent_record_offset":69616,"ancestor_record_offset":67780,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":567,"record_offset":69572,"parent_record_offset":69544,"ancestor_record_offset":67780,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":567,"record_offset":69476,"parent_record_offset":69440,"ancestor_record_offset":67780,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":567,"record_offset":69440,"parent_record_offset":69404,"ancestor_record_offset":67780,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00567.jsonl.gz"}
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
// 首条位置: {"module":567,"record_offset":72484,"rva":11812128,"va":5380521248}
// 编译实例 RVAs: 0xb43d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a777c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-837476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e89c38f3ff","size":5,"soft_fail":false,"target_va":5379684288,"va":5380521759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12258314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e80a0cbb00","size":5,"soft_fail":false,"target_va":5392780768,"va":5380522449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: audio_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35862922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e88a392302","size":5,"soft_fail":false,"target_va":5416385024,"va":5380522097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36099316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e8f4d42602","size":5,"soft_fail":false,"target_va":5416621408,"va":5380522087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e8da0c0000","size":5,"soft_fail":false,"target_va":5380524624,"va":5380521329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36099201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e881d42602","size":5,"soft_fail":false,"target_va":5416621376,"va":5380522170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-836641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e8df3bf3ff","size":5,"soft_fail":false,"target_va":5379685360,"va":5380521996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3442105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e8b9853400","size":5,"soft_fail":false,"target_va":5383963792,"va":5380521682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3450609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e8f1a63400","size":5,"soft_fail":false,"target_va":5383972224,"va":5380521610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3442864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e8b0883400","size":5,"soft_fail":false,"target_va":5383964912,"va":5380522043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3441518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e86e833400","size":5,"soft_fail":false,"target_va":5383962816,"va":5380521293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36097220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380521248,"raw_hex":"e8c4cc2602","size":5,"soft_fail":false,"target_va":5416619360,"va":5380522135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":75468,"parent_record_offset":72484,"ancestor_record_offset":72484,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":74992,"parent_record_offset":72484,"ancestor_record_offset":72484,"inlinee":33534,"source_line":354,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":74876,"parent_record_offset":72484,"ancestor_record_offset":72484,"inlinee":33533,"source_line":333,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":75108,"parent_record_offset":72484,"ancestor_record_offset":72484,"inlinee":33531,"source_line":339,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":75256,"parent_record_offset":72484,"ancestor_record_offset":72484,"inlinee":33530,"source_line":348,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::update
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":75372,"parent_record_offset":72484,"ancestor_record_offset":72484,"inlinee":33536,"source_line":308,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":74068,"parent_record_offset":72484,"ancestor_record_offset":72484,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":72668,"parent_record_offset":72484,"ancestor_record_offset":72484,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":73056,"parent_record_offset":72484,"ancestor_record_offset":72484,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":567,"record_offset":74932,"parent_record_offset":74876,"ancestor_record_offset":72484,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":74396,"parent_record_offset":74368,"ancestor_record_offset":72484,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":74424,"parent_record_offset":74396,"ancestor_record_offset":72484,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":74344,"parent_record_offset":74316,"ancestor_record_offset":72484,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":74368,"parent_record_offset":74344,"ancestor_record_offset":72484,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":74484,"parent_record_offset":74368,"ancestor_record_offset":72484,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":567,"record_offset":74596,"parent_record_offset":74572,"ancestor_record_offset":72484,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":567,"record_offset":74448,"parent_record_offset":74424,"ancestor_record_offset":72484,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":567,"record_offset":74216,"parent_record_offset":74176,"ancestor_record_offset":72484,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":567,"record_offset":74904,"parent_record_offset":74876,"ancestor_record_offset":72484,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":73004,"parent_record_offset":72976,"ancestor_record_offset":72484,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":74832,"parent_record_offset":74176,"ancestor_record_offset":72484,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":74548,"parent_record_offset":74316,"ancestor_record_offset":72484,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":73476,"parent_record_offset":73452,"ancestor_record_offset":72484,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":73452,"parent_record_offset":73428,"ancestor_record_offset":72484,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":73660,"parent_record_offset":73632,"ancestor_record_offset":72484,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":567,"record_offset":73760,"parent_record_offset":73732,"ancestor_record_offset":72484,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":73732,"parent_record_offset":73704,"ancestor_record_offset":72484,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":73404,"parent_record_offset":73380,"ancestor_record_offset":72484,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":73428,"parent_record_offset":73404,"ancestor_record_offset":72484,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":73788,"parent_record_offset":73760,"ancestor_record_offset":72484,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":73380,"parent_record_offset":73264,"ancestor_record_offset":72484,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":567,"record_offset":72796,"parent_record_offset":72772,"ancestor_record_offset":72484,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":567,"record_offset":73092,"parent_record_offset":73056,"ancestor_record_offset":72484,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":567,"record_offset":73232,"parent_record_offset":73200,"ancestor_record_offset":72484,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":567,"record_offset":73200,"parent_record_offset":73164,"ancestor_record_offset":72484,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":73140,"parent_record_offset":73116,"ancestor_record_offset":72484,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":73164,"parent_record_offset":73140,"ancestor_record_offset":72484,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":73116,"parent_record_offset":73092,"ancestor_record_offset":72484,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":567,"record_offset":73704,"parent_record_offset":73164,"ancestor_record_offset":72484,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":72772,"parent_record_offset":72740,"ancestor_record_offset":72484,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":72740,"parent_record_offset":72712,"ancestor_record_offset":72484,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":72896,"parent_record_offset":72864,"ancestor_record_offset":72484,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":72952,"parent_record_offset":72896,"ancestor_record_offset":72484,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":72864,"parent_record_offset":72836,"ancestor_record_offset":72484,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":72712,"parent_record_offset":72668,"ancestor_record_offset":72484,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":567,"record_offset":72976,"parent_record_offset":72952,"ancestor_record_offset":72484,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":73264,"parent_record_offset":73232,"ancestor_record_offset":72484,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":73548,"parent_record_offset":73264,"ancestor_record_offset":72484,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":73520,"parent_record_offset":73264,"ancestor_record_offset":72484,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":567,"record_offset":73324,"parent_record_offset":73264,"ancestor_record_offset":72484,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_ambience_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":75076,"parent_record_offset":74992,"ancestor_record_offset":72484,"inlinee":33420,"source_line":249,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_dawn_jingle_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":75224,"parent_record_offset":75108,"ancestor_record_offset":72484,"inlinee":33414,"source_line":241,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_master_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":74960,"parent_record_offset":74876,"ancestor_record_offset":72484,"inlinee":33419,"source_line":233,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_music_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":75196,"parent_record_offset":75108,"ancestor_record_offset":72484,"inlinee":33413,"source_line":237,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioCoreInner::set_sfx_volume
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":567,"record_offset":75340,"parent_record_offset":75256,"ancestor_record_offset":72484,"inlinee":33412,"source_line":245,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":74104,"parent_record_offset":74068,"ancestor_record_offset":72484,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":75532,"parent_record_offset":75500,"ancestor_record_offset":72484,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":75972,"parent_record_offset":75768,"ancestor_record_offset":72484,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":567,"record_offset":74244,"parent_record_offset":74216,"ancestor_record_offset":72484,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":74316,"parent_record_offset":74176,"ancestor_record_offset":72484,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":567,"record_offset":74520,"parent_record_offset":74316,"ancestor_record_offset":72484,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":567,"record_offset":74272,"parent_record_offset":74244,"ancestor_record_offset":72484,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":567,"record_offset":74176,"parent_record_offset":74140,"ancestor_record_offset":72484,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":567,"record_offset":74140,"parent_record_offset":74104,"ancestor_record_offset":72484,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00567.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// 信息 ID: d22d424a4f3ec7db4e4a6835
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b1e21fd0d013a3dd5ca06110: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > @ 0x19299a6 -> 0xb43800 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> @ 0x1c27a1c -> 0xb43800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::audio_update::boot_audio_update_system
// 信息 ID: 20e1096ddcef4a7de15c25ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cdfe10352a97cf17480ce5fb: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > @ 0x2a67a8 -> 0xb43d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> @ 0x592618 -> 0xb43d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
