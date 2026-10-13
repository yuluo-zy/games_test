// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/world_raster.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a0e27601724de31ec44d34bc082ac4a375c60190c09be821bdcf0b72c5691453
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: world_raster.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 38, 77, 89, 254, 470, 493, 502, 517, 518, 549, 560, 571, 581, 585, 588, 593, 596, 603, 604, 609, 1232, 1481, 1622, 1686, 1698, 1700, 1719, 1745, 1803, 1804, 1806, 1842, 2587, 2630, 2836, 2862, 3273, 3875, 3882, 3895, 3901, 3929, 4054, 4064, 4065, 4153, 4155, 4164, 4165, 4168, 4176, 4729, 4742, 4810, 5000
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use ahash; // namespace candidate
//   package identities: [{"name":"ahash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.11"}]
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tabloid; // namespace candidate
//   package identities: [{"name":"tabloid","source":"local_or_path","version":"0.1.0"}]
// use zeno; // namespace candidate
//   package identities: [{"name":"zeno","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f","version":"0.2.2"},{"name":"zeno","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::clear_buffer<512>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":21312,"rva":13136256,"va":5381845376}
// 编译实例 RVAs: 0xc87180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34539054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845376,"raw_hex":"e82e060f02","size":5,"soft_fail":false,"target_va":5416385024,"va":5381845965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35211203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845376,"raw_hex":"e8c3471902","size":5,"soft_fail":false,"target_va":5417057088,"va":5381845880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34775448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845376,"raw_hex":"e898a11202","size":5,"soft_fail":false,"target_va":5416621408,"va":5381845955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845376,"raw_hex":"e8da260000","size":5,"soft_fail":false,"target_va":5381855392,"va":5381845441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34775333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845376,"raw_hex":"e825a11202","size":5,"soft_fail":false,"target_va":5416621376,"va":5381846038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2117957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845376,"raw_hex":"e845512000","size":5,"soft_fail":false,"target_va":5383963792,"va":5381845830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2126461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845376,"raw_hex":"e87d722000","size":5,"soft_fail":false,"target_va":5383972224,"va":5381845758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2118987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845376,"raw_hex":"e84b552000","size":5,"soft_fail":false,"target_va":5383964912,"va":5381845920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2117403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845376,"raw_hex":"e81b4f2000","size":5,"soft_fail":false,"target_va":5383962816,"va":5381845408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34773352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845376,"raw_hex":"e868991202","size":5,"soft_fail":false,"target_va":5416619360,"va":5381846003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t35211365","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381845376,"raw_hex":"e965481902","size":5,"soft_fail":false,"target_va":5417057088,"va":5381845718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":23728,"parent_record_offset":21312,"ancestor_record_offset":21312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":22868,"parent_record_offset":21312,"ancestor_record_offset":21312,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":22932,"parent_record_offset":21312,"ancestor_record_offset":21312,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":21476,"parent_record_offset":21312,"ancestor_record_offset":21312,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":21856,"parent_record_offset":21312,"ancestor_record_offset":21312,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":23248,"parent_record_offset":23220,"ancestor_record_offset":21312,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":23276,"parent_record_offset":23248,"ancestor_record_offset":21312,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":23196,"parent_record_offset":23168,"ancestor_record_offset":21312,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":23220,"parent_record_offset":23196,"ancestor_record_offset":21312,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":23336,"parent_record_offset":23220,"ancestor_record_offset":21312,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":23448,"parent_record_offset":23424,"ancestor_record_offset":21312,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":609,"record_offset":23300,"parent_record_offset":23276,"ancestor_record_offset":21312,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":609,"record_offset":23068,"parent_record_offset":23028,"ancestor_record_offset":21312,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":21804,"parent_record_offset":21780,"ancestor_record_offset":21312,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":23684,"parent_record_offset":23028,"ancestor_record_offset":21312,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":23400,"parent_record_offset":23168,"ancestor_record_offset":21312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":22276,"parent_record_offset":22252,"ancestor_record_offset":21312,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":22252,"parent_record_offset":22228,"ancestor_record_offset":21312,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":22460,"parent_record_offset":22432,"ancestor_record_offset":21312,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":609,"record_offset":22560,"parent_record_offset":22532,"ancestor_record_offset":21312,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":22532,"parent_record_offset":22504,"ancestor_record_offset":21312,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":21312,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":22228,"parent_record_offset":22204,"ancestor_record_offset":21312,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":22588,"parent_record_offset":22560,"ancestor_record_offset":21312,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":22180,"parent_record_offset":22064,"ancestor_record_offset":21312,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$2::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":609,"record_offset":22896,"parent_record_offset":22868,"ancestor_record_offset":21312,"inlinee":19477,"source_line":31,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":21604,"parent_record_offset":21580,"ancestor_record_offset":21312,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":21892,"parent_record_offset":21856,"ancestor_record_offset":21312,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":22032,"parent_record_offset":22000,"ancestor_record_offset":21312,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":22000,"parent_record_offset":21964,"ancestor_record_offset":21312,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":21940,"parent_record_offset":21916,"ancestor_record_offset":21312,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":21964,"parent_record_offset":21940,"ancestor_record_offset":21312,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":21916,"parent_record_offset":21892,"ancestor_record_offset":21312,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":22504,"parent_record_offset":21964,"ancestor_record_offset":21312,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":21580,"parent_record_offset":21548,"ancestor_record_offset":21312,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":21548,"parent_record_offset":21520,"ancestor_record_offset":21312,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":21704,"parent_record_offset":21672,"ancestor_record_offset":21312,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":21756,"parent_record_offset":21704,"ancestor_record_offset":21312,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":21672,"parent_record_offset":21644,"ancestor_record_offset":21312,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":21520,"parent_record_offset":21476,"ancestor_record_offset":21312,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":21780,"parent_record_offset":21756,"ancestor_record_offset":21312,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":22064,"parent_record_offset":22032,"ancestor_record_offset":21312,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":22348,"parent_record_offset":22064,"ancestor_record_offset":21312,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":22320,"parent_record_offset":22064,"ancestor_record_offset":21312,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":22124,"parent_record_offset":22064,"ancestor_record_offset":21312,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":22964,"parent_record_offset":22932,"ancestor_record_offset":21312,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":23792,"parent_record_offset":23760,"ancestor_record_offset":21312,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":24232,"parent_record_offset":24028,"ancestor_record_offset":21312,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":609,"record_offset":23096,"parent_record_offset":23068,"ancestor_record_offset":21312,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":23168,"parent_record_offset":23028,"ancestor_record_offset":21312,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":23372,"parent_record_offset":23168,"ancestor_record_offset":21312,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":609,"record_offset":23124,"parent_record_offset":23096,"ancestor_record_offset":21312,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":23028,"parent_record_offset":22996,"ancestor_record_offset":21312,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":22996,"parent_record_offset":22964,"ancestor_record_offset":21312,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":34408,"rva":13138448,"va":5381847568}
// 编译实例 RVAs: 0xc87a10, 0x285a290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6169599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e801dca1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381848282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-6170254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e872d9a1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381848953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13126076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e844b637ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381848263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10931267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e843cca600","size":5,"soft_fail":false,"target_va":5392780768,"va":5381849496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a74110
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14598588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8bcc1de00","size":5,"soft_fail":false,"target_va":5396447504,"va":5381848911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a748e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14600613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8a5c9de00","size":5,"soft_fail":false,"target_va":5396449504,"va":5381848886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a750b0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14602643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e893d1de00","size":5,"soft_fail":false,"target_va":5396451504,"va":5381848856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34535968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e820fa0e02","size":5,"soft_fail":false,"target_va":5416385024,"va":5381849051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34939728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e850231502","size":5,"soft_fail":false,"target_va":5416788096,"va":5381848363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34772542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e83e961202","size":5,"soft_fail":false,"target_va":5416620794,"va":5381848247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34772362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e88a951202","size":5,"soft_fail":false,"target_va":5416621408,"va":5381849041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34772466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8f2951202","size":5,"soft_fail":false,"target_va":5416621600,"va":5381849129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34773117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e87d981202","size":5,"soft_fail":false,"target_va":5416622208,"va":5381849086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8ad1d0000","size":5,"soft_fail":false,"target_va":5381855392,"va":5381847790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t34772204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8ec941202","size":5,"soft_fail":false,"target_va":5416621376,"va":5381849167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2115632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e830482000","size":5,"soft_fail":false,"target_va":5383963792,"va":5381848155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2124139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e86b692000","size":5,"soft_fail":false,"target_va":5383972224,"va":5381848080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2115894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e836492000","size":5,"soft_fail":false,"target_va":5383964912,"va":5381849013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2115068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8fc452000","size":5,"soft_fail":false,"target_va":5383962816,"va":5381847743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6001648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411021456,"raw_hex":"e8f0935b00","size":5,"soft_fail":false,"target_va":5417024592,"va":5411022939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411021456,"raw_hex":"e8c30c0000","size":5,"soft_fail":false,"target_va":5411026656,"va":5411023384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34770239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e83f8d1202","size":5,"soft_fail":false,"target_va":5416619360,"va":5381849116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::mask::Mask$LT$D$GT$::blend_into
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     evidence={"assembly":"\tcallq\t2761219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411021456,"raw_hex":"e803222a00","size":5,"soft_fail":false,"target_va":5413783968,"va":5411022744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":37712,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":37124,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":609,"record_offset":38672,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":609,"record_offset":37544,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":609,"record_offset":34776,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":38823,"source_line":174,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":609,"record_offset":39864,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":38704,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::ws_to_px
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":609,"record_offset":37628,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":38824,"source_line":203,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":36316,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":34916,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":35304,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::format
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":609,"record_offset":38584,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":34265,"source_line":120,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":609,"record_offset":38556,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":38825,"source_line":69,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::size
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":609,"record_offset":38640,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":38826,"source_line":140,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::style
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":609,"record_offset":38612,"parent_record_offset":34408,"ancestor_record_offset":34408,"inlinee":34266,"source_line":107,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":37436,"parent_record_offset":37412,"ancestor_record_offset":34408,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":37460,"parent_record_offset":37436,"ancestor_record_offset":34408,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":38892,"parent_record_offset":38868,"ancestor_record_offset":34408,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":38916,"parent_record_offset":38892,"ancestor_record_offset":34408,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":37484,"parent_record_offset":37460,"ancestor_record_offset":34408,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":38940,"parent_record_offset":38916,"ancestor_record_offset":34408,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":37412,"parent_record_offset":37228,"ancestor_record_offset":34408,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":38868,"parent_record_offset":38776,"ancestor_record_offset":34408,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":37800,"parent_record_offset":37776,"ancestor_record_offset":34408,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":37172,"parent_record_offset":37148,"ancestor_record_offset":34408,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":38804,"parent_record_offset":38776,"ancestor_record_offset":34408,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":38776,"parent_record_offset":38752,"ancestor_record_offset":34408,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":37848,"parent_record_offset":37824,"ancestor_record_offset":34408,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":37824,"parent_record_offset":37800,"ancestor_record_offset":34408,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":37228,"parent_record_offset":37196,"ancestor_record_offset":34408,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":37196,"parent_record_offset":37172,"ancestor_record_offset":34408,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":38752,"parent_record_offset":38728,"ancestor_record_offset":34408,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":37268,"parent_record_offset":37228,"ancestor_record_offset":34408,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":37776,"parent_record_offset":37740,"ancestor_record_offset":34408,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":37740,"parent_record_offset":37712,"ancestor_record_offset":34408,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":37148,"parent_record_offset":37124,"ancestor_record_offset":34408,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":609,"record_offset":37292,"parent_record_offset":37268,"ancestor_record_offset":34408,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":36644,"parent_record_offset":36616,"ancestor_record_offset":34408,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":36672,"parent_record_offset":36644,"ancestor_record_offset":34408,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":36592,"parent_record_offset":36564,"ancestor_record_offset":34408,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":36616,"parent_record_offset":36592,"ancestor_record_offset":34408,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":36732,"parent_record_offset":36616,"ancestor_record_offset":34408,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":36844,"parent_record_offset":36820,"ancestor_record_offset":34408,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":609,"record_offset":37372,"parent_record_offset":37316,"ancestor_record_offset":34408,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":609,"record_offset":36696,"parent_record_offset":36672,"ancestor_record_offset":34408,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":609,"record_offset":37316,"parent_record_offset":37292,"ancestor_record_offset":34408,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":609,"record_offset":37344,"parent_record_offset":37316,"ancestor_record_offset":34408,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":609,"record_offset":38836,"parent_record_offset":38804,"ancestor_record_offset":34408,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":38464,"parent_record_offset":38440,"ancestor_record_offset":34408,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":38520,"parent_record_offset":38496,"ancestor_record_offset":34408,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":37676,"parent_record_offset":37652,"ancestor_record_offset":34408,"inlinee":14581,"source_line":985,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":609,"record_offset":36464,"parent_record_offset":36424,"ancestor_record_offset":34408,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":35252,"parent_record_offset":35224,"ancestor_record_offset":34408,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":37080,"parent_record_offset":36424,"ancestor_record_offset":34408,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":34872,"parent_record_offset":34848,"ancestor_record_offset":34408,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":36796,"parent_record_offset":36564,"ancestor_record_offset":34408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":609,"record_offset":34848,"parent_record_offset":34824,"ancestor_record_offset":34408,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":38204,"parent_record_offset":38180,"ancestor_record_offset":34408,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":35724,"parent_record_offset":35700,"ancestor_record_offset":34408,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":34824,"parent_record_offset":34800,"ancestor_record_offset":34408,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":35700,"parent_record_offset":35676,"ancestor_record_offset":34408,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":35908,"parent_record_offset":35880,"ancestor_record_offset":34408,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":37888,"parent_record_offset":37740,"ancestor_record_offset":34408,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":609,"record_offset":36008,"parent_record_offset":35980,"ancestor_record_offset":34408,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":35980,"parent_record_offset":35952,"ancestor_record_offset":34408,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":35652,"parent_record_offset":35628,"ancestor_record_offset":34408,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":35676,"parent_record_offset":35652,"ancestor_record_offset":34408,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":34800,"parent_record_offset":34776,"ancestor_record_offset":34408,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":36036,"parent_record_offset":36008,"ancestor_record_offset":34408,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":37568,"parent_record_offset":37544,"ancestor_record_offset":34408,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":35628,"parent_record_offset":35512,"ancestor_record_offset":34408,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":35044,"parent_record_offset":35020,"ancestor_record_offset":34408,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":35340,"parent_record_offset":35304,"ancestor_record_offset":34408,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":35480,"parent_record_offset":35448,"ancestor_record_offset":34408,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":35448,"parent_record_offset":35412,"ancestor_record_offset":34408,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":35388,"parent_record_offset":35364,"ancestor_record_offset":34408,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":35412,"parent_record_offset":35388,"ancestor_record_offset":34408,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":35364,"parent_record_offset":35340,"ancestor_record_offset":34408,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":35952,"parent_record_offset":35412,"ancestor_record_offset":34408,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":35020,"parent_record_offset":34988,"ancestor_record_offset":34408,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":34988,"parent_record_offset":34960,"ancestor_record_offset":34408,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":35144,"parent_record_offset":35112,"ancestor_record_offset":34408,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":35200,"parent_record_offset":35144,"ancestor_record_offset":34408,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":35112,"parent_record_offset":35084,"ancestor_record_offset":34408,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":34960,"parent_record_offset":34916,"ancestor_record_offset":34408,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":35224,"parent_record_offset":35200,"ancestor_record_offset":34408,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":35512,"parent_record_offset":35480,"ancestor_record_offset":34408,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":35796,"parent_record_offset":35512,"ancestor_record_offset":34408,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":35768,"parent_record_offset":35512,"ancestor_record_offset":34408,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":35572,"parent_record_offset":35512,"ancestor_record_offset":34408,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$22::mul_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":38496,"parent_record_offset":38352,"ancestor_record_offset":34408,"inlinee":30716,"source_line":1293,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$44::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":38440,"parent_record_offset":38352,"ancestor_record_offset":34408,"inlinee":34262,"source_line":1478,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$6::div_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":37652,"parent_record_offset":37628,"ancestor_record_offset":34408,"inlinee":34260,"source_line":1158,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":36352,"parent_record_offset":36316,"ancestor_record_offset":34408,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":39060,"parent_record_offset":39028,"ancestor_record_offset":34408,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":39500,"parent_record_offset":39296,"ancestor_record_offset":34408,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":609,"record_offset":36492,"parent_record_offset":36464,"ancestor_record_offset":34408,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":36564,"parent_record_offset":36424,"ancestor_record_offset":34408,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":36768,"parent_record_offset":36564,"ancestor_record_offset":34408,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":609,"record_offset":36520,"parent_record_offset":36492,"ancestor_record_offset":34408,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":36424,"parent_record_offset":36388,"ancestor_record_offset":34408,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":36388,"parent_record_offset":36352,"ancestor_record_offset":34408,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":28516,"rva":13138448,"va":5381847568}
// 编译实例 RVAs: 0xc87a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6169599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e801dca1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381848282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6170254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e872d9a1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381848953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13126076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e844b637ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381848263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10931267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e843cca600","size":5,"soft_fail":false,"target_va":5392780768,"va":5381849496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a74110
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14598588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8bcc1de00","size":5,"soft_fail":false,"target_va":5396447504,"va":5381848911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a748e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14600613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8a5c9de00","size":5,"soft_fail":false,"target_va":5396449504,"va":5381848886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a750b0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14602643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e893d1de00","size":5,"soft_fail":false,"target_va":5396451504,"va":5381848856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34535968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e820fa0e02","size":5,"soft_fail":false,"target_va":5416385024,"va":5381849051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34939728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e850231502","size":5,"soft_fail":false,"target_va":5416788096,"va":5381848363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34772542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e83e961202","size":5,"soft_fail":false,"target_va":5416620794,"va":5381848247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34772362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e88a951202","size":5,"soft_fail":false,"target_va":5416621408,"va":5381849041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34772466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8f2951202","size":5,"soft_fail":false,"target_va":5416621600,"va":5381849129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34773117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e87d981202","size":5,"soft_fail":false,"target_va":5416622208,"va":5381849086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8ad1d0000","size":5,"soft_fail":false,"target_va":5381855392,"va":5381847790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34772204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8ec941202","size":5,"soft_fail":false,"target_va":5416621376,"va":5381849167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2115632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e830482000","size":5,"soft_fail":false,"target_va":5383963792,"va":5381848155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2124139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e86b692000","size":5,"soft_fail":false,"target_va":5383972224,"va":5381848080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2115894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e836492000","size":5,"soft_fail":false,"target_va":5383964912,"va":5381849013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2115068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e8fc452000","size":5,"soft_fail":false,"target_va":5383962816,"va":5381847743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34770239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381847568,"raw_hex":"e83f8d1202","size":5,"soft_fail":false,"target_va":5416619360,"va":5381849116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":31820,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":31232,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":609,"record_offset":32780,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":609,"record_offset":31652,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":609,"record_offset":28884,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":38823,"source_line":174,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":609,"record_offset":33972,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":32812,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::ws_to_px
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":609,"record_offset":31736,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":38824,"source_line":203,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":30424,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":29024,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":29412,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::format
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":609,"record_offset":32692,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":34265,"source_line":120,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":609,"record_offset":32664,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":38825,"source_line":69,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":609,"record_offset":32748,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":38826,"source_line":140,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"zeno","version":"0.2.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"zeno","version":"0.2.2","source":"git+https://github.com/PounceLight/zeno.git?rev=b8e8646d88f4556333867db62c2ab638e7b5375f#b8e8646d88f4556333867db62c2ab638e7b5375f"}]
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-0a863456bddec36c/b8e8646/src/mask.rs
//     evidence={"module":609,"record_offset":32720,"parent_record_offset":28516,"ancestor_record_offset":28516,"inlinee":34266,"source_line":107,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":31544,"parent_record_offset":31520,"ancestor_record_offset":28516,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":31568,"parent_record_offset":31544,"ancestor_record_offset":28516,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":33000,"parent_record_offset":32976,"ancestor_record_offset":28516,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":33024,"parent_record_offset":33000,"ancestor_record_offset":28516,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":31592,"parent_record_offset":31568,"ancestor_record_offset":28516,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":33048,"parent_record_offset":33024,"ancestor_record_offset":28516,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":31520,"parent_record_offset":31336,"ancestor_record_offset":28516,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":32976,"parent_record_offset":32884,"ancestor_record_offset":28516,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":31908,"parent_record_offset":31884,"ancestor_record_offset":28516,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":31280,"parent_record_offset":31256,"ancestor_record_offset":28516,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":32912,"parent_record_offset":32884,"ancestor_record_offset":28516,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":32884,"parent_record_offset":32860,"ancestor_record_offset":28516,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":31956,"parent_record_offset":31932,"ancestor_record_offset":28516,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":31932,"parent_record_offset":31908,"ancestor_record_offset":28516,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":31336,"parent_record_offset":31304,"ancestor_record_offset":28516,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":31304,"parent_record_offset":31280,"ancestor_record_offset":28516,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":32860,"parent_record_offset":32836,"ancestor_record_offset":28516,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":31376,"parent_record_offset":31336,"ancestor_record_offset":28516,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":31884,"parent_record_offset":31848,"ancestor_record_offset":28516,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":31848,"parent_record_offset":31820,"ancestor_record_offset":28516,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":31256,"parent_record_offset":31232,"ancestor_record_offset":28516,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":609,"record_offset":31400,"parent_record_offset":31376,"ancestor_record_offset":28516,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":30752,"parent_record_offset":30724,"ancestor_record_offset":28516,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":30780,"parent_record_offset":30752,"ancestor_record_offset":28516,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":30700,"parent_record_offset":30672,"ancestor_record_offset":28516,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":30724,"parent_record_offset":30700,"ancestor_record_offset":28516,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":30840,"parent_record_offset":30724,"ancestor_record_offset":28516,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":30952,"parent_record_offset":30928,"ancestor_record_offset":28516,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":609,"record_offset":31480,"parent_record_offset":31424,"ancestor_record_offset":28516,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":609,"record_offset":30804,"parent_record_offset":30780,"ancestor_record_offset":28516,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":609,"record_offset":31424,"parent_record_offset":31400,"ancestor_record_offset":28516,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":609,"record_offset":31452,"parent_record_offset":31424,"ancestor_record_offset":28516,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":609,"record_offset":32944,"parent_record_offset":32912,"ancestor_record_offset":28516,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":32572,"parent_record_offset":32548,"ancestor_record_offset":28516,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":32628,"parent_record_offset":32604,"ancestor_record_offset":28516,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":31784,"parent_record_offset":31760,"ancestor_record_offset":28516,"inlinee":14581,"source_line":985,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":609,"record_offset":30572,"parent_record_offset":30532,"ancestor_record_offset":28516,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":29360,"parent_record_offset":29332,"ancestor_record_offset":28516,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":31188,"parent_record_offset":30532,"ancestor_record_offset":28516,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":28980,"parent_record_offset":28956,"ancestor_record_offset":28516,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":30904,"parent_record_offset":30672,"ancestor_record_offset":28516,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":609,"record_offset":28956,"parent_record_offset":28932,"ancestor_record_offset":28516,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":32312,"parent_record_offset":32288,"ancestor_record_offset":28516,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":29832,"parent_record_offset":29808,"ancestor_record_offset":28516,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":28932,"parent_record_offset":28908,"ancestor_record_offset":28516,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":29808,"parent_record_offset":29784,"ancestor_record_offset":28516,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":30016,"parent_record_offset":29988,"ancestor_record_offset":28516,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":31996,"parent_record_offset":31848,"ancestor_record_offset":28516,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":609,"record_offset":30116,"parent_record_offset":30088,"ancestor_record_offset":28516,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":30088,"parent_record_offset":30060,"ancestor_record_offset":28516,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":29760,"parent_record_offset":29736,"ancestor_record_offset":28516,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":29784,"parent_record_offset":29760,"ancestor_record_offset":28516,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":28908,"parent_record_offset":28884,"ancestor_record_offset":28516,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":30144,"parent_record_offset":30116,"ancestor_record_offset":28516,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":31676,"parent_record_offset":31652,"ancestor_record_offset":28516,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":29736,"parent_record_offset":29620,"ancestor_record_offset":28516,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":29152,"parent_record_offset":29128,"ancestor_record_offset":28516,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":29448,"parent_record_offset":29412,"ancestor_record_offset":28516,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":29588,"parent_record_offset":29556,"ancestor_record_offset":28516,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":29556,"parent_record_offset":29520,"ancestor_record_offset":28516,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":29496,"parent_record_offset":29472,"ancestor_record_offset":28516,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":29520,"parent_record_offset":29496,"ancestor_record_offset":28516,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":29472,"parent_record_offset":29448,"ancestor_record_offset":28516,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":30060,"parent_record_offset":29520,"ancestor_record_offset":28516,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":29128,"parent_record_offset":29096,"ancestor_record_offset":28516,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":29096,"parent_record_offset":29068,"ancestor_record_offset":28516,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":29252,"parent_record_offset":29220,"ancestor_record_offset":28516,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":29308,"parent_record_offset":29252,"ancestor_record_offset":28516,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":29220,"parent_record_offset":29192,"ancestor_record_offset":28516,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":29068,"parent_record_offset":29024,"ancestor_record_offset":28516,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":29332,"parent_record_offset":29308,"ancestor_record_offset":28516,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":29620,"parent_record_offset":29588,"ancestor_record_offset":28516,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":29904,"parent_record_offset":29620,"ancestor_record_offset":28516,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":29876,"parent_record_offset":29620,"ancestor_record_offset":28516,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":29680,"parent_record_offset":29620,"ancestor_record_offset":28516,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$22::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":32604,"parent_record_offset":32460,"ancestor_record_offset":28516,"inlinee":30716,"source_line":1293,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$44::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":32548,"parent_record_offset":32460,"ancestor_record_offset":28516,"inlinee":34262,"source_line":1478,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$6::div_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":31760,"parent_record_offset":31736,"ancestor_record_offset":28516,"inlinee":34260,"source_line":1158,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":30460,"parent_record_offset":30424,"ancestor_record_offset":28516,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":33168,"parent_record_offset":33136,"ancestor_record_offset":28516,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":33608,"parent_record_offset":33404,"ancestor_record_offset":28516,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":609,"record_offset":30600,"parent_record_offset":30572,"ancestor_record_offset":28516,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":30672,"parent_record_offset":30532,"ancestor_record_offset":28516,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":30876,"parent_record_offset":30672,"ancestor_record_offset":28516,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":609,"record_offset":30628,"parent_record_offset":30600,"ancestor_record_offset":28516,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":30532,"parent_record_offset":30496,"ancestor_record_offset":28516,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":30496,"parent_record_offset":30460,"ancestor_record_offset":28516,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::DirtyTilePublisher::unique
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x90C4
// 首条位置: {"module":581,"inlinee":37060}
// 编译实例 RVAs: 0xc57980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tabloid::impl$0::default
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":581,"record_offset":146532,"parent_record_offset":146508,"ancestor_record_offset":146508,"inlinee":20485,"source_line":50,"archive":"_pdb/C/modules/00581.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::DirtyTilePublisher::shared
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7702
// 首条位置: {"module":518,"inlinee":30466}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":518,"record_offset":108936,"parent_record_offset":108880,"ancestor_record_offset":108880,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] tabloid::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":518,"record_offset":108908,"parent_record_offset":108880,"ancestor_record_offset":108880,"inlinee":20485,"source_line":50,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":518,"record_offset":109064,"parent_record_offset":109040,"ancestor_record_offset":108880,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":518,"record_offset":109088,"parent_record_offset":109064,"ancestor_record_offset":108880,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":518,"record_offset":109112,"parent_record_offset":109088,"ancestor_record_offset":108880,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":518,"record_offset":109040,"parent_record_offset":109008,"ancestor_record_offset":108880,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":518,"record_offset":109008,"parent_record_offset":108964,"ancestor_record_offset":108880,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":518,"record_offset":108964,"parent_record_offset":108936,"ancestor_record_offset":108880,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::DirtyTilePublisher::secondary
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7703
// 首条位置: {"module":518,"inlinee":30467}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":518,"record_offset":109288,"parent_record_offset":109264,"ancestor_record_offset":109264,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":518,"record_offset":109316,"parent_record_offset":109288,"ancestor_record_offset":109264,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":518,"record_offset":109340,"parent_record_offset":109316,"ancestor_record_offset":109264,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::new<512>
// 记录形式: compiled_procedure; instances=1
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":18620,"rva":13135104,"va":5381844224}
// 编译实例 RVAs: 0xc86d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6165701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381844224,"raw_hex":"e83beba1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381844384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-6165554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381844224,"raw_hex":"e8ceeba1ff","size":5,"soft_fail":false,"target_va":5375678736,"va":5381844285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-6166160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381844224,"raw_hex":"e870e9a1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381844859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=5
//     target_aliases=1699 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13122078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381844224,"raw_hex":"e8e2c537ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381844265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409c3d70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2896469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381844224,"raw_hex":"e8abcdd3ff","size":5,"soft_fail":false,"target_va":5378948464,"va":5381844928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35212503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381844224,"raw_hex":"e8d74c1902","size":5,"soft_fail":false,"target_va":5417057088,"va":5381844580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34776086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381844224,"raw_hex":"e816a41202","size":5,"soft_fail":false,"target_va":5416620794,"va":5381844703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":18768,"parent_record_offset":18620,"ancestor_record_offset":18620,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":20048,"parent_record_offset":18620,"ancestor_record_offset":18620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":18932,"parent_record_offset":18908,"ancestor_record_offset":18620,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":18956,"parent_record_offset":18932,"ancestor_record_offset":18620,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":20168,"parent_record_offset":20144,"ancestor_record_offset":18620,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":20192,"parent_record_offset":20168,"ancestor_record_offset":18620,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":18620,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":18980,"parent_record_offset":18956,"ancestor_record_offset":18620,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":20216,"parent_record_offset":20192,"ancestor_record_offset":18620,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":19496,"parent_record_offset":19468,"ancestor_record_offset":18620,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":18908,"parent_record_offset":18884,"ancestor_record_offset":18620,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":609,"record_offset":20144,"parent_record_offset":20120,"ancestor_record_offset":18620,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":19408,"parent_record_offset":19380,"ancestor_record_offset":18620,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":18824,"parent_record_offset":18796,"ancestor_record_offset":18620,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":20120,"parent_record_offset":20096,"ancestor_record_offset":18620,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":18884,"parent_record_offset":18852,"ancestor_record_offset":18620,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":19436,"parent_record_offset":19408,"ancestor_record_offset":18620,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":18852,"parent_record_offset":18824,"ancestor_record_offset":18620,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":609,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":18620,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":19624,"parent_record_offset":19344,"ancestor_record_offset":18620,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":609,"record_offset":19380,"parent_record_offset":19344,"ancestor_record_offset":18620,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":609,"record_offset":19344,"parent_record_offset":19316,"ancestor_record_offset":18620,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":609,"record_offset":18796,"parent_record_offset":18768,"ancestor_record_offset":18620,"inlinee":9075,"source_line":49,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":609,"record_offset":19724,"parent_record_offset":19696,"ancestor_record_offset":18620,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":609,"record_offset":19696,"parent_record_offset":19624,"ancestor_record_offset":18620,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":20072,"parent_record_offset":20048,"ancestor_record_offset":18620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":19660,"parent_record_offset":19624,"ancestor_record_offset":18620,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::as_float_grid
// 记录形式: inline_instance, inlinee_declaration; instances=69
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BE2
// 首条位置: {"module":470,"inlinee":27618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::ws_to_px
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97A8
// 首条位置: {"module":609,"inlinee":38824}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$22::mul_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":32604,"parent_record_offset":32460,"ancestor_record_offset":32460,"inlinee":30716,"source_line":1293,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$44::add_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":32548,"parent_record_offset":32460,"ancestor_record_offset":32460,"inlinee":34262,"source_line":1478,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$6::div_assign
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":609,"record_offset":31760,"parent_record_offset":31736,"ancestor_record_offset":31736,"inlinee":34260,"source_line":1158,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":32572,"parent_record_offset":32548,"ancestor_record_offset":32460,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":32628,"parent_record_offset":32604,"ancestor_record_offset":32460,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":609,"record_offset":31784,"parent_record_offset":31760,"ancestor_record_offset":31736,"inlinee":14581,"source_line":985,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::subscribe<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":20724,"rva":13136000,"va":5381845120}
// 编译实例 RVAs: 0xc87080, 0x2b20e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=2
//     target_aliases=51 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12539769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845120,"raw_hex":"e887a840ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5381845348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409c7e80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2880095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845120,"raw_hex":"e8a10dd4ff","size":5,"soft_fail":false,"target_va":5378965120,"va":5381845210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34776948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845120,"raw_hex":"e874a71202","size":5,"soft_fail":false,"target_va":5416622240,"va":5381845287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t34731526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845120,"raw_hex":"e806f61102","size":5,"soft_fail":false,"target_va":5416576832,"va":5381845301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t34730826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845120,"raw_hex":"e84af31102","size":5,"soft_fail":false,"target_va":5416576144,"va":5381845313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2139645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845120,"raw_hex":"e8fda52000","size":5,"soft_fail":false,"target_va":5383984848,"va":5381845198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2140129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381845120,"raw_hex":"e8e1a72000","size":5,"soft_fail":false,"target_va":5383985360,"va":5381845226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1409c7e80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-2880148","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381845120,"raw_hex":"e96c0dd4ff","size":5,"soft_fail":false,"target_va":5378965120,"va":5381845263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":609,"record_offset":20888,"parent_record_offset":20724,"ancestor_record_offset":20724,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":21128,"parent_record_offset":20724,"ancestor_record_offset":20724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":609,"record_offset":20916,"parent_record_offset":20724,"ancestor_record_offset":20724,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":609,"record_offset":21072,"parent_record_offset":20724,"ancestor_record_offset":20724,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":21096,"parent_record_offset":21072,"ancestor_record_offset":20724,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":609,"record_offset":21240,"parent_record_offset":21180,"ancestor_record_offset":20724,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":609,"record_offset":20980,"parent_record_offset":20944,"ancestor_record_offset":20724,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":609,"record_offset":21264,"parent_record_offset":21240,"ancestor_record_offset":20724,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":609,"record_offset":21004,"parent_record_offset":20980,"ancestor_record_offset":20724,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":609,"record_offset":21152,"parent_record_offset":21128,"ancestor_record_offset":20724,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":609,"record_offset":20944,"parent_record_offset":20916,"ancestor_record_offset":20724,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":609,"record_offset":21180,"parent_record_offset":21152,"ancestor_record_offset":20724,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":609,"record_offset":21036,"parent_record_offset":20944,"ancestor_record_offset":20724,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":609,"record_offset":21212,"parent_record_offset":21180,"ancestor_record_offset":20724,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::last_staged
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 222
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F5F
// 首条位置: {"module":502,"inlinee":28511}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::replace_composite
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F63
// 首条位置: {"module":502,"inlinee":28515}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":502,"record_offset":9812,"parent_record_offset":9772,"ancestor_record_offset":9772,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":9860,"parent_record_offset":9836,"ancestor_record_offset":9772,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":502,"record_offset":9836,"parent_record_offset":9812,"ancestor_record_offset":9772,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::clear_composite
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F62
// 首条位置: {"module":502,"inlinee":28514}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":502,"record_offset":9656,"parent_record_offset":9628,"ancestor_record_offset":9628,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":502,"record_offset":9680,"parent_record_offset":9656,"ancestor_record_offset":9628,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::prepare_tile
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97B9
// 首条位置: {"module":1745,"inlinee":38841}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":27276,"parent_record_offset":25820,"ancestor_record_offset":25820,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":26468,"parent_record_offset":25820,"ancestor_record_offset":25820,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":26036,"parent_record_offset":25820,"ancestor_record_offset":25820,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":27332,"parent_record_offset":25820,"ancestor_record_offset":25820,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":27300,"parent_record_offset":27276,"ancestor_record_offset":25820,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":26368,"parent_record_offset":26272,"ancestor_record_offset":25820,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1745,"record_offset":26608,"parent_record_offset":26552,"ancestor_record_offset":25820,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":26552,"parent_record_offset":26468,"ancestor_record_offset":25820,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":26272,"parent_record_offset":26176,"ancestor_record_offset":25820,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":27356,"parent_record_offset":27332,"ancestor_record_offset":25820,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":26100,"parent_record_offset":26036,"ancestor_record_offset":25820,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97C9
// 首条位置: {"module":1803,"inlinee":38857}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":490072,"parent_record_offset":490036,"ancestor_record_offset":490036,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":490096,"parent_record_offset":490072,"ancestor_record_offset":490036,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":490120,"parent_record_offset":490096,"ancestor_record_offset":490036,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":490144,"parent_record_offset":490120,"ancestor_record_offset":490036,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15737
// 首条位置: {"module":1803,"inlinee":87863}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::value::impl$30::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":489940,"parent_record_offset":489912,"ancestor_record_offset":489912,"inlinee":87849,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":490504,"parent_record_offset":490480,"ancestor_record_offset":489912,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":490528,"parent_record_offset":490504,"ancestor_record_offset":489912,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":490552,"parent_record_offset":490528,"ancestor_record_offset":489912,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":490480,"parent_record_offset":490412,"ancestor_record_offset":489912,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":490448,"parent_record_offset":490412,"ancestor_record_offset":489912,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":490412,"parent_record_offset":490384,"ancestor_record_offset":489912,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":490384,"parent_record_offset":490356,"ancestor_record_offset":489912,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":490096,"parent_record_offset":490072,"ancestor_record_offset":489912,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":490300,"parent_record_offset":489996,"ancestor_record_offset":489912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":490120,"parent_record_offset":490096,"ancestor_record_offset":489912,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":490144,"parent_record_offset":490120,"ancestor_record_offset":489912,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":490072,"parent_record_offset":490036,"ancestor_record_offset":489912,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1803,"record_offset":490036,"parent_record_offset":489996,"ancestor_record_offset":489912,"inlinee":38857,"source_line":291,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":489996,"parent_record_offset":489968,"ancestor_record_offset":489912,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::value::impl$30::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/value.rs
//     evidence={"module":1803,"record_offset":489968,"parent_record_offset":489940,"ancestor_record_offset":489912,"inlinee":87850,"source_line":683,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97A3
// 首条位置: {"module":609,"inlinee":38819}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":18548,"parent_record_offset":18492,"ancestor_record_offset":18492,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":18580,"parent_record_offset":18492,"ancestor_record_offset":18492,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":609,"record_offset":18520,"parent_record_offset":18492,"ancestor_record_offset":18492,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97CD
// 首条位置: {"module":1803,"inlinee":38861}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":323932,"parent_record_offset":323908,"ancestor_record_offset":323908,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":323956,"parent_record_offset":323932,"ancestor_record_offset":323908,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":323980,"parent_record_offset":323956,"ancestor_record_offset":323908,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":324008,"parent_record_offset":323980,"ancestor_record_offset":323908,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156E4
// 首条位置: {"module":1803,"inlinee":87780}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":323808,"parent_record_offset":323784,"ancestor_record_offset":323784,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":324228,"parent_record_offset":324204,"ancestor_record_offset":323784,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":324252,"parent_record_offset":324228,"ancestor_record_offset":323784,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":324276,"parent_record_offset":324252,"ancestor_record_offset":323784,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":324204,"parent_record_offset":324148,"ancestor_record_offset":323784,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":324176,"parent_record_offset":324148,"ancestor_record_offset":323784,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":324148,"parent_record_offset":324124,"ancestor_record_offset":323784,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":324124,"parent_record_offset":324100,"ancestor_record_offset":323784,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":323956,"parent_record_offset":323932,"ancestor_record_offset":323784,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":324052,"parent_record_offset":323880,"ancestor_record_offset":323784,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":323980,"parent_record_offset":323956,"ancestor_record_offset":323784,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":324008,"parent_record_offset":323980,"ancestor_record_offset":323784,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":323932,"parent_record_offset":323908,"ancestor_record_offset":323784,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1803,"record_offset":323908,"parent_record_offset":323880,"ancestor_record_offset":323784,"inlinee":38861,"source_line":291,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":323880,"parent_record_offset":323856,"ancestor_record_offset":323784,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":323832,"parent_record_offset":323808,"ancestor_record_offset":323784,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":323856,"parent_record_offset":323832,"ancestor_record_offset":323784,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15782
// 首条位置: {"module":1804,"inlinee":87938}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":227652,"parent_record_offset":227548,"ancestor_record_offset":227548,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":228164,"parent_record_offset":227548,"ancestor_record_offset":227548,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":228304,"parent_record_offset":227548,"ancestor_record_offset":227548,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":227596,"parent_record_offset":227548,"ancestor_record_offset":227548,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":227688,"parent_record_offset":227548,"ancestor_record_offset":227548,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":227804,"parent_record_offset":227780,"ancestor_record_offset":227548,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":227780,"parent_record_offset":227744,"ancestor_record_offset":227548,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":228236,"parent_record_offset":228212,"ancestor_record_offset":227548,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":228212,"parent_record_offset":228188,"ancestor_record_offset":227548,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":228260,"parent_record_offset":228236,"ancestor_record_offset":227548,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":227620,"parent_record_offset":227596,"ancestor_record_offset":227548,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":227716,"parent_record_offset":227688,"ancestor_record_offset":227548,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":227864,"parent_record_offset":227840,"ancestor_record_offset":227548,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":227840,"parent_record_offset":227744,"ancestor_record_offset":227548,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":228084,"parent_record_offset":228060,"ancestor_record_offset":227548,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":227744,"parent_record_offset":227716,"ancestor_record_offset":227548,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":18248,"rva":13134624,"va":5381843744}
// 编译实例 RVAs: 0xc86b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde_core..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t34716433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843744,"raw_hex":"e811bb1102","size":5,"soft_fail":false,"target_va":5416560512,"va":5381844074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde_core..de..VariantAccess$GT$::unit_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1406573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843744,"raw_hex":"e86d761500","size":5,"soft_fail":false,"target_va":5383250560,"va":5381843982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409de280
//     owner=unresolved_namespace; count=3
//     target_aliases=16 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2787847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843744,"raw_hex":"e8f975d5ff","size":5,"soft_fail":false,"target_va":5379056256,"va":5381844098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bacfd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15885329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843744,"raw_hex":"e81164f200","size":5,"soft_fail":false,"target_va":5397729232,"va":5381843898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141baf610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15895053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843744,"raw_hex":"e80d8af200","size":5,"soft_fail":false,"target_va":5397739024,"va":5381843966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141beee50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16155323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843744,"raw_hex":"e8bb82f600","size":5,"soft_fail":false,"target_va":5397999184,"va":5381843856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t34717129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843744,"raw_hex":"e8c9bd1102","size":5,"soft_fail":false,"target_va":5416561184,"va":5381844050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$4::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":609,"record_offset":18428,"parent_record_offset":18248,"ancestor_record_offset":18248,"inlinee":33271,"source_line":328,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":18548,"parent_record_offset":18492,"ancestor_record_offset":18248,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":609,"record_offset":18580,"parent_record_offset":18492,"ancestor_record_offset":18248,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":609,"record_offset":18492,"parent_record_offset":18428,"ancestor_record_offset":18248,"inlinee":38819,"source_line":291,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":609,"record_offset":18520,"parent_record_offset":18492,"ancestor_record_offset":18248,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":38,"record_offset":248,"rva":1652896,"va":5370362016}
// 编译实例 RVAs: 0x1938a0, 0xc868e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140464a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2953481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370362016,"raw_hex":"e809112d00","size":5,"soft_fail":false,"target_va":5373315680,"va":5370362194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14046b660
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8499920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843168,"raw_hex":"e8304d7eff","size":5,"soft_fail":false,"target_va":5373343328,"va":5381843243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12793321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843168,"raw_hex":"e8e935c300","size":5,"soft_fail":false,"target_va":5394636672,"va":5381843346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34509520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843168,"raw_hex":"e8d0920e02","size":5,"soft_fail":false,"target_va":5416353040,"va":5381843515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34778549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843168,"raw_hex":"e8b5ad1202","size":5,"soft_fail":false,"target_va":5416622208,"va":5381843654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tcallq\t34716668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381843168,"raw_hex":"e8fcbb1102","size":5,"soft_fail":false,"target_va":5416560208,"va":5381843535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     evidence={"assembly":"\tjmp\t34716649","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381843168,"raw_hex":"e9e9bb1102","size":5,"soft_fail":false,"target_va":5416560208,"va":5381843554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$10::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":2244,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7412,"source_line":752,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$10::serialize_field
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":2188,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7411,"source_line":738,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct_variant
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":532,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7409,"source_line":383,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":3256,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7408,"source_line":214,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":932,"parent_record_offset":900,"ancestor_record_offset":248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":800,"parent_record_offset":772,"ancestor_record_offset":248,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":860,"parent_record_offset":828,"ancestor_record_offset":248,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":38,"record_offset":828,"parent_record_offset":800,"ancestor_record_offset":248,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":732,"parent_record_offset":704,"ancestor_record_offset":248,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":900,"parent_record_offset":732,"ancestor_record_offset":248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":676,"parent_record_offset":648,"ancestor_record_offset":248,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":1020,"parent_record_offset":732,"ancestor_record_offset":248,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":38,"record_offset":772,"parent_record_offset":732,"ancestor_record_offset":248,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":38,"record_offset":704,"parent_record_offset":676,"ancestor_record_offset":248,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":38,"record_offset":1132,"parent_record_offset":1100,"ancestor_record_offset":248,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":38,"record_offset":1100,"parent_record_offset":1072,"ancestor_record_offset":248,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":592,"parent_record_offset":532,"ancestor_record_offset":248,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object_value
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":1168,"parent_record_offset":532,"ancestor_record_offset":248,"inlinee":7401,"source_line":1910,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":2292,"parent_record_offset":2244,"ancestor_record_offset":248,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":1672,"parent_record_offset":532,"ancestor_record_offset":248,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":1072,"parent_record_offset":532,"ancestor_record_offset":248,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":38,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":248,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":38,"record_offset":648,"parent_record_offset":620,"ancestor_record_offset":248,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":38,"record_offset":620,"parent_record_offset":592,"ancestor_record_offset":248,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00038.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$16::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97B0
// 首条位置: {"module":609,"inlinee":38832}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::clear_all<512>
// 记录形式: compiled_procedure; instances=1
// 源码行: 345
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":47244,"rva":13142592,"va":5381851712}
// 编译实例 RVAs: 0xc88a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13129999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e8f1a637ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381852186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34532632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e818ed0e02","size":5,"soft_fail":false,"target_va":5416385024,"va":5381852387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35204944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e8502f1902","size":5,"soft_fail":false,"target_va":5417057088,"va":5381852139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34769026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e882881202","size":5,"soft_fail":false,"target_va":5416621408,"va":5381852377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e81a0e0000","size":5,"soft_fail":false,"target_va":5381855392,"va":5381851777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34768911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e80f881202","size":5,"soft_fail":false,"target_va":5416621376,"va":5381852460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2111674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e8ba382000","size":5,"soft_fail":false,"target_va":5383963792,"va":5381852113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2120178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e8f2592000","size":5,"soft_fail":false,"target_va":5383972224,"va":5381852041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2112565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e8353c2000","size":5,"soft_fail":false,"target_va":5383964912,"va":5381852342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2111069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e85d362000","size":5,"soft_fail":false,"target_va":5383962816,"va":5381851742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34766930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381851712,"raw_hex":"e852801202","size":5,"soft_fail":false,"target_va":5416619360,"va":5381852425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":49904,"parent_record_offset":47244,"ancestor_record_offset":47244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":49608,"parent_record_offset":47244,"ancestor_record_offset":47244,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":48800,"parent_record_offset":47244,"ancestor_record_offset":47244,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":47408,"parent_record_offset":47244,"ancestor_record_offset":47244,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":47788,"parent_record_offset":47244,"ancestor_record_offset":47244,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":49128,"parent_record_offset":49100,"ancestor_record_offset":47244,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":49156,"parent_record_offset":49128,"ancestor_record_offset":47244,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":49076,"parent_record_offset":49048,"ancestor_record_offset":47244,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":49100,"parent_record_offset":49076,"ancestor_record_offset":47244,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":49216,"parent_record_offset":49100,"ancestor_record_offset":47244,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":49328,"parent_record_offset":49304,"ancestor_record_offset":47244,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":609,"record_offset":49180,"parent_record_offset":49156,"ancestor_record_offset":47244,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":609,"record_offset":48948,"parent_record_offset":48908,"ancestor_record_offset":47244,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":47736,"parent_record_offset":47712,"ancestor_record_offset":47244,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":49564,"parent_record_offset":48908,"ancestor_record_offset":47244,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":49280,"parent_record_offset":49048,"ancestor_record_offset":47244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":49864,"parent_record_offset":49788,"ancestor_record_offset":47244,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":48208,"parent_record_offset":48184,"ancestor_record_offset":47244,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":48184,"parent_record_offset":48160,"ancestor_record_offset":47244,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":48392,"parent_record_offset":48364,"ancestor_record_offset":47244,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":609,"record_offset":48492,"parent_record_offset":48464,"ancestor_record_offset":47244,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":48464,"parent_record_offset":48436,"ancestor_record_offset":47244,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":48136,"parent_record_offset":48112,"ancestor_record_offset":47244,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":48160,"parent_record_offset":48136,"ancestor_record_offset":47244,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":48520,"parent_record_offset":48492,"ancestor_record_offset":47244,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":48112,"parent_record_offset":47996,"ancestor_record_offset":47244,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":49788,"parent_record_offset":49748,"ancestor_record_offset":47244,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$2::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":609,"record_offset":49632,"parent_record_offset":49608,"ancestor_record_offset":47244,"inlinee":19477,"source_line":31,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$6::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":609,"record_offset":49748,"parent_record_offset":49720,"ancestor_record_offset":47244,"inlinee":38828,"source_line":52,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":47536,"parent_record_offset":47512,"ancestor_record_offset":47244,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":47824,"parent_record_offset":47788,"ancestor_record_offset":47244,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":47964,"parent_record_offset":47932,"ancestor_record_offset":47244,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":47932,"parent_record_offset":47896,"ancestor_record_offset":47244,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":47872,"parent_record_offset":47848,"ancestor_record_offset":47244,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":47896,"parent_record_offset":47872,"ancestor_record_offset":47244,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":47848,"parent_record_offset":47824,"ancestor_record_offset":47244,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":48436,"parent_record_offset":47896,"ancestor_record_offset":47244,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":47512,"parent_record_offset":47480,"ancestor_record_offset":47244,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":47480,"parent_record_offset":47452,"ancestor_record_offset":47244,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":47636,"parent_record_offset":47604,"ancestor_record_offset":47244,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":47688,"parent_record_offset":47636,"ancestor_record_offset":47244,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":47604,"parent_record_offset":47576,"ancestor_record_offset":47244,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":47452,"parent_record_offset":47408,"ancestor_record_offset":47244,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":47712,"parent_record_offset":47688,"ancestor_record_offset":47244,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":47996,"parent_record_offset":47964,"ancestor_record_offset":47244,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":48280,"parent_record_offset":47996,"ancestor_record_offset":47244,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":48252,"parent_record_offset":47996,"ancestor_record_offset":47244,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":48056,"parent_record_offset":47996,"ancestor_record_offset":47244,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":48836,"parent_record_offset":48800,"ancestor_record_offset":47244,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":49968,"parent_record_offset":49936,"ancestor_record_offset":47244,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":50408,"parent_record_offset":50204,"ancestor_record_offset":47244,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":609,"record_offset":48976,"parent_record_offset":48948,"ancestor_record_offset":47244,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":49048,"parent_record_offset":48908,"ancestor_record_offset":47244,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":49252,"parent_record_offset":49048,"ancestor_record_offset":47244,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":609,"record_offset":49004,"parent_record_offset":48976,"ancestor_record_offset":47244,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":48908,"parent_record_offset":48872,"ancestor_record_offset":47244,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":48872,"parent_record_offset":48836,"ancestor_record_offset":47244,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::copy_composite_to_scratch<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 367
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":40300,"rva":13140768,"va":5381849888}
// 编译实例 RVAs: 0xc88320, 0x285aba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10930035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e873c7a600","size":5,"soft_fail":false,"target_va":5392780768,"va":5381850728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34534542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e88ef40e02","size":5,"soft_fail":false,"target_va":5416385024,"va":5381850477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35206689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e821361902","size":5,"soft_fail":false,"target_va":5417057056,"va":5381850362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34770936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e8f88f1202","size":5,"soft_fail":false,"target_va":5416621408,"va":5381850467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::copy_from_slice_impl::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34772311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e857951202","size":5,"soft_fail":false,"target_va":5416622768,"va":5381850452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t5433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e839150000","size":5,"soft_fail":false,"target_va":5381855392,"va":5381849954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t34770821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e8858f1202","size":5,"soft_fail":false,"target_va":5416621376,"va":5381850550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2113475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e8c33f2000","size":5,"soft_fail":false,"target_va":5383963792,"va":5381850312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2121979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e8fb602000","size":5,"soft_fail":false,"target_va":5383972224,"va":5381850240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2114500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e8c4432000","size":5,"soft_fail":false,"target_va":5383964912,"va":5381850407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2112893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e87d3d2000","size":5,"soft_fail":false,"target_va":5383962816,"va":5381849918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6000222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411023776,"raw_hex":"e85e8e5b00","size":5,"soft_fail":false,"target_va":5417024592,"va":5411024365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411023776,"raw_hex":"e8f3070000","size":5,"soft_fail":false,"target_va":5411026656,"va":5411024616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34768840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381849888,"raw_hex":"e8c8871202","size":5,"soft_fail":false,"target_va":5416619360,"va":5381850515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":42788,"parent_record_offset":40300,"ancestor_record_offset":40300,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":42684,"parent_record_offset":40300,"ancestor_record_offset":40300,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":41876,"parent_record_offset":40300,"ancestor_record_offset":40300,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":40480,"parent_record_offset":40300,"ancestor_record_offset":40300,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":40864,"parent_record_offset":40300,"ancestor_record_offset":40300,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":42204,"parent_record_offset":42176,"ancestor_record_offset":40300,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":42232,"parent_record_offset":42204,"ancestor_record_offset":40300,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":42152,"parent_record_offset":42124,"ancestor_record_offset":40300,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":42176,"parent_record_offset":42152,"ancestor_record_offset":40300,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":42292,"parent_record_offset":42176,"ancestor_record_offset":40300,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":42404,"parent_record_offset":42380,"ancestor_record_offset":40300,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":609,"record_offset":42256,"parent_record_offset":42232,"ancestor_record_offset":40300,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":609,"record_offset":42024,"parent_record_offset":41984,"ancestor_record_offset":40300,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":40812,"parent_record_offset":40784,"ancestor_record_offset":40300,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":42640,"parent_record_offset":41984,"ancestor_record_offset":40300,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":42752,"parent_record_offset":42716,"ancestor_record_offset":40300,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":42356,"parent_record_offset":42124,"ancestor_record_offset":40300,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":41284,"parent_record_offset":41260,"ancestor_record_offset":40300,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":41260,"parent_record_offset":41236,"ancestor_record_offset":40300,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":41468,"parent_record_offset":41440,"ancestor_record_offset":40300,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":609,"record_offset":41568,"parent_record_offset":41540,"ancestor_record_offset":40300,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":42716,"parent_record_offset":42684,"ancestor_record_offset":40300,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":41540,"parent_record_offset":41512,"ancestor_record_offset":40300,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":41212,"parent_record_offset":41188,"ancestor_record_offset":40300,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":41236,"parent_record_offset":41212,"ancestor_record_offset":40300,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":41596,"parent_record_offset":41568,"ancestor_record_offset":40300,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":41188,"parent_record_offset":41072,"ancestor_record_offset":40300,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":40608,"parent_record_offset":40584,"ancestor_record_offset":40300,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":40900,"parent_record_offset":40864,"ancestor_record_offset":40300,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":41040,"parent_record_offset":41008,"ancestor_record_offset":40300,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":41008,"parent_record_offset":40972,"ancestor_record_offset":40300,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":40948,"parent_record_offset":40924,"ancestor_record_offset":40300,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":40972,"parent_record_offset":40948,"ancestor_record_offset":40300,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":40924,"parent_record_offset":40900,"ancestor_record_offset":40300,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":41512,"parent_record_offset":40972,"ancestor_record_offset":40300,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":40584,"parent_record_offset":40552,"ancestor_record_offset":40300,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":40552,"parent_record_offset":40524,"ancestor_record_offset":40300,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":40708,"parent_record_offset":40676,"ancestor_record_offset":40300,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":40760,"parent_record_offset":40708,"ancestor_record_offset":40300,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":40676,"parent_record_offset":40648,"ancestor_record_offset":40300,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":40524,"parent_record_offset":40480,"ancestor_record_offset":40300,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":40784,"parent_record_offset":40760,"ancestor_record_offset":40300,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":41072,"parent_record_offset":41040,"ancestor_record_offset":40300,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":41356,"parent_record_offset":41072,"ancestor_record_offset":40300,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":41328,"parent_record_offset":41072,"ancestor_record_offset":40300,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":41132,"parent_record_offset":41072,"ancestor_record_offset":40300,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":41912,"parent_record_offset":41876,"ancestor_record_offset":40300,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":42852,"parent_record_offset":42820,"ancestor_record_offset":40300,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":43292,"parent_record_offset":43088,"ancestor_record_offset":40300,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":609,"record_offset":42052,"parent_record_offset":42024,"ancestor_record_offset":40300,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":42124,"parent_record_offset":41984,"ancestor_record_offset":40300,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":42328,"parent_record_offset":42124,"ancestor_record_offset":40300,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":609,"record_offset":42080,"parent_record_offset":42052,"ancestor_record_offset":40300,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":41984,"parent_record_offset":41948,"ancestor_record_offset":40300,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":41948,"parent_record_offset":41912,"ancestor_record_offset":40300,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::commit_scratch_to_composite<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 376
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":43772,"rva":13141680,"va":5381850800}
// 编译实例 RVAs: 0xc886b0, 0x285af30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10929123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e8e3c3a600","size":5,"soft_fail":false,"target_va":5392780768,"va":5381851640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34533630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e8fef00e02","size":5,"soft_fail":false,"target_va":5416385024,"va":5381851389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35205777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e891321902","size":5,"soft_fail":false,"target_va":5417057056,"va":5381851274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34770024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e8688c1202","size":5,"soft_fail":false,"target_va":5416621408,"va":5381851379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::copy_from_slice_impl::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34771399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e8c7911202","size":5,"soft_fail":false,"target_va":5416622768,"va":5381851364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e8a9110000","size":5,"soft_fail":false,"target_va":5381855392,"va":5381850866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t34769909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e8f58b1202","size":5,"soft_fail":false,"target_va":5416621376,"va":5381851462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2112563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e8333c2000","size":5,"soft_fail":false,"target_va":5383963792,"va":5381851224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2121067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e86b5d2000","size":5,"soft_fail":false,"target_va":5383972224,"va":5381851152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2113588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e834402000","size":5,"soft_fail":false,"target_va":5383964912,"va":5381851319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2111981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e8ed392000","size":5,"soft_fail":false,"target_va":5383962816,"va":5381850830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5999310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411024688,"raw_hex":"e8ce8a5b00","size":5,"soft_fail":false,"target_va":5417024592,"va":5411025277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411024688,"raw_hex":"e863040000","size":5,"soft_fail":false,"target_va":5411026656,"va":5411025528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34767928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381850800,"raw_hex":"e838841202","size":5,"soft_fail":false,"target_va":5416619360,"va":5381851427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":46260,"parent_record_offset":43772,"ancestor_record_offset":43772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":46156,"parent_record_offset":43772,"ancestor_record_offset":43772,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":45348,"parent_record_offset":43772,"ancestor_record_offset":43772,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":43952,"parent_record_offset":43772,"ancestor_record_offset":43772,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":44336,"parent_record_offset":43772,"ancestor_record_offset":43772,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":45676,"parent_record_offset":45648,"ancestor_record_offset":43772,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":45704,"parent_record_offset":45676,"ancestor_record_offset":43772,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":45624,"parent_record_offset":45596,"ancestor_record_offset":43772,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":45648,"parent_record_offset":45624,"ancestor_record_offset":43772,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":45764,"parent_record_offset":45648,"ancestor_record_offset":43772,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":45876,"parent_record_offset":45852,"ancestor_record_offset":43772,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":609,"record_offset":45728,"parent_record_offset":45704,"ancestor_record_offset":43772,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":609,"record_offset":45496,"parent_record_offset":45456,"ancestor_record_offset":43772,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":44284,"parent_record_offset":44256,"ancestor_record_offset":43772,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":46112,"parent_record_offset":45456,"ancestor_record_offset":43772,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":46224,"parent_record_offset":46188,"ancestor_record_offset":43772,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":45828,"parent_record_offset":45596,"ancestor_record_offset":43772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":44756,"parent_record_offset":44732,"ancestor_record_offset":43772,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":44732,"parent_record_offset":44708,"ancestor_record_offset":43772,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":44940,"parent_record_offset":44912,"ancestor_record_offset":43772,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":609,"record_offset":45040,"parent_record_offset":45012,"ancestor_record_offset":43772,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":46188,"parent_record_offset":46156,"ancestor_record_offset":43772,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":45012,"parent_record_offset":44984,"ancestor_record_offset":43772,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":44684,"parent_record_offset":44660,"ancestor_record_offset":43772,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":44708,"parent_record_offset":44684,"ancestor_record_offset":43772,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":45068,"parent_record_offset":45040,"ancestor_record_offset":43772,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":44660,"parent_record_offset":44544,"ancestor_record_offset":43772,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":44080,"parent_record_offset":44056,"ancestor_record_offset":43772,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":44372,"parent_record_offset":44336,"ancestor_record_offset":43772,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":44512,"parent_record_offset":44480,"ancestor_record_offset":43772,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":44480,"parent_record_offset":44444,"ancestor_record_offset":43772,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":44420,"parent_record_offset":44396,"ancestor_record_offset":43772,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":44444,"parent_record_offset":44420,"ancestor_record_offset":43772,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":44396,"parent_record_offset":44372,"ancestor_record_offset":43772,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":44984,"parent_record_offset":44444,"ancestor_record_offset":43772,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":44056,"parent_record_offset":44024,"ancestor_record_offset":43772,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":44024,"parent_record_offset":43996,"ancestor_record_offset":43772,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":44180,"parent_record_offset":44148,"ancestor_record_offset":43772,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":44232,"parent_record_offset":44180,"ancestor_record_offset":43772,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":44148,"parent_record_offset":44120,"ancestor_record_offset":43772,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":43996,"parent_record_offset":43952,"ancestor_record_offset":43772,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":44256,"parent_record_offset":44232,"ancestor_record_offset":43772,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":44544,"parent_record_offset":44512,"ancestor_record_offset":43772,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":44828,"parent_record_offset":44544,"ancestor_record_offset":43772,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":44800,"parent_record_offset":44544,"ancestor_record_offset":43772,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":44604,"parent_record_offset":44544,"ancestor_record_offset":43772,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":45384,"parent_record_offset":45348,"ancestor_record_offset":43772,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":46324,"parent_record_offset":46292,"ancestor_record_offset":43772,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":46764,"parent_record_offset":46560,"ancestor_record_offset":43772,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":609,"record_offset":45524,"parent_record_offset":45496,"ancestor_record_offset":43772,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":45596,"parent_record_offset":45456,"ancestor_record_offset":43772,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":45800,"parent_record_offset":45596,"ancestor_record_offset":43772,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":609,"record_offset":45552,"parent_record_offset":45524,"ancestor_record_offset":43772,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":45456,"parent_record_offset":45420,"ancestor_record_offset":43772,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":45420,"parent_record_offset":45384,"ancestor_record_offset":43772,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::rasterize_stroke::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=51
// 源码行: 437
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15D08
// 首条位置: {"module":1842,"inlinee":89352}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1842,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":15036,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01842.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::rasterize_stroke::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=51
// 源码行: 440
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15D05
// 首条位置: {"module":1842,"inlinee":89349}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1842,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":7024,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01842.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::rasterize_stroke::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=51
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15D01
// 首条位置: {"module":1842,"inlinee":89345}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$6::saturating_add
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1842,"record_offset":4324,"parent_record_offset":4296,"ancestor_record_offset":4296,"inlinee":89346,"source_line":2300,"archive":"_pdb/C/modules/01842.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::extend_dirty
// 记录形式: inline_instance, inlinee_declaration; instances=54
// 源码行: 610
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8D70
// 首条位置: {"module":571,"inlinee":36208}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=48
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":571,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":6052,"inlinee":33040,"source_line":189,"archive":"_pdb/C/modules/00571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::copy_to_staging<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 614
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":609,"record_offset":24600,"rva":13137088,"va":5381846208}
// 编译实例 RVAs: 0xc874c0, 0x2859d40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=2
//     target_aliases=51 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12541960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8f89f40ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5381847539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409c7460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2884326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e81afdd3ff","size":5,"soft_fail":false,"target_va":5378962528,"va":5381846849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10933355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e86bd4a600","size":5,"soft_fail":false,"target_va":5392780768,"va":5381847408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a71280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14588775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8679bde00","size":5,"soft_fail":false,"target_va":5396435584,"va":5381846804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428d47d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t501770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411020096,"raw_hex":"e80aa80700","size":5,"soft_fail":false,"target_va":5411522512,"va":5411020737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142afa550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2753975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411020096,"raw_hex":"e8b7052a00","size":5,"soft_fail":false,"target_va":5413774672,"va":5411020692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34537943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8d7010f02","size":5,"soft_fail":false,"target_va":5416385024,"va":5381847076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35210214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8e6431902","size":5,"soft_fail":false,"target_va":5417057056,"va":5381846837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35210808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e838461902","size":5,"soft_fail":false,"target_va":5417057088,"va":5381846275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34774337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8419d1202","size":5,"soft_fail":false,"target_va":5416621408,"va":5381847066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t9060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e864230000","size":5,"soft_fail":false,"target_va":5381855392,"va":5381846327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t34774185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8a99c1202","size":5,"soft_fail":false,"target_va":5416621376,"va":5381847186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t34729698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8e2ee1102","size":5,"soft_fail":false,"target_va":5416576832,"va":5381847129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t34728994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e822ec1102","size":5,"soft_fail":false,"target_va":5416576144,"va":5381847145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2137960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8689f2000","size":5,"soft_fail":false,"target_va":5383984848,"va":5381846883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2138411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e82ba12000","size":5,"soft_fail":false,"target_va":5383985360,"va":5381846944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2117104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8f04d2000","size":5,"soft_fail":false,"target_va":5383963792,"va":5381846683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2125611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e82b6f2000","size":5,"soft_fail":false,"target_va":5383972224,"va":5381846608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2117884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8fc502000","size":5,"soft_fail":false,"target_va":5383964912,"va":5381847023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2116517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e8a54b2000","size":5,"soft_fail":false,"target_va":5383962816,"va":5381846294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6003623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411020096,"raw_hex":"e8a79b5b00","size":5,"soft_fail":false,"target_va":5417024592,"va":5411020964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411020096,"raw_hex":"e8eb140000","size":5,"soft_fail":false,"target_va":5411026656,"va":5411021296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34772241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381846208,"raw_hex":"e811951202","size":5,"soft_fail":false,"target_va":5416619360,"va":5381847114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":609,"record_offset":27076,"parent_record_offset":24600,"ancestor_record_offset":24600,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":27344,"parent_record_offset":24600,"ancestor_record_offset":24600,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":24784,"parent_record_offset":24600,"ancestor_record_offset":24600,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$4::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":609,"record_offset":27048,"parent_record_offset":24600,"ancestor_record_offset":24600,"inlinee":38821,"source_line":13,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":609,"record_offset":27104,"parent_record_offset":24600,"ancestor_record_offset":24600,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":609,"record_offset":27260,"parent_record_offset":24600,"ancestor_record_offset":24600,"inlinee":7519,"source_line":696,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":26240,"parent_record_offset":24600,"ancestor_record_offset":24600,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":24840,"parent_record_offset":24600,"ancestor_record_offset":24600,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":25228,"parent_record_offset":24600,"ancestor_record_offset":24600,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":26568,"parent_record_offset":26540,"ancestor_record_offset":24600,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":26596,"parent_record_offset":26568,"ancestor_record_offset":24600,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":26516,"parent_record_offset":26488,"ancestor_record_offset":24600,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":26540,"parent_record_offset":26516,"ancestor_record_offset":24600,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":26656,"parent_record_offset":26540,"ancestor_record_offset":24600,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":609,"record_offset":26768,"parent_record_offset":26744,"ancestor_record_offset":24600,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":609,"record_offset":26620,"parent_record_offset":26596,"ancestor_record_offset":24600,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":609,"record_offset":26388,"parent_record_offset":26348,"ancestor_record_offset":24600,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":25176,"parent_record_offset":25148,"ancestor_record_offset":24600,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":609,"record_offset":27004,"parent_record_offset":26348,"ancestor_record_offset":24600,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":609,"record_offset":26720,"parent_record_offset":26488,"ancestor_record_offset":24600,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":25648,"parent_record_offset":25624,"ancestor_record_offset":24600,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":25624,"parent_record_offset":25600,"ancestor_record_offset":24600,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":609,"record_offset":25832,"parent_record_offset":25804,"ancestor_record_offset":24600,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":609,"record_offset":25932,"parent_record_offset":25904,"ancestor_record_offset":24600,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":609,"record_offset":25904,"parent_record_offset":25876,"ancestor_record_offset":24600,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":25576,"parent_record_offset":25552,"ancestor_record_offset":24600,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":25600,"parent_record_offset":25576,"ancestor_record_offset":24600,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":25960,"parent_record_offset":25932,"ancestor_record_offset":24600,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":609,"record_offset":25552,"parent_record_offset":25436,"ancestor_record_offset":24600,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$2::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":609,"record_offset":24808,"parent_record_offset":24784,"ancestor_record_offset":24600,"inlinee":19477,"source_line":31,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":24968,"parent_record_offset":24944,"ancestor_record_offset":24600,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":609,"record_offset":25264,"parent_record_offset":25228,"ancestor_record_offset":24600,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":25404,"parent_record_offset":25372,"ancestor_record_offset":24600,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":609,"record_offset":25372,"parent_record_offset":25336,"ancestor_record_offset":24600,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":25312,"parent_record_offset":25288,"ancestor_record_offset":24600,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":25336,"parent_record_offset":25312,"ancestor_record_offset":24600,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":25288,"parent_record_offset":25264,"ancestor_record_offset":24600,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":609,"record_offset":25876,"parent_record_offset":25336,"ancestor_record_offset":24600,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":24944,"parent_record_offset":24912,"ancestor_record_offset":24600,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":24912,"parent_record_offset":24884,"ancestor_record_offset":24600,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":25068,"parent_record_offset":25036,"ancestor_record_offset":24600,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":25124,"parent_record_offset":25068,"ancestor_record_offset":24600,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":25036,"parent_record_offset":25008,"ancestor_record_offset":24600,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":24884,"parent_record_offset":24840,"ancestor_record_offset":24600,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":609,"record_offset":25148,"parent_record_offset":25124,"ancestor_record_offset":24600,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":25436,"parent_record_offset":25404,"ancestor_record_offset":24600,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":25720,"parent_record_offset":25436,"ancestor_record_offset":24600,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":25692,"parent_record_offset":25436,"ancestor_record_offset":24600,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":609,"record_offset":25496,"parent_record_offset":25436,"ancestor_record_offset":24600,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":609,"record_offset":27464,"parent_record_offset":27400,"ancestor_record_offset":24600,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":609,"record_offset":27168,"parent_record_offset":27132,"ancestor_record_offset":24600,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":609,"record_offset":27488,"parent_record_offset":27464,"ancestor_record_offset":24600,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":609,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":24600,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":609,"record_offset":27372,"parent_record_offset":27344,"ancestor_record_offset":24600,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":609,"record_offset":27132,"parent_record_offset":27104,"ancestor_record_offset":24600,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":609,"record_offset":27400,"parent_record_offset":27372,"ancestor_record_offset":24600,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":609,"record_offset":27224,"parent_record_offset":27132,"ancestor_record_offset":24600,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":609,"record_offset":27436,"parent_record_offset":27400,"ancestor_record_offset":24600,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":26276,"parent_record_offset":26240,"ancestor_record_offset":24600,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":27596,"parent_record_offset":27564,"ancestor_record_offset":24600,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":28036,"parent_record_offset":27832,"ancestor_record_offset":24600,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":609,"record_offset":26416,"parent_record_offset":26388,"ancestor_record_offset":24600,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":26488,"parent_record_offset":26348,"ancestor_record_offset":24600,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":609,"record_offset":26692,"parent_record_offset":26488,"ancestor_record_offset":24600,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":609,"record_offset":26444,"parent_record_offset":26416,"ancestor_record_offset":24600,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":26348,"parent_record_offset":26312,"ancestor_record_offset":24600,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":609,"record_offset":26312,"parent_record_offset":26276,"ancestor_record_offset":24600,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::copy_to_staging::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 639
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14786
// 首条位置: {"module":1745,"inlinee":83846}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ahash::fallback_hash::impl$1::finish
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":1745,"record_offset":28292,"parent_record_offset":25720,"ancestor_record_offset":25720,"inlinee":6068,"source_line":196,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] ahash::fallback_hash::impl$1::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":1745,"record_offset":27392,"parent_record_offset":25720,"ancestor_record_offset":25720,"inlinee":15794,"source_line":171,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1745,"record_offset":28416,"parent_record_offset":25720,"ancestor_record_offset":25720,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":28608,"parent_record_offset":25720,"ancestor_record_offset":25720,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":28512,"parent_record_offset":25720,"ancestor_record_offset":25720,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::prepare_tile
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1745,"record_offset":25820,"parent_record_offset":25720,"ancestor_record_offset":25720,"inlinee":38841,"source_line":255,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_last_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/convert.rs
//     evidence={"module":1745,"record_offset":27940,"parent_record_offset":27900,"ancestor_record_offset":25720,"inlinee":15801,"source_line":134,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_u128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/convert.rs
//     evidence={"module":1745,"record_offset":27600,"parent_record_offset":27392,"ancestor_record_offset":25720,"inlinee":15797,"source_line":110,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/convert.rs
//     evidence={"module":4729,"record_offset":5384,"parent_record_offset":5284,"ancestor_record_offset":2512,"inlinee":15802,"source_line":92,"archive":"_pdb/C/modules/04729.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/convert.rs
//     evidence={"module":4729,"record_offset":5328,"parent_record_offset":5284,"ancestor_record_offset":2512,"inlinee":15800,"source_line":98,"archive":"_pdb/C/modules/04729.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::AHasher::large_update
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":1745,"record_offset":27452,"parent_record_offset":27392,"ancestor_record_offset":25720,"inlinee":15795,"source_line":111,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::operations::folded_multiply
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/operations.rs
//     evidence={"module":1745,"record_offset":27480,"parent_record_offset":27452,"ancestor_record_offset":25720,"inlinee":6062,"source_line":17,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] ahash::operations::read_small
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/operations.rs
//     evidence={"module":1745,"record_offset":27900,"parent_record_offset":27392,"ancestor_record_offset":25720,"inlinee":15799,"source_line":33,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_left
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1745,"record_offset":27564,"parent_record_offset":27540,"ancestor_record_offset":25720,"inlinee":6069,"source_line":2025,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":27300,"parent_record_offset":27276,"ancestor_record_offset":25720,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1745,"record_offset":27276,"parent_record_offset":25820,"ancestor_record_offset":25720,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":27508,"parent_record_offset":27480,"ancestor_record_offset":25720,"inlinee":6064,"source_line":2532,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":26368,"parent_record_offset":26272,"ancestor_record_offset":25720,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":27540,"parent_record_offset":27452,"ancestor_record_offset":25720,"inlinee":6732,"source_line":385,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1745,"record_offset":27832,"parent_record_offset":27740,"ancestor_record_offset":25720,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1745,"record_offset":27696,"parent_record_offset":27672,"ancestor_record_offset":25720,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1745,"record_offset":26608,"parent_record_offset":26552,"ancestor_record_offset":25720,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":26552,"parent_record_offset":26468,"ancestor_record_offset":25720,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":26468,"parent_record_offset":25820,"ancestor_record_offset":25720,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":27624,"parent_record_offset":27600,"ancestor_record_offset":25720,"inlinee":15798,"source_line":1952,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":27648,"parent_record_offset":27624,"ancestor_record_offset":25720,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1745,"record_offset":27672,"parent_record_offset":27648,"ancestor_record_offset":25720,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":26036,"parent_record_offset":25820,"ancestor_record_offset":25720,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":27332,"parent_record_offset":25820,"ancestor_record_offset":25720,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":28472,"parent_record_offset":28444,"ancestor_record_offset":25720,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":26272,"parent_record_offset":26176,"ancestor_record_offset":25720,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":27356,"parent_record_offset":27332,"ancestor_record_offset":25720,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1745,"record_offset":26100,"parent_record_offset":26036,"ancestor_record_offset":25720,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/01745.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::on_uploaded_to_texture
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 752
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20A9
// 首条位置: {"module":254,"inlinee":8361}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::on_texture_processed
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 761
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x760A
// 首条位置: {"module":517,"inlinee":30218}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":517,"record_offset":51404,"parent_record_offset":51376,"ancestor_record_offset":51376,"inlinee":30219,"source_line":156,"archive":"_pdb/C/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::build_tile_hasher
// 记录形式: compiled_procedure; instances=1
// 源码行: 767
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":603,"record_offset":1436,"rva":12941632,"va":5381650752}
// 编译实例 RVAs: 0xc57940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ahash::random_state::impl$5::build_hasher
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/random_state.rs
//     evidence={"module":603,"record_offset":1576,"parent_record_offset":1436,"ancestor_record_offset":1436,"inlinee":15824,"source_line":418,"archive":"_pdb/C/modules/00603.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::AHasher::from_random_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":603,"record_offset":1596,"parent_record_offset":1576,"ancestor_record_offset":1436,"inlinee":15825,"source_line":57,"archive":"_pdb/C/modules/00603.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$5::load_index_unchecked
// 记录形式: inline_instance, inlinee_declaration; instances=72
// 源码行: 788
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97B3
// 首条位置: {"module":609,"inlinee":38835}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::impl$52::deref_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 862
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":2356,"rva":7119936,"va":5375829056}
// 编译实例 RVAs: 0x6ca440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::impl$81::deref
// 记录形式: compiled_procedure; instances=1
// 源码行: 862
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":972,"rva":7119936,"va":5375829056}
// 编译实例 RVAs: 0x6ca440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::Cobblestone1Raster::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 870
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7705
// 首条位置: {"module":518,"inlinee":30469}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::Cobblestone2Raster::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 870
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7707
// 首条位置: {"module":518,"inlinee":30471}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::Cobblestone3Raster::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 870
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7709
// 首条位置: {"module":518,"inlinee":30473}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::GardenRaster::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 870
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x90C6
// 首条位置: {"module":581,"inlinee":37062}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::PathRaster::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 870, 871
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F98
// 首条位置: {"module":502,"record_offset":5268,"rva":9435632,"va":5378144752}
// 编译实例 RVAs: 0x8ff9f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140c86d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3699467","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378144752,"raw_hex":"e90b733800","size":5,"soft_fail":false,"target_va":5381844224,"va":5378144752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::PlantExemplarRaster::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 870
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x90C8
// 首条位置: {"module":581,"inlinee":37064}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::apply
// 记录形式: compiled_procedure; instances=1
// 源码行: 883
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":18216,"rva":13072,"va":5368722192}
// 编译实例 RVAs: 0x3310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::get_param
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 883, 900
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x298D
// 首条位置: {"module":77,"inlinee":10637}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$116::get_param
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":77,"record_offset":3822888,"parent_record_offset":3822860,"ancestor_record_offset":3822860,"inlinee":5950,"source_line":2160,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":77,"record_offset":3823324,"parent_record_offset":3823300,"ancestor_record_offset":3822860,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":77,"record_offset":3823372,"parent_record_offset":3823348,"ancestor_record_offset":3822860,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":77,"record_offset":3823348,"parent_record_offset":3823324,"ancestor_record_offset":3822860,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":3823300,"parent_record_offset":3823276,"ancestor_record_offset":3822860,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":3823276,"parent_record_offset":3823236,"ancestor_record_offset":3822860,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":3823236,"parent_record_offset":3823212,"ancestor_record_offset":3822860,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":77,"record_offset":3823612,"parent_record_offset":3823588,"ancestor_record_offset":3822860,"inlinee":4199,"source_line":348,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":77,"record_offset":3823588,"parent_record_offset":3823556,"ancestor_record_offset":3822860,"inlinee":4198,"source_line":317,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":77,"record_offset":3823528,"parent_record_offset":3823492,"ancestor_record_offset":3822860,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":77,"record_offset":3823464,"parent_record_offset":3822948,"ancestor_record_offset":3822860,"inlinee":4185,"source_line":125,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":77,"record_offset":3823492,"parent_record_offset":3823464,"ancestor_record_offset":3822860,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":77,"record_offset":3822988,"parent_record_offset":3822948,"ancestor_record_offset":3822860,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":77,"record_offset":3823556,"parent_record_offset":3823492,"ancestor_record_offset":3822860,"inlinee":4197,"source_line":132,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":77,"record_offset":3823036,"parent_record_offset":3823012,"ancestor_record_offset":3822860,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":77,"record_offset":3823012,"parent_record_offset":3822988,"ancestor_record_offset":3822860,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":77,"record_offset":3823212,"parent_record_offset":3823188,"ancestor_record_offset":3822860,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::get_param
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1698,"record_offset":3985808,"parent_record_offset":3985776,"ancestor_record_offset":3985748,"inlinee":4156,"source_line":803,"archive":"_pdb/C/modules/01698.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$8::get_param
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":77,"record_offset":3822920,"parent_record_offset":3822888,"ancestor_record_offset":3822860,"inlinee":4194,"source_line":882,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":77,"record_offset":3823680,"parent_record_offset":3823656,"ancestor_record_offset":3822860,"inlinee":4201,"source_line":2868,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":77,"record_offset":3823656,"parent_record_offset":3822948,"ancestor_record_offset":3822860,"inlinee":4200,"source_line":302,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":77,"record_offset":3822948,"parent_record_offset":3822920,"ancestor_record_offset":3822860,"inlinee":4195,"source_line":540,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":1698,"record_offset":3985836,"parent_record_offset":3985808,"ancestor_record_offset":3985748,"inlinee":4157,"source_line":641,"archive":"_pdb/C/modules/01698.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":77,"record_offset":3823124,"parent_record_offset":3823064,"ancestor_record_offset":3822860,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":77,"record_offset":3823148,"parent_record_offset":3823124,"ancestor_record_offset":3822860,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":77,"record_offset":3823188,"parent_record_offset":3823012,"ancestor_record_offset":3822860,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":77,"record_offset":3823064,"parent_record_offset":3823036,"ancestor_record_offset":3822860,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":77,"record_offset":3823096,"parent_record_offset":3823064,"ancestor_record_offset":3822860,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":77,"record_offset":3823416,"parent_record_offset":3823236,"ancestor_record_offset":3822860,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":3823704,"parent_record_offset":3823680,"ancestor_record_offset":3822860,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":77,"record_offset":3823728,"parent_record_offset":3823704,"ancestor_record_offset":3822860,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1698,"record_offset":3986112,"parent_record_offset":3986088,"ancestor_record_offset":3985748,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01698.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::init_access
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 883, 900
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FA5
// 首条位置: {"module":502,"record_offset":18488,"rva":9441424,"va":5378150544}
// 编译实例 RVAs: 0x901090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x141682d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t14163115","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378150544,"raw_hex":"e9ab1cd800","size":5,"soft_fail":false,"target_va":5392313664,"va":5378150544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::init_state
// 记录形式: compiled_procedure; instances=1
// 源码行: 883
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":12864,"rva":9440128,"va":5378149248}
// 编译实例 RVAs: 0x900b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-2470678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378149248,"raw_hex":"e8ea4cdaff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378149377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416d32f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14493090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378149248,"raw_hex":"e8a225dd00","size":5,"soft_fail":false,"target_va":5392642800,"va":5378149705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416d3dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14496058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378149248,"raw_hex":"e83a31dd00","size":5,"soft_fail":false,"target_va":5392645584,"va":5378149521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416d7430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14509703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378149248,"raw_hex":"e88766dd00","size":5,"soft_fail":false,"target_va":5392659504,"va":5378149796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416d9c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14520098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378149248,"raw_hex":"e8228fdd00","size":5,"soft_fail":false,"target_va":5392669712,"va":5378149609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14563439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378149248,"raw_hex":"e86f38de00","size":5,"soft_fail":false,"target_va":5392713328,"va":5378149884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416ea4b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14588106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378149248,"raw_hex":"e8ca98de00","size":5,"soft_fail":false,"target_va":5392737456,"va":5378149345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416eabf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14589874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378149248,"raw_hex":"e8b29fde00","size":5,"soft_fail":false,"target_va":5392739312,"va":5378149433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$116::init_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":502,"record_offset":13016,"parent_record_offset":12864,"ancestor_record_offset":12864,"inlinee":5946,"source_line":2123,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":13336,"parent_record_offset":13312,"ancestor_record_offset":12864,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":13360,"parent_record_offset":13336,"ancestor_record_offset":12864,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":13384,"parent_record_offset":13360,"ancestor_record_offset":12864,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":13312,"parent_record_offset":13224,"ancestor_record_offset":12864,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":13252,"parent_record_offset":13224,"ancestor_record_offset":12864,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":13224,"parent_record_offset":13200,"ancestor_record_offset":12864,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":13200,"parent_record_offset":13180,"ancestor_record_offset":12864,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::register::ComponentsRegistrator::new
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/component/register.rs
//     evidence={"module":502,"record_offset":13108,"parent_record_offset":13076,"ancestor_record_offset":12864,"inlinee":4111,"source_line":85,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::init_state
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":502,"record_offset":13044,"parent_record_offset":13016,"ancestor_record_offset":12864,"inlinee":4108,"source_line":761,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::components_registrator
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs
//     evidence={"module":502,"record_offset":13076,"parent_record_offset":13044,"ancestor_record_offset":12864,"inlinee":4109,"source_line":238,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":502,"record_offset":13280,"parent_record_offset":13252,"ancestor_record_offset":12864,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":13140,"parent_record_offset":13044,"ancestor_record_offset":12864,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::queue
// 记录形式: compiled_procedure; instances=1
// 源码行: 883
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":18352,"rva":13072,"va":5368722192}
// 编译实例 RVAs: 0x3310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::validate_param
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 883, 900
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2981
// 首条位置: {"module":77,"inlinee":10625}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$7::validate_param
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":1698,"record_offset":1756992,"parent_record_offset":1756968,"ancestor_record_offset":1756968,"inlinee":4222,"source_line":783,"archive":"_pdb/C/modules/01698.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$8::validate_param
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":77,"record_offset":1923788,"parent_record_offset":1923764,"ancestor_record_offset":1923764,"inlinee":4225,"source_line":862,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":77,"record_offset":1924148,"parent_record_offset":1924124,"ancestor_record_offset":1923764,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":77,"record_offset":1924196,"parent_record_offset":1924172,"ancestor_record_offset":1923764,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":77,"record_offset":1924172,"parent_record_offset":1924148,"ancestor_record_offset":1923764,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":1924124,"parent_record_offset":1924100,"ancestor_record_offset":1923764,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":1924100,"parent_record_offset":1924060,"ancestor_record_offset":1923764,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":77,"record_offset":1924060,"parent_record_offset":1924036,"ancestor_record_offset":1923764,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_vec.rs
//     evidence={"module":77,"record_offset":1924356,"parent_record_offset":1924332,"ancestor_record_offset":1923764,"inlinee":4188,"source_line":87,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":77,"record_offset":1924332,"parent_record_offset":1924312,"ancestor_record_offset":1923764,"inlinee":4186,"source_line":88,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/resource.rs
//     evidence={"module":77,"record_offset":1923816,"parent_record_offset":1923788,"ancestor_record_offset":1923764,"inlinee":4159,"source_line":332,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":77,"record_offset":1923864,"parent_record_offset":1923840,"ancestor_record_offset":1923764,"inlinee":4163,"source_line":56,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":77,"record_offset":1923840,"parent_record_offset":1923816,"ancestor_record_offset":1923764,"inlinee":4161,"source_line":405,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":77,"record_offset":1924036,"parent_record_offset":1924012,"ancestor_record_offset":1923764,"inlinee":4173,"source_line":406,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":77,"record_offset":1923952,"parent_record_offset":1923892,"ancestor_record_offset":1923764,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":77,"record_offset":1923972,"parent_record_offset":1923952,"ancestor_record_offset":1923764,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":77,"record_offset":1924288,"parent_record_offset":1923788,"ancestor_record_offset":1923764,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":77,"record_offset":1924012,"parent_record_offset":1923840,"ancestor_record_offset":1923764,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":77,"record_offset":1923892,"parent_record_offset":1923864,"ancestor_record_offset":1923764,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":77,"record_offset":1923924,"parent_record_offset":1923892,"ancestor_record_offset":1923764,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":77,"record_offset":1924240,"parent_record_offset":1924060,"ancestor_record_offset":1923764,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":77,"record_offset":1924676,"parent_record_offset":1924652,"ancestor_record_offset":1923764,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00077.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterRead::read
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 891
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F9A
// 首条位置: {"module":1806,"inlinee":28570}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::last_staged
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1806,"record_offset":118580,"parent_record_offset":118556,"ancestor_record_offset":118556,"inlinee":28511,"source_line":222,"archive":"_pdb/C/modules/01806.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::ordered_names
// 记录形式: compiled_procedure; instances=1
// 源码行: 908
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":5404,"rva":9435648,"va":5378144768}
// 编译实例 RVAs: 0x8ffa00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2466108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144768,"raw_hex":"e8c45edaff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378144791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9422589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144768,"raw_hex":"e8033970ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378144776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38475789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144768,"raw_hex":"e80d184b02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378144975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":502,"record_offset":5740,"parent_record_offset":5404,"ancestor_record_offset":5404,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":502,"record_offset":5572,"parent_record_offset":5404,"ancestor_record_offset":5404,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":5648,"parent_record_offset":5628,"ancestor_record_offset":5404,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":5668,"parent_record_offset":5648,"ancestor_record_offset":5404,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":5404,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":5628,"parent_record_offset":5600,"ancestor_record_offset":5404,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":502,"record_offset":5600,"parent_record_offset":5572,"ancestor_record_offset":5404,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":502,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":5404,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":502,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5404,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::write
// 记录形式: compiled_procedure; instances=1
// 源码行: 917
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":9312,"rva":9438016,"va":5378147136}
// 编译实例 RVAs: 0x900340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c856c0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3690356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378147136,"raw_hex":"e8744f3800","size":5,"soft_fail":false,"target_va":5381838528,"va":5378148167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38613436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378147136,"raw_hex":"e8bc314d02","size":5,"soft_fail":false,"target_va":5416761616,"va":5378148175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38909790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378147136,"raw_hex":"e85eb75102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378147261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38909866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378147136,"raw_hex":"e8aab75102","size":5,"soft_fail":false,"target_va":5417057088,"va":5378147217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6475138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378147136,"raw_hex":"e882cd6200","size":5,"soft_fail":false,"target_va":5384624080,"va":5378148937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t38473408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378147136,"raw_hex":"e8c00e4b02","size":5,"soft_fail":false,"target_va":5416621632,"va":5378148219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":502,"record_offset":9572,"parent_record_offset":9312,"ancestor_record_offset":9312,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":502,"record_offset":12412,"parent_record_offset":9312,"ancestor_record_offset":9312,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":502,"record_offset":9484,"parent_record_offset":9312,"ancestor_record_offset":9312,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::clear_composite
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":502,"record_offset":9628,"parent_record_offset":9312,"ancestor_record_offset":9312,"inlinee":28514,"source_line":244,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::replace_composite
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":502,"record_offset":9772,"parent_record_offset":9312,"ancestor_record_offset":9312,"inlinee":28515,"source_line":229,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":502,"record_offset":9596,"parent_record_offset":9572,"ancestor_record_offset":9312,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":9860,"parent_record_offset":9836,"ancestor_record_offset":9312,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":12444,"parent_record_offset":12412,"ancestor_record_offset":9312,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":502,"record_offset":9540,"parent_record_offset":9484,"ancestor_record_offset":9312,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":502,"record_offset":9512,"parent_record_offset":9484,"ancestor_record_offset":9312,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":502,"record_offset":9836,"parent_record_offset":9812,"ancestor_record_offset":9312,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":502,"record_offset":9812,"parent_record_offset":9772,"ancestor_record_offset":9312,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":502,"record_offset":9656,"parent_record_offset":9628,"ancestor_record_offset":9312,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed
// 记录形式: compiled_procedure; instances=1
// 源码行: 930
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":7232,"rva":9436448,"va":5378145568}
// 编译实例 RVAs: 0x8ffd20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ff6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378145568,"raw_hex":"e8bff3ffff","size":5,"soft_fail":false,"target_va":5378143968,"va":5378147100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38614503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378145568,"raw_hex":"e8e7354d02","size":5,"soft_fail":false,"target_va":5416761616,"va":5378147108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":502,"record_offset":7460,"parent_record_offset":7232,"ancestor_record_offset":7232,"inlinee":13036,"source_line":2126,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":502,"record_offset":7408,"parent_record_offset":7232,"ancestor_record_offset":7232,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::last_staged
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":502,"record_offset":7648,"parent_record_offset":7232,"ancestor_record_offset":7232,"inlinee":28511,"source_line":222,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":502,"record_offset":7488,"parent_record_offset":7460,"ancestor_record_offset":7232,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":502,"record_offset":7544,"parent_record_offset":7516,"ancestor_record_offset":7232,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":502,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":7232,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":502,"record_offset":7572,"parent_record_offset":7544,"ancestor_record_offset":7232,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":502,"record_offset":7600,"parent_record_offset":7572,"ancestor_record_offset":7232,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":502,"record_offset":7516,"parent_record_offset":7488,"ancestor_record_offset":7232,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging
// 记录形式: compiled_procedure; instances=1
// 源码行: 957
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":5828,"rva":9435872,"va":5378144992}
// 编译实例 RVAs: 0x8ffae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3432712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144992,"raw_hex":"e808613400","size":5,"soft_fail":false,"target_va":5381577744,"va":5378145027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c874c0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3701130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144992,"raw_hex":"e88a793800","size":5,"soft_fail":false,"target_va":5381846208,"va":5378145073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38476048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378144992,"raw_hex":"e810194b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5378145547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140c874c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3700668","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378144992,"raw_hex":"e9bc773800","size":5,"soft_fail":false,"target_va":5381846208,"va":5378145535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378144992,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5378145048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":502,"record_offset":6008,"parent_record_offset":5828,"ancestor_record_offset":5828,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":502,"record_offset":6120,"parent_record_offset":5828,"ancestor_record_offset":5828,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":502,"record_offset":6060,"parent_record_offset":5828,"ancestor_record_offset":5828,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":502,"record_offset":6148,"parent_record_offset":5828,"ancestor_record_offset":5828,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":502,"record_offset":6032,"parent_record_offset":6008,"ancestor_record_offset":5828,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":502,"record_offset":6084,"parent_record_offset":6060,"ancestor_record_offset":5828,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00502.jsonl.gz"}
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
// 完整结构及原始类型记录: world_raster.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::DirtyTilePublisher
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::world_raster::DirtyTilePublisher>", "constructor": "enum2$", "arguments": ["country_core::resources::world_raster::DirtyTilePublisher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::resources::world_raster::DirtyTilePublisher> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::resources::world_raster::DirtyTilePublisher>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::RasterBufferRef
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::world_raster::RasterBufferRef<512>", "constructor": "country_core::resources::world_raster::RasterBufferRef", "arguments": ["512"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >", "constructor": "utils::float_grid::impl$0::bilinear_sample_at_world_xz", "arguments": ["country_core::resources::world_raster::RasterBufferRef<512>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::StrokeBlendMode
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::world_raster::StrokeBlendMode>", "constructor": "enum2$", "arguments": ["country_core::resources::world_raster::StrokeBlendMode"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::resources::world_raster::StrokeBlendMode> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "enum2$<country_core::resources::world_raster::StrokeBlendMode>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<enum2$<country_core::resources::world_raster::StrokeBlendMode> >", "constructor": "core::fmt::impl$80::fmt", "arguments": ["enum2$<country_core::resources::world_raster::StrokeBlendMode>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::StrokeDeltaAccum
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 31
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::messages::Messages<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::message::messages::Messages", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<bevy_ecs::message::messages::Messages<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd> >", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["bevy_ecs::message::messages::Messages<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::message::message_reader::MessageReader", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<>", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$110::init_access<country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd> >", "constructor": "bevy_ecs::system::system_param::impl$110::init_access", "arguments": ["country_core::resources::world_raster_defs::AllWorldRasterWrite", "bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::message_registry::MessageRegistry::register_message<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::message::message_registry::MessageRegistry::register_message", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::message::message_writer::MessageWriter", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$134::init_access<country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter> >", "constructor": "bevy_ecs::system::system_param::impl$134::init_access", "arguments": ["country_core::resources::world_raster_defs::AllWorldRasterWrite", "bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::InsertTreeCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$8::init_state<bevy_ecs::message::messages::Messages<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd> >", "constructor": "bevy_ecs::system::system_param::impl$8::init_state", "arguments": ["bevy_ecs::message::messages::Messages<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::WorldRaster
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 32
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::world_raster::WorldRaster<512>", "constructor": "country_core::resources::world_raster::WorldRaster", "arguments": ["512"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::world_raster::WorldRaster<512> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::world_raster::WorldRaster<512>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$12,country_core::systems::paths::draw_path::draw_path::closure_env$13,country_core::systems::paths::draw_path::draw_path::closure_env$14>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$12", "country_core::systems::paths::draw_path::draw_path::closure_env$13", "country_core::systems::paths::draw_path::draw_path::closure_env$14"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> >", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster> >", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2,country_core::systems::garden::draw_garden::draw_garden::closure_env$3>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::garden::draw_garden::draw_garden::closure_env$1", "country_core::systems::garden::draw_garden::draw_garden::closure_env$2", "country_core::systems::garden::draw_garden::draw_garden::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$6,country_core::systems::paths::draw_path::draw_path::closure_env$7,country_core::systems::paths::draw_path::draw_path::closure_env$8>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$6", "country_core::systems::paths::draw_path::draw_path::closure_env$7", "country_core::systems::paths::draw_path::draw_path::closure_env$8"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$3<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$4<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$5<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster> >", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$3<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$4<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$5<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> >", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>", "country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$9,country_core::systems::paths::draw_path::draw_path::closure_env$10,country_core::systems::paths::draw_path::draw_path::closure_env$11>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$9", "country_core::systems::paths::draw_path::draw_path::closure_env$10", "country_core::systems::paths::draw_path::draw_path::closure_env$11"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$0", "country_core::systems::paths::draw_path::draw_path::closure_env$1", "country_core::systems::paths::draw_path::draw_path::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$3,country_core::systems::paths::draw_path::draw_path::closure_env$4,country_core::systems::paths::draw_path::draw_path::closure_env$5>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$3", "country_core::systems::paths::draw_path::draw_path::closure_env$4", "country_core::systems::paths::draw_path::draw_path::closure_env$5"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::world_raster::WorldRaster::new<512>", "constructor": "country_core::resources::world_raster::WorldRaster::new", "arguments": ["512"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::world_raster::WorldRaster::subscribe<512>", "constructor": "country_core::resources::world_raster::WorldRaster::subscribe", "arguments": ["512"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$2", "arguments": ["country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3,system_water::draw_water::draw_water::closure_env$4>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "system_water::draw_water::draw_water::closure_env$2", "system_water::draw_water::draw_water::closure_env$3", "system_water::draw_water::draw_water::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: world_raster.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-production-authority/three-version-path-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/world_raster.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/world_raster.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/world_raster.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: world_raster.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 884; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []
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
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::DirtyTilePublisher::secondary
// 信息 ID: 6b53304faa5289b767dbcf07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=518, inline_record=109264; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=518, inline_record=109408; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=518, inline_record=109552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::DirtyTilePublisher::shared
// 信息 ID: 240b42c08c5f383e8b25a802
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=518, inline_record=108880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::DirtyTilePublisher::unique
// 信息 ID: 50f42229d76dc6d862cb96f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_water::impl$0::build @ 0x6ebba7 -> 0xc57980 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::garden::impl$0::build; module=581, inline_record=146508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::garden::impl$0::build; module=581, inline_record=146592; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::as_float_grid
// 信息 ID: 93ccbec0273a9db78440995d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 46 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs; module=470, inline_record=772; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors; module=470, inline_record=17320; 不是独立调用边。
// [PDB 内联宿主] system_decorator::audio::op_stairs_audio::op_stairs_audio; module=493, inline_record=10980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_system; module=549, inline_record=42232; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::walk::impl$2::constrain_to_be_above_terrain; module=549, inline_record=84276; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_system; module=588, inline_record=44260; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::draw_path::PathRastersResources::as_float_grid; module=593, inline_record=94356; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::paths::draw_path::PathRastersResources::as_float_grid; module=593, inline_record=94384; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::clear_composite
// 信息 ID: ab1beb27b85f9258c3804a21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=9628; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=10048; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=10472; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=10896; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=11508; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=11932; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=12136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::last_staged
// 信息 ID: 8be772c238b20fc3eebe0796
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=502, inline_record=7648; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=502, inline_record=7920; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=502, inline_record=8192; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=502, inline_record=8464; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=502, inline_record=8736; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=502, inline_record=9008; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed; module=502, inline_record=9280; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot_save_data; module=1806, inline_record=118580; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::new<512>
// 信息 ID: 9d9dfef8bd1c2b15dad48a7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::WorldRaster::new -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::world_raster_defs::PathRaster::new @ 0x8ff9f0 -> 0xc86d00 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::impl$0::build @ 0x9c449a -> 0xc86d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::impl$0::build @ 0x9c44e1 -> 0xc86d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::impl$0::build @ 0x9c4528 -> 0xc86d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::impl$0::build @ 0x9c4595 -> 0xc86d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::garden::impl$0::build @ 0xbcfdc9 -> 0xc86d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::garden::impl$0::build @ 0xbcfe15 -> 0xc86d00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::prepare_tile
// 信息 ID: c926c4b692738ed689983ec9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512> >; module=1745, inline_record=25820; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512> >; module=4729, inline_record=2612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::replace_composite
// 信息 ID: 7f582873f248e5edd9ad1e98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=9772; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=10196; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=10620; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=11044; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=11320; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=11744; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write; module=502, inline_record=12284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::subscribe<512>
// 信息 ID: 3fef286cc05738d9fc9d5aab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fc68acc99a848e3e9f4a896d: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [类型关系] country_core::resources::world_raster::WorldRaster::subscribe -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> @ 0x116ba5f -> 0xc87080 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > @ 0x1954bff -> 0xc87080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids @ 0x28c6a5d -> 0x2b20e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids @ 0x28c6aae -> 0x2b20e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids @ 0x28c6aff -> 0x2b20e30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::ws_to_px
// 信息 ID: 6b1fa37ce1cd243d890032c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster::impl$3::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >; module=609, inline_record=31736; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$3::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >; module=609, inline_record=32460; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$3::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=609, inline_record=37628; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$3::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=609, inline_record=38352; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$3::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=3882, inline_record=7404; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$3::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >; module=3882, inline_record=8128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: c7415dce61662371930152a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::world_raster::_::impl$0::deserialize::__Field> >; module=1803, inline_record=490036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: c07a1b55b80674fc0f2dac72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::variant_seed<core::marker::PhantomData<country_core::resources::world_raster::_::impl$0::deserialize::__Field> >; module=1803, inline_record=489912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 4c2f69178dbc03c7bb9f8953
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=609, inline_record=18492; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_enum<country_core::resources::world_raster::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=435028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: ce5526d10264a418ea16dc05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=323908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: 8e0f8c8fb0212bead8baefb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::__Field> >; module=1803, inline_record=323784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: b2447d40552f7bed5823e6c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1804, inline_record=227548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 6ed955e792ab9c6b86b3112c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor> @ 0x1b8dfde -> 0xc86b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 0ca0e3076a10530392b26649
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::resources::world_raster::StrokeBlendMode> > @ 0x45d0a2 -> 0x1938a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,enum2$<country_core::resources::world_raster::StrokeBlendMode> > @ 0x18b4e62 -> 0xc868e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::build_tile_hasher
// 信息 ID: cbb704ad7cd2bda6b14e348e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512> > @ 0x2afa72d -> 0xc57940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$16::fmt
// 信息 ID: 984693bdd56804a16f7482cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<enum2$<country_core::resources::world_raster::StrokeBlendMode> >; module=609, inline_record=56928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::clear_all<512>
// 信息 ID: a7c0eda59a2c1933cec1a2e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$3::clear_all -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa5ac7e -> 0xc88a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa5ac93 -> 0xc88a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa5aca8 -> 0xc88a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa5add7 -> 0xc88a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa5adec -> 0xc88a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa5ae01 -> 0xc88a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa5ae16 -> 0xc88a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::clear_buffer<512>
// 信息 ID: 8ea74d285f74f11a67017e80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$3::clear_buffer -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64816 -> 0xc87180 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb66fd7 -> 0xc87180 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6978a -> 0xc87180 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c066 -> 0xc87180 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6e826 -> 0xc87180 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb710b6 -> 0xc87180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::commit_scratch_to_composite<512>
// 信息 ID: 385be40795b68bb6ea99be81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$3::commit_scratch_to_composite -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb66041 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb687cd -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6b0dd -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6d891 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb70132 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb728e1 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$12,country_core::systems::paths::draw_path::draw_path::closure_env$13,country_core::systems::paths::draw_path::draw_path::closure_env$14> @ 0xc5b076 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xc5c1cc -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster> > @ 0xc5d345 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2,country_core::systems::garden::draw_garden::draw_garden::closure_env$3> @ 0xc5e466 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$6,country_core::systems::paths::draw_path::draw_path::closure_env$7,country_core::systems::paths::draw_path::draw_path::closure_env$8> @ 0xc5f746 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$3<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$4<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$5<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xc609fc -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xc61bbc -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$9,country_core::systems::paths::draw_path::draw_path::closure_env$10,country_core::systems::paths::draw_path::draw_path::closure_env$11> @ 0xc62dd6 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xc640a0 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$3,country_core::systems::paths::draw_path::draw_path::closure_env$4,country_core::systems::paths::draw_path::draw_path::closure_env$5> @ 0xc652a6 -> 0xc886b0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::copy_composite_to_scratch<512>
// 信息 ID: 59d55a68a2db85c77537da6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$3::copy_composite_to_scratch -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65ea8 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb68634 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6aeee -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6d6f8 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6ffa0 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb72748 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$12,country_core::systems::paths::draw_path::draw_path::closure_env$13,country_core::systems::paths::draw_path::draw_path::closure_env$14> @ 0xc5bb63 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xc5cd92 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$6<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$7<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$8<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::PathRaster> > @ 0xc5de21 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2,country_core::systems::garden::draw_garden::draw_garden::closure_env$3> @ 0xc5f034 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$6,country_core::systems::paths::draw_path::draw_path::closure_env$7,country_core::systems::paths::draw_path::draw_path::closure_env$8> @ 0xc60314 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$3<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$4<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$5<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::Cobblestone2Raster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xc615c2 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$0<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$1<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster>,country_core::systems::paths::draw_path::erase_other_path_variants::closure_env$2<country_core::resources::world_raster_defs::Cobblestone1Raster,country_core::resources::world_raster_defs::PathRaster,country_core::resources::world_raster_defs::Cobblestone3Raster> > @ 0xc62782 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$9,country_core::systems::paths::draw_path::draw_path::closure_env$10,country_core::systems::paths::draw_path::draw_path::closure_env$11> @ 0xc639a4 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xc64b83 -> 0xc88320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$3,country_core::systems::paths::draw_path::draw_path::closure_env$4,country_core::systems::paths::draw_path::draw_path::closure_env$5> @ 0xc65e74 -> 0xc88320 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::copy_to_staging::closure$0
// 信息 ID: 59db693b1573c00c9608c2b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::world_raster::impl$3::copy_to_staging", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512> >; module=1745, inline_record=25720; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$3::copy_to_staging::closure_env$0<512> >; module=4729, inline_record=2512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::copy_to_staging<512>
// 信息 ID: 1c8f79dbde1597bc7c5a408c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$3::copy_to_staging -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 37 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x8ffb31 -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x8ffb7b -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x8ffbc5 -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x8ffc0f -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x8ffc62 -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x8ffcad -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x8ffcff -> 0xc874c0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64e04 -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65f2f -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb675d0 -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb686bb -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb69d92 -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6af6e -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c654 -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6d77f -> 0xc874c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6ee37 -> 0xc874c0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::extend_dirty
// 信息 ID: b9fe7eafb0e725df3e2e1cea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 51 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=6052; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=14088; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> >; module=571, inline_record=14144; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=22228; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=30288; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> >; module=571, inline_record=30344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=571, inline_record=38340; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=571, inline_record=46400; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::on_texture_processed
// 信息 ID: 45f8192cf2056fb5fcc5a191
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu; module=517, inline_record=51376; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=517, inline_record=80080; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=517, inline_record=80196; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=517, inline_record=80312; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks; module=517, inline_record=80428; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu; module=560, inline_record=11056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::on_uploaded_to_texture
// 信息 ID: 762f9bb151798046118af7f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>; module=254, inline_record=38084; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>; module=254, inline_record=44392; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>; module=254, inline_record=50700; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone1Raster>; module=254, inline_record=57016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone3Raster>; module=254, inline_record=63332; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PlantExemplarRaster>; module=254, inline_record=69648; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::Cobblestone2Raster>; module=254, inline_record=75964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::rasterize_stroke::closure$0
// 信息 ID: dcc538419414dda959a6147f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::world_raster::impl$3::rasterize_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 48 条；闭包环境类型引用 35 条。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=15036; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=15340; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=15540; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=27388; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=27692; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=27892; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=40252; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=40556; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::rasterize_stroke::closure$1
// 信息 ID: 284dde1b4df468e9831ae2f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::world_raster::impl$3::rasterize_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 48 条；闭包环境类型引用 35 条。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=7024; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=7328; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=7524; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=22280; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=22584; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=22784; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=30112; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=30416; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::rasterize_stroke::closure$2
// 信息 ID: b15f2c7c8057fc716ae9a0ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::world_raster::impl$3::rasterize_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2 <- zeno::mask::Mask::blend_into<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 48 条；闭包环境类型引用 35 条。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=4296; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=4604; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=4800; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=9748; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=10056; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=10252; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=12480; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$3::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> > > >; module=1842, inline_record=12788; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >
// 信息 ID: 052ff5e3e4413acfe8bbed6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::impl$3::rasterize_stroke -> ["512", "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$2::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 29 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64979 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb649fd -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65eee -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67139 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb671be -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb6867a -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb698e9 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6996e -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6af2d -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c1c9 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c24d -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6d73e -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6e989 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6ea0d -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6ffdf -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb71219 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >
// 信息 ID: c017439b1d3251b854684495
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::impl$3::rasterize_stroke -> ["512", "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$3::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 28 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb64979 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb649fd -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone3PathFilter> > @ 0xb65eee -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb67139 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb671be -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::DirtPathFilter> > @ 0xb6867a -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb698e9 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6996e -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xb6af2d -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c1c9 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6c24d -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone2PathFilter> > @ 0xb6d73e -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6e989 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6ea0d -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xb6ffdf -> 0xc87a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$1<country_core::systems::history::replay_path::Cobblestone1PathFilter> > @ 0xb71219 -> 0xc87a10 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$5::load_index_unchecked
// 信息 ID: 755f3776f5e9ca3c9c4ea4e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 64 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=609, inline_record=76808; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=609, inline_record=76880; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=609, inline_record=76952; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=609, inline_record=77020; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=609, inline_record=77180; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=609, inline_record=77256; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=609, inline_record=77332; 不是独立调用边。
// [PDB 内联宿主] utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >; module=609, inline_record=77404; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterRead::read
// 信息 ID: b38fff7cf4e86db46465b2c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot_save_data; module=1806, inline_record=118556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::ordered_names
// 信息 ID: 8abd1ab8cf61cad5c8ff59f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x1bfeb1f -> 0x8ffa00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging
// 信息 ID: 4f67647eb8c665922ae79600
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::world_raster::upload_world_raster_buffers_to_staging @ 0xb4f282 -> 0x8ffae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed
// 信息 ID: 0cdf381b98cbe6d3e7900b2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x1bfecb7 -> 0x8ffd20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::write
// 信息 ID: 3cb4beeb4d69af2c7b988c9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::init_game_state_from_snapshot @ 0x1bfa858 -> 0x900340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::Cobblestone1Raster::new
// 信息 ID: 3e950ad8c17849c63c8f0c3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=518, inline_record=109380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::Cobblestone2Raster::new
// 信息 ID: 5e4a0077cd24405ace959e9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=518, inline_record=109524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::Cobblestone3Raster::new
// 信息 ID: b1cd6c63d3d6bbfa139c1f74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=518, inline_record=109668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::GardenRaster::new
// 信息 ID: f8dfaaec6163bb194591959f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::garden::impl$0::build; module=581, inline_record=146564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::PathRaster::new
// 信息 ID: f0f82575afa904c3335120b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_water::impl$0::build @ 0x6ebbb6 -> 0x8ff9f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=518, inline_record=109696; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::PlantExemplarRaster::new
// 信息 ID: d5b9d4fb9c2e9fb47c2a387b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::garden::impl$0::build; module=581, inline_record=146648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::apply
// 信息 ID: f475f24e9f97e46a6f969a38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 000cf4217700e36734691cdd: void (*)(bevy_ecs::change_detection::ResMut<toolbar::quick_tool_palette::QuickToolPalette>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::system::query::Query<ref$<country_core::components::Ui2dExtent>,tuple$<> >,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// [泛型宿主/闭包/直接引用方参数候选] 00605d96b14cdb92382e4f3e: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// [泛型宿主/闭包/直接引用方参数候选] 0094895827cf75842dfd9081: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 0103bc1d475ed50b624e8f2f: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)
// [泛型宿主/闭包/直接引用方参数候选] 01a560f3d409056d7626bac9: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// [泛型宿主/闭包/直接引用方参数候选] 01e5ad06aacd15ca43210285: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)
// [泛型宿主/闭包/直接引用方参数候选] 01f77029bb05d937ca08973d: void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// [泛型宿主/闭包/直接引用方参数候选] 0203a61afd44dd916d707a37: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 其余候选接口见完整附档，共 881 条。
// 引用方: 原程序直接转移 13667 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> @ 0x2d7e -> 0x3310 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> @ 0x2dbe -> 0x3310 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> | country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> @ 0x78c77 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> | country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> @ 0x78c97 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$0>,never$> > | core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$0>,never$> >,tuple$<ref$<std::sync::once::OnceState> > > @ 0x791b6 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$1>,never$> > | core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$1>,never$> >,tuple$<ref$<std::sync::once::OnceState> > > @ 0x792ec -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::after<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)>,void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)> @ 0x795ac -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x796bf -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x796e3 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x79726 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)>,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)> @ 0x7d59f -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)>,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)> @ 0x7d5c3 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)>,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)> @ 0x7d60c -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] priority_queue::store::Store::with_capacity_and_hasher<alloc::string::String,u32,std::hash::random::RandomState> @ 0xc8e45 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] priority_queue::store::Store::with_capacity_and_hasher<alloc::string::String,u32,std::hash::random::RandomState> @ 0xc8de8 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] priority_queue::store::Store::with_capacity_and_hasher<alloc::string::String,u32,std::hash::random::RandomState> @ 0xc8e06 -> 0x3310 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::get_param
// 信息 ID: 8cbbeb196d1cfe6426f68676
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 65a9dffdb34114317d139994: void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)
// [泛型宿主/闭包/直接引用方参数候选] 676eb43b5a0b94cab578486d: void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// [泛型宿主/闭包/直接引用方参数候选] 9838df11118b4891eff7244c: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >; module=77, inline_record=3822860; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>; module=89, inline_record=12192; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >; module=1698, inline_record=3110856; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),tuple$<>,void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)> >; module=1698, inline_record=3985748; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),tuple$<>,void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)>; module=1700, inline_record=45080; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>; module=1719, inline_record=35244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::init_access
// 信息 ID: fbffdfd25b5ef4e27be38223
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system_param::impl$110::init_access<country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd> > @ 0x43bb5b -> 0x901090 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] bevy_ecs::system::system_param::impl$122::init_access<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >; module=585, inline_record=1364; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system_param::impl$134::init_access<country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetStoneFoundationColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter> >; module=585, inline_record=98372; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::init_state
// 信息 ID: 674086688f41c26a70f5ad81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 65a9dffdb34114317d139994: void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)
// [泛型宿主/闭包/直接引用方参数候选] 676eb43b5a0b94cab578486d: void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::initialize<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x325918 -> 0x900b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::initialize<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),tuple$<>,void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)> @ 0x1974125 -> 0x900b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::_::impl$0::init_state @ 0x1bf3a5a -> 0x900b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::queue
// 信息 ID: 64b3f013deb9d4e3b83b9a8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 000cf4217700e36734691cdd: void (*)(bevy_ecs::change_detection::ResMut<toolbar::quick_tool_palette::QuickToolPalette>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>,bevy_ecs::system::query::Query<ref$<country_core::components::Ui2dExtent>,tuple$<> >,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// [泛型宿主/闭包/直接引用方参数候选] 00605d96b14cdb92382e4f3e: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// [泛型宿主/闭包/直接引用方参数候选] 0094895827cf75842dfd9081: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 0103bc1d475ed50b624e8f2f: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)
// [泛型宿主/闭包/直接引用方参数候选] 01a560f3d409056d7626bac9: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// [泛型宿主/闭包/直接引用方参数候选] 01e5ad06aacd15ca43210285: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)
// [泛型宿主/闭包/直接引用方参数候选] 01f77029bb05d937ca08973d: void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// [泛型宿主/闭包/直接引用方参数候选] 0203a61afd44dd916d707a37: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 其余候选接口见完整附档，共 881 条。
// 引用方: 原程序直接转移 13667 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> @ 0x2d7e -> 0x3310 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> @ 0x2dbe -> 0x3310 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> | country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> @ 0x78c77 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> | country_core::shader::inc::window_color_customization::impl$0::descriptors<shabriri_rt::api::EmptyProtocol> @ 0x78c97 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$0>,never$> > | core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$0>,never$> >,tuple$<ref$<std::sync::once::OnceState> > > @ 0x791b6 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$1>,never$> > | core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<alloc::string::String,std::sync::once_lock::impl$0::get_or_init::closure_env$0<alloc::string::String,tiny_glade::app_config::impl$6::augment_args::closure_env$1>,never$> >,tuple$<ref$<std::sync::once::OnceState> > > @ 0x792ec -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::after<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)>,void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::mode_manager::RequestHistoryCommand>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)> @ 0x795ac -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x796bf -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x796e3 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::collision::world::Raycasting,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs> >,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)> > > @ 0x79726 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)>,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)> @ 0x7d59f -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)>,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)> @ 0x7d5c3 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>),void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,tuple$<> > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_render_slots::WallRenderSlots>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PlantExemplarRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallGroupSelection>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)>,bool (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ClearCanvasCmd>)> @ 0x7d60c -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] priority_queue::store::Store::with_capacity_and_hasher<alloc::string::String,u32,std::hash::random::RandomState> @ 0xc8e45 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] priority_queue::store::Store::with_capacity_and_hasher<alloc::string::String,u32,std::hash::random::RandomState> @ 0xc8de8 -> 0x3310 (direct_call); 同地址别名未裁定。
// [静态直接引用] priority_queue::store::Store::with_capacity_and_hasher<alloc::string::String,u32,std::hash::random::RandomState> @ 0xc8e06 -> 0x3310 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::validate_param
// 信息 ID: fe3dfb6cd0119d60f85506f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 65a9dffdb34114317d139994: void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)
// [泛型宿主/闭包/直接引用方参数候选] 676eb43b5a0b94cab578486d: void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// [泛型宿主/闭包/直接引用方参数候选] 9838df11118b4891eff7244c: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>; module=77, inline_record=1923764; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<>,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>; module=89, inline_record=34472; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>; module=1698, inline_record=1727940; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),tuple$<>,void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)>; module=1698, inline_record=1756968; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),tuple$<>,void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)>; module=1700, inline_record=106104; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>; module=1719, inline_record=100224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::impl$52::deref_mut
// 信息 ID: f6b61d6d88e64f620688aa77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 引用方: 原程序直接转移 206 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::load_language_library::load_language_library @ 0xf2741 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x15092b -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150952 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150bcc -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cd4 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cfe -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x14fde0 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150e75 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1879d5 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1875c2 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1878ea -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3be3 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3bf9 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3cb6 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x22453b -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224c02 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::impl$81::deref
// 信息 ID: 80dbc6253c6ef629e9d09790
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 引用方: 原程序直接转移 206 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::load_language_library::load_language_library @ 0xf2741 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x15092b -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150952 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150bcc -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cd4 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cfe -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x14fde0 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150e75 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1879d5 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1875c2 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1878ea -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3be3 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3bf9 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3cb6 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x22453b -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224c02 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
