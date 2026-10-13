// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/panic_reporter/panic_screenshot.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: dc240e27f3766126ef4b26b9d6707fcfff4f28ff9756ae1440847e36950f8e75
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: panic_screenshot.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 122, 228
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":122,"record_offset":18032,"rva":4162128,"va":5372871248}
// 编译实例 RVAs: 0x3f8250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11737690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e85a1ab300","size":5,"soft_fail":false,"target_va":5384609408,"va":5372871713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2806902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e876d42a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372871797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=2
//     target_aliases=51 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3566342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e8fa94c9ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5372871921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3513692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e8a462caff","size":5,"soft_fail":false,"target_va":5369358064,"va":5372871751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009f150
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3511186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e86e6ccaff","size":5,"soft_fail":false,"target_va":5369360720,"va":5372871901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140103f50
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3098160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e8d0b9d0ff","size":5,"soft_fail":false,"target_va":5369773904,"va":5372872059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43750250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e86a939b02","size":5,"soft_fail":false,"target_va":5416622208,"va":5372871953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t43704940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e86ce29a02","size":5,"soft_fail":false,"target_va":5416576832,"va":5372871887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t43704644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e844e19a02","size":5,"soft_fail":false,"target_va":5416576144,"va":5372871495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11113455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e8ef93a900","size":5,"soft_fail":false,"target_va":5383984848,"va":5372871388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11113900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e8ac95a900","size":5,"soft_fail":false,"target_va":5383985360,"va":5372871455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t10616396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e84cfea100","size":5,"soft_fail":false,"target_va":5383487696,"va":5372871295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_mapped_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t10850718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372871248,"raw_hex":"e89e91a500","size":5,"soft_fail":false,"target_va":5383722048,"va":5372871325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":18748,"parent_record_offset":18032,"ancestor_record_offset":18032,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":18428,"parent_record_offset":18032,"ancestor_record_offset":18032,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":18368,"parent_record_offset":18032,"ancestor_record_offset":18032,"inlinee":11351,"source_line":1943,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":18484,"parent_record_offset":18032,"ancestor_record_offset":18032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":122,"record_offset":18212,"parent_record_offset":18032,"ancestor_record_offset":18032,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":18804,"parent_record_offset":18032,"ancestor_record_offset":18032,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":18664,"parent_record_offset":18032,"ancestor_record_offset":18032,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":19140,"parent_record_offset":19116,"ancestor_record_offset":18032,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":19164,"parent_record_offset":19140,"ancestor_record_offset":18032,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":19188,"parent_record_offset":19164,"ancestor_record_offset":18032,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":19116,"parent_record_offset":19060,"ancestor_record_offset":18032,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":19088,"parent_record_offset":19060,"ancestor_record_offset":18032,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":19060,"parent_record_offset":19036,"ancestor_record_offset":18032,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":19036,"parent_record_offset":19012,"ancestor_record_offset":18032,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":122,"record_offset":18392,"parent_record_offset":18368,"ancestor_record_offset":18032,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":18452,"parent_record_offset":18428,"ancestor_record_offset":18032,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":18772,"parent_record_offset":18748,"ancestor_record_offset":18032,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":18940,"parent_record_offset":18916,"ancestor_record_offset":18032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":18596,"parent_record_offset":18532,"ancestor_record_offset":18032,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":18276,"parent_record_offset":18240,"ancestor_record_offset":18032,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":18688,"parent_record_offset":18664,"ancestor_record_offset":18032,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":18032,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":18300,"parent_record_offset":18276,"ancestor_record_offset":18032,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":18712,"parent_record_offset":18688,"ancestor_record_offset":18032,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":122,"record_offset":18508,"parent_record_offset":18484,"ancestor_record_offset":18032,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":18852,"parent_record_offset":18828,"ancestor_record_offset":18032,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":122,"record_offset":18828,"parent_record_offset":18804,"ancestor_record_offset":18032,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":122,"record_offset":18876,"parent_record_offset":18852,"ancestor_record_offset":18032,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":122,"record_offset":18240,"parent_record_offset":18212,"ancestor_record_offset":18032,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":122,"record_offset":18532,"parent_record_offset":18508,"ancestor_record_offset":18032,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":122,"record_offset":18332,"parent_record_offset":18240,"ancestor_record_offset":18032,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":122,"record_offset":18568,"parent_record_offset":18532,"ancestor_record_offset":18032,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::capture_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":122,"record_offset":12960,"rva":4159680,"va":5372868800}
// 编译实例 RVAs: 0x3f78c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..import_export..GraphImportable$GT$::export
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10623099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e87b18a200","size":5,"soft_fail":false,"target_va":5383492416,"va":5372869312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..rg..import_export..GraphImportable$GT$::import_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10623220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e8f418a200","size":5,"soft_fail":false,"target_va":5383492272,"va":5372869047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3563940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e85c9ec9ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5372869519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44187824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e8b040a202","size":5,"soft_fail":false,"target_va":5417057056,"va":5372869227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43752730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e81a9d9b02","size":5,"soft_fail":false,"target_va":5416622208,"va":5372869473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t43707412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e814ec9a02","size":5,"soft_fail":false,"target_va":5416576832,"va":5372869415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t43706786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e8a2e99a02","size":5,"soft_fail":false,"target_va":5416576144,"va":5372869353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11115978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e8ca9da900","size":5,"soft_fail":false,"target_va":5383984848,"va":5372868865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11116030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e8fe9da900","size":5,"soft_fail":false,"target_va":5383985360,"va":5372869325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::bind_entry_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t521619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e893f50700","size":5,"soft_fail":false,"target_va":5373390816,"va":5372869192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t10667391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e87fc5a200","size":5,"soft_fail":false,"target_va":5383536672,"va":5372869276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t10670003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e8b3cfa200","size":5,"soft_fail":false,"target_va":5383539152,"va":5372869144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::shader_lib::ShaderLibrary::get_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t18946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372868800,"raw_hex":"e8024a0000","size":5,"soft_fail":false,"target_va":5372888064,"va":5372869113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t43706690","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372868800,"raw_hex":"e942e99a02","size":5,"soft_fail":false,"target_va":5416576144,"va":5372869449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":122,"record_offset":13440,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":13564,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":13025,"source_line":3657,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":13300,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":13680,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":122,"record_offset":13144,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":122,"record_offset":13496,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphResourceApi::export
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":122,"record_offset":13624,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":13028,"source_line":105,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::import_mut_with_access_type
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":122,"record_offset":13468,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":12379,"source_line":1321,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":122,"record_offset":13596,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":122,"record_offset":13328,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":13021,"source_line":791,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":122,"record_offset":13384,"parent_record_offset":12960,"ancestor_record_offset":12960,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":122,"record_offset":13408,"parent_record_offset":13384,"ancestor_record_offset":12960,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":13788,"parent_record_offset":13728,"ancestor_record_offset":12960,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":13208,"parent_record_offset":13172,"ancestor_record_offset":12960,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":12960,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":13232,"parent_record_offset":13208,"ancestor_record_offset":12960,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::capture_panic_screenshot::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":122,"record_offset":13532,"parent_record_offset":13496,"ancestor_record_offset":12960,"inlinee":13024,"source_line":1502,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":122,"record_offset":13704,"parent_record_offset":13680,"ancestor_record_offset":12960,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":122,"record_offset":13172,"parent_record_offset":13144,"ancestor_record_offset":12960,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":122,"record_offset":13728,"parent_record_offset":13704,"ancestor_record_offset":12960,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":122,"record_offset":13264,"parent_record_offset":13172,"ancestor_record_offset":12960,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":122,"record_offset":13760,"parent_record_offset":13728,"ancestor_record_offset":12960,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::export_with_access_type
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":122,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":12960,"inlinee":12390,"source_line":1341,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/resource.rs
//     evidence={"module":122,"record_offset":13352,"parent_record_offset":13328,"ancestor_record_offset":12960,"inlinee":9291,"source_line":410,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":122,"record_offset":14048,"rva":4160432,"va":5372869552}
// 编译实例 RVAs: 0x3f7bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11788648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e868e1b300","size":5,"soft_fail":false,"target_va":5384659264,"va":5372870611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11819379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e87359b400","size":5,"soft_fail":false,"target_va":5384689616,"va":5372870232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11786619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e87bd9b300","size":5,"soft_fail":false,"target_va":5384656240,"va":5372869616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11786306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e842d8b300","size":5,"soft_fail":false,"target_va":5384656512,"va":5372870201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t2808550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e8e6da2a00","size":5,"soft_fail":false,"target_va":5375678704,"va":5372870149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3565640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e8b897c9ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5372871219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140146080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2826526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e8e2ded4ff","size":5,"soft_fail":false,"target_va":5370044544,"va":5372871065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140302c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1004085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e8cbadf0ff","size":5,"soft_fail":false,"target_va":5371866240,"va":5372870320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2958672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e850252d00","size":5,"soft_fail":false,"target_va":5375829056,"va":5372870379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d66e40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43446562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e822f19602","size":5,"soft_fail":false,"target_va":5416316480,"va":5372869913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d66e80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43446656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e880f19602","size":5,"soft_fail":false,"target_va":5416316544,"va":5372869883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d67dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43449722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e87afd9602","size":5,"soft_fail":false,"target_va":5416320464,"va":5372870737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43451605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e8d5049702","size":5,"soft_fail":false,"target_va":5416321856,"va":5372870246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11925011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e813f6b500","size":5,"soft_fail":false,"target_va":5384794864,"va":5372869848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43750679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e817959b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5372870916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t43751492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e844989b02","size":5,"soft_fail":false,"target_va":5416622320,"va":5372870823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::free_functions::save_buffer_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10092567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e817009a00","size":5,"soft_fail":false,"target_va":5382962992,"va":5372870420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t43705585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e8f1e49a02","size":5,"soft_fail":false,"target_va":5416576144,"va":5372870554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11115174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e8a69aa900","size":5,"soft_fail":false,"target_va":5383984848,"va":5372869669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11115255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372869552,"raw_hex":"e8f79aa900","size":5,"soft_fail":false,"target_va":5383985360,"va":5372870100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":122,"record_offset":14880,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":122,"record_offset":14992,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":122,"record_offset":16444,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":122,"record_offset":14616,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":6246,"source_line":818,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":122,"record_offset":14964,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":122,"record_offset":15956,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":14492,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":17344,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":15160,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":122,"record_offset":15800,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":122,"record_offset":14520,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::from_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":122,"record_offset":16040,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":9736,"source_line":674,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::save
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":122,"record_offset":16384,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":9735,"source_line":996,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::try_lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":122,"record_offset":14284,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":13029,"source_line":227,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":122,"record_offset":15716,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":122,"record_offset":14256,"parent_record_offset":14048,"ancestor_record_offset":14048,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":15596,"parent_record_offset":15572,"ancestor_record_offset":14048,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":15620,"parent_record_offset":15596,"ancestor_record_offset":14048,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":15644,"parent_record_offset":15620,"ancestor_record_offset":14048,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":122,"record_offset":15572,"parent_record_offset":15516,"ancestor_record_offset":14048,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":122,"record_offset":14928,"parent_record_offset":14904,"ancestor_record_offset":14048,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":15544,"parent_record_offset":15516,"ancestor_record_offset":14048,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":15516,"parent_record_offset":15492,"ancestor_record_offset":14048,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":122,"record_offset":15492,"parent_record_offset":15468,"ancestor_record_offset":14048,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":122,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":14048,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":122,"record_offset":16520,"parent_record_offset":16480,"ancestor_record_offset":14048,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":122,"record_offset":16480,"parent_record_offset":16444,"ancestor_record_offset":14048,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$7::try_from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":122,"record_offset":14640,"parent_record_offset":14616,"ancestor_record_offset":14048,"inlinee":9708,"source_line":258,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":122,"record_offset":16004,"parent_record_offset":15980,"ancestor_record_offset":14048,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":16168,"parent_record_offset":16144,"ancestor_record_offset":14048,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":14580,"parent_record_offset":14548,"ancestor_record_offset":14048,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":16192,"parent_record_offset":16168,"ancestor_record_offset":14048,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":122,"record_offset":16688,"parent_record_offset":16660,"ancestor_record_offset":14048,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":16120,"parent_record_offset":16092,"ancestor_record_offset":14048,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":16348,"parent_record_offset":16064,"ancestor_record_offset":14048,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":122,"record_offset":14904,"parent_record_offset":14880,"ancestor_record_offset":14048,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":122,"record_offset":15372,"parent_record_offset":15348,"ancestor_record_offset":14048,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":122,"record_offset":14664,"parent_record_offset":14640,"ancestor_record_offset":14048,"inlinee":9709,"source_line":1711,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":122,"record_offset":15920,"parent_record_offset":15856,"ancestor_record_offset":14048,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":122,"record_offset":14548,"parent_record_offset":14520,"ancestor_record_offset":14048,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":15280,"parent_record_offset":15216,"ancestor_record_offset":14048,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":14400,"parent_record_offset":14308,"ancestor_record_offset":14048,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":14344,"parent_record_offset":14308,"ancestor_record_offset":14048,"inlinee":5580,"source_line":2842,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":15304,"parent_record_offset":15280,"ancestor_record_offset":14048,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":14048,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":122,"record_offset":14368,"parent_record_offset":14344,"ancestor_record_offset":14048,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::check_image_fits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":122,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":14048,"inlinee":9682,"source_line":802,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":122,"record_offset":16092,"parent_record_offset":16064,"ancestor_record_offset":14048,"inlinee":9623,"source_line":807,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":122,"record_offset":16144,"parent_record_offset":16120,"ancestor_record_offset":14048,"inlinee":9625,"source_line":809,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":122,"record_offset":16256,"parent_record_offset":16232,"ancestor_record_offset":14048,"inlinee":9626,"source_line":810,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::save_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":122,"record_offset":16412,"parent_record_offset":16384,"ancestor_record_offset":14048,"inlinee":9681,"source_line":1111,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":122,"record_offset":15188,"parent_record_offset":15160,"ancestor_record_offset":14048,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::try_lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":122,"record_offset":14308,"parent_record_offset":14284,"ancestor_record_offset":14048,"inlinee":13030,"source_line":78,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":122,"record_offset":15216,"parent_record_offset":15188,"ancestor_record_offset":14048,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":122,"record_offset":14456,"parent_record_offset":14308,"ancestor_record_offset":14048,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":122,"record_offset":15252,"parent_record_offset":15216,"ancestor_record_offset":14048,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":122,"record_offset":15768,"parent_record_offset":15716,"ancestor_record_offset":14048,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":122,"record_offset":15740,"parent_record_offset":15716,"ancestor_record_offset":14048,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00122.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::ycbcr_to_srgb
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3303
// 首条位置: {"module":228,"inlinee":13059}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":228,"record_offset":15472,"parent_record_offset":15392,"ancestor_record_offset":15392,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x429E
// 首条位置: {"module":228,"inlinee":17054}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::ycbcr_to_srgb
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_screenshot.rs
//     evidence={"module":228,"record_offset":15392,"parent_record_offset":15312,"ancestor_record_offset":15312,"inlinee":13059,"source_line":135,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":228,"record_offset":15472,"parent_record_offset":15392,"ancestor_record_offset":15312,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00228.jsonl.gz"}
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
// 完整结构及原始类型记录: panic_screenshot.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::panic_reporter::panic_screenshot::GraphicsResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources>", "constructor": "core::option::Option", "arguments": ["tiny_glade::panic_reporter::panic_screenshot::GraphicsResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> >", "constructor": "enum2$", "arguments": ["core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> > >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<tiny_glade::panic_reporter::panic_screenshot::GraphicsResources> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: panic_screenshot.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: panic_screenshot.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >", "ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::capture_image
// 信息 ID: 4d49e74d4084f5f416747d4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39a877 -> 0x3f78c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources
// 信息 ID: 5a2c04b60f06c26894eab216
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x41bc61 -> 0x3f8250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot
// 信息 ID: 9f2cb05e26585493f67cfcfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x185643 -> 0x3f7bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure$0
// 信息 ID: 54974a2904aaa3c6b42267ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_screenshot::dump_screenshot", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<array$<u8,3>,2>,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$21::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0 <- core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$21::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$21::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; module=228, inline_record=15312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::ycbcr_to_srgb
// 信息 ID: abc5b6701109fcab05b03c20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$21::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; module=228, inline_record=15392; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<array$<u8,3>,2>,core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<array$<array$<u8,3>,2>,alloc::vec::impl$21::extend_trusted::closure_env$0<array$<array$<u8,3>,2>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Chunks<u8>,tiny_glade::panic_reporter::panic_screenshot::dump_screenshot::closure_env$0> > > >; module=228, inline_record=15572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
