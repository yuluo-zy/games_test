// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/backend/flush_screen_log.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3b2907a6ead8be0ec94d56244a3426a44e8d2c285161bfee4e0fe01a757136c9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: flush_screen_log.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 541
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::flush_screen_log::flush_error_log_to_screen
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":22404,"rva":10972112,"va":5379681232}
// 编译实例 RVAs: 0xa76bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t36934448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379681232,"raw_hex":"e830933302","size":5,"soft_fail":false,"target_va":5416616112,"va":5379681659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4002822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379681232,"raw_hex":"e8faebc2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379681521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10441459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379681232,"raw_hex":"e80dad60ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5379681902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10416850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379681232,"raw_hex":"e82e0d61ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5379681965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bb1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10206388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379681232,"raw_hex":"e84c4364ff","size":5,"soft_fail":false,"target_va":5369475504,"va":5379681887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36939878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379681232,"raw_hex":"e866a83302","size":5,"soft_fail":false,"target_va":5416621632,"va":5379681749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::DebugGui::log_to_screen
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1926819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379681232,"raw_hex":"e8a3661d00","size":5,"soft_fail":false,"target_va":5381608320,"va":5379681496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36938016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379681232,"raw_hex":"e820a13302","size":5,"soft_fail":false,"target_va":5416619696,"va":5379681675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t36934519","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379681232,"raw_hex":"e977933302","size":5,"soft_fail":false,"target_va":5416616304,"va":5379681780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":23092,"parent_record_offset":22404,"ancestor_record_offset":22404,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":541,"record_offset":23408,"parent_record_offset":22404,"ancestor_record_offset":22404,"inlinee":5049,"source_line":155,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":541,"record_offset":23584,"parent_record_offset":22404,"ancestor_record_offset":22404,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":541,"record_offset":23380,"parent_record_offset":22404,"ancestor_record_offset":22404,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":23640,"parent_record_offset":22404,"ancestor_record_offset":22404,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":541,"record_offset":23048,"parent_record_offset":22404,"ancestor_record_offset":22404,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":22588,"parent_record_offset":22404,"ancestor_record_offset":22404,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":23816,"parent_record_offset":23792,"ancestor_record_offset":22404,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":23840,"parent_record_offset":23816,"ancestor_record_offset":22404,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":23864,"parent_record_offset":23840,"ancestor_record_offset":22404,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":23792,"parent_record_offset":23736,"ancestor_record_offset":22404,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":23204,"parent_record_offset":23180,"ancestor_record_offset":22404,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":23764,"parent_record_offset":23736,"ancestor_record_offset":22404,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":23736,"parent_record_offset":23712,"ancestor_record_offset":22404,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":23252,"parent_record_offset":23228,"ancestor_record_offset":22404,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":23228,"parent_record_offset":23204,"ancestor_record_offset":22404,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":23712,"parent_record_offset":23688,"ancestor_record_offset":22404,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":23180,"parent_record_offset":23092,"ancestor_record_offset":22404,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":23124,"parent_record_offset":23092,"ancestor_record_offset":22404,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":23152,"parent_record_offset":23092,"ancestor_record_offset":22404,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":541,"record_offset":23512,"parent_record_offset":23440,"ancestor_record_offset":22404,"inlinee":5060,"source_line":156,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":541,"record_offset":23948,"parent_record_offset":23924,"ancestor_record_offset":22404,"inlinee":4794,"source_line":174,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":541,"record_offset":23608,"parent_record_offset":23584,"ancestor_record_offset":22404,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":23664,"parent_record_offset":23640,"ancestor_record_offset":22404,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":541,"record_offset":23340,"parent_record_offset":23316,"ancestor_record_offset":22404,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":541,"record_offset":23480,"parent_record_offset":23440,"ancestor_record_offset":22404,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":23540,"parent_record_offset":23512,"ancestor_record_offset":22404,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":541,"record_offset":23292,"parent_record_offset":23092,"ancestor_record_offset":22404,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":541,"record_offset":23316,"parent_record_offset":23292,"ancestor_record_offset":22404,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":541,"record_offset":23440,"parent_record_offset":23408,"ancestor_record_offset":22404,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":22968,"parent_record_offset":22940,"ancestor_record_offset":22404,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":24604,"parent_record_offset":24416,"ancestor_record_offset":22404,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":22660,"parent_record_offset":22628,"ancestor_record_offset":22404,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":24696,"parent_record_offset":24664,"ancestor_record_offset":22404,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":22872,"parent_record_offset":22840,"ancestor_record_offset":22404,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":22684,"parent_record_offset":22660,"ancestor_record_offset":22404,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":22896,"parent_record_offset":22872,"ancestor_record_offset":22404,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":24628,"parent_record_offset":24604,"ancestor_record_offset":22404,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":24720,"parent_record_offset":24696,"ancestor_record_offset":22404,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":541,"record_offset":22840,"parent_record_offset":22812,"ancestor_record_offset":22404,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":541,"record_offset":22812,"parent_record_offset":22784,"ancestor_record_offset":22404,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":24416,"parent_record_offset":24380,"ancestor_record_offset":22404,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":22940,"parent_record_offset":22748,"ancestor_record_offset":22404,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":22748,"parent_record_offset":22720,"ancestor_record_offset":22404,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":22720,"parent_record_offset":22588,"ancestor_record_offset":22404,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":24380,"parent_record_offset":24352,"ancestor_record_offset":22404,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":541,"record_offset":22628,"parent_record_offset":22588,"ancestor_record_offset":22404,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":541,"record_offset":24664,"parent_record_offset":24380,"ancestor_record_offset":22404,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":541,"record_offset":22784,"parent_record_offset":22748,"ancestor_record_offset":22404,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::flush_screen_log::log_error_to_screen
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":17860,"rva":10970832,"va":5379679952}
// 编译实例 RVAs: 0xa766d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t36935826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e892983302","size":5,"soft_fail":false,"target_va":5416616112,"va":5379680281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4001406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e882f1c2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379680089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4001821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e8e3efc2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379680520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10957884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e8c4cb58ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379680071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10440018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e8aeb260ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5379680461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10415452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e8a41261ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5379680567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36670212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e8048b2f02","size":5,"soft_fail":false,"target_va":5416350384,"va":5379680167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37376939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e8ab533a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5379680112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36940738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e8c2ab3302","size":5,"soft_fail":false,"target_va":5416620794,"va":5379680051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36941256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e8c8ad3302","size":5,"soft_fail":false,"target_va":5416621632,"va":5379680371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36939394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379679952,"raw_hex":"e882a63302","size":5,"soft_fail":false,"target_va":5416619696,"va":5379680297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t36935903","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379679952,"raw_hex":"e9df983302","size":5,"soft_fail":false,"target_va":5416616304,"va":5379680396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":541,"record_offset":18536,"parent_record_offset":17860,"ancestor_record_offset":17860,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":19168,"parent_record_offset":17860,"ancestor_record_offset":17860,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":19716,"parent_record_offset":17860,"ancestor_record_offset":17860,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":541,"record_offset":18492,"parent_record_offset":17860,"ancestor_record_offset":17860,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":18032,"parent_record_offset":17860,"ancestor_record_offset":17860,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":18984,"parent_record_offset":18960,"ancestor_record_offset":17860,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":19008,"parent_record_offset":18984,"ancestor_record_offset":17860,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":19596,"parent_record_offset":19572,"ancestor_record_offset":17860,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":19620,"parent_record_offset":19596,"ancestor_record_offset":17860,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":19032,"parent_record_offset":19008,"ancestor_record_offset":17860,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":19644,"parent_record_offset":19620,"ancestor_record_offset":17860,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":18960,"parent_record_offset":18804,"ancestor_record_offset":17860,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":19572,"parent_record_offset":19516,"ancestor_record_offset":17860,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":19280,"parent_record_offset":19248,"ancestor_record_offset":17860,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":18740,"parent_record_offset":18712,"ancestor_record_offset":17860,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":19544,"parent_record_offset":19516,"ancestor_record_offset":17860,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":19516,"parent_record_offset":19492,"ancestor_record_offset":17860,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":19328,"parent_record_offset":19304,"ancestor_record_offset":17860,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":19304,"parent_record_offset":19280,"ancestor_record_offset":17860,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":18804,"parent_record_offset":18768,"ancestor_record_offset":17860,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":18768,"parent_record_offset":18740,"ancestor_record_offset":17860,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":19492,"parent_record_offset":19468,"ancestor_record_offset":17860,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":18848,"parent_record_offset":18804,"ancestor_record_offset":17860,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":18620,"parent_record_offset":18592,"ancestor_record_offset":17860,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":18648,"parent_record_offset":18620,"ancestor_record_offset":17860,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":18676,"parent_record_offset":18648,"ancestor_record_offset":17860,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":18592,"parent_record_offset":18564,"ancestor_record_offset":17860,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":541,"record_offset":18564,"parent_record_offset":18536,"ancestor_record_offset":17860,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":19248,"parent_record_offset":19196,"ancestor_record_offset":17860,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":19196,"parent_record_offset":19168,"ancestor_record_offset":17860,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":18712,"parent_record_offset":18676,"ancestor_record_offset":17860,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":541,"record_offset":18872,"parent_record_offset":18848,"ancestor_record_offset":17860,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":541,"record_offset":18920,"parent_record_offset":18896,"ancestor_record_offset":17860,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":541,"record_offset":18896,"parent_record_offset":18872,"ancestor_record_offset":17860,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":541,"record_offset":19088,"parent_record_offset":18676,"ancestor_record_offset":17860,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":19112,"parent_record_offset":19088,"ancestor_record_offset":17860,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":19396,"parent_record_offset":19196,"ancestor_record_offset":17860,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":19368,"parent_record_offset":19196,"ancestor_record_offset":17860,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":18412,"parent_record_offset":18384,"ancestor_record_offset":17860,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":19968,"parent_record_offset":19780,"ancestor_record_offset":17860,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":18104,"parent_record_offset":18072,"ancestor_record_offset":17860,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":20060,"parent_record_offset":20028,"ancestor_record_offset":17860,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":18316,"parent_record_offset":18284,"ancestor_record_offset":17860,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":18128,"parent_record_offset":18104,"ancestor_record_offset":17860,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":18340,"parent_record_offset":18316,"ancestor_record_offset":17860,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":19992,"parent_record_offset":19968,"ancestor_record_offset":17860,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":20084,"parent_record_offset":20060,"ancestor_record_offset":17860,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":541,"record_offset":18284,"parent_record_offset":18256,"ancestor_record_offset":17860,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":541,"record_offset":18256,"parent_record_offset":18228,"ancestor_record_offset":17860,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":19780,"parent_record_offset":19744,"ancestor_record_offset":17860,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":18384,"parent_record_offset":18192,"ancestor_record_offset":17860,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":18192,"parent_record_offset":18164,"ancestor_record_offset":17860,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":18164,"parent_record_offset":18032,"ancestor_record_offset":17860,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":19744,"parent_record_offset":19716,"ancestor_record_offset":17860,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":541,"record_offset":18072,"parent_record_offset":18032,"ancestor_record_offset":17860,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":541,"record_offset":20028,"parent_record_offset":19744,"ancestor_record_offset":17860,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":541,"record_offset":18228,"parent_record_offset":18192,"ancestor_record_offset":17860,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::flush_screen_log::log_warning_to_screen
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":20132,"rva":10971472,"va":5379680592}
// 编译实例 RVAs: 0xa76950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t36935186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e812963302","size":5,"soft_fail":false,"target_va":5416616112,"va":5379680921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4002046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e802efc2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379680729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4002461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e863edc2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379681160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10958524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e844c958ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379680711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10440658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e82eb060ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5379681101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10416092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e8241061ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5379681207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36669572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e884882f02","size":5,"soft_fail":false,"target_va":5416350384,"va":5379680807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37376299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e82b513a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5379680752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36940098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e842a93302","size":5,"soft_fail":false,"target_va":5416620794,"va":5379680691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36940616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e848ab3302","size":5,"soft_fail":false,"target_va":5416621632,"va":5379681011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36938754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379680592,"raw_hex":"e802a43302","size":5,"soft_fail":false,"target_va":5416619696,"va":5379680937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t36935263","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379680592,"raw_hex":"e95f963302","size":5,"soft_fail":false,"target_va":5416616304,"va":5379681036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":541,"record_offset":20808,"parent_record_offset":20132,"ancestor_record_offset":20132,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":21440,"parent_record_offset":20132,"ancestor_record_offset":20132,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":21988,"parent_record_offset":20132,"ancestor_record_offset":20132,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":541,"record_offset":20764,"parent_record_offset":20132,"ancestor_record_offset":20132,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":20304,"parent_record_offset":20132,"ancestor_record_offset":20132,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":21256,"parent_record_offset":21232,"ancestor_record_offset":20132,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":21280,"parent_record_offset":21256,"ancestor_record_offset":20132,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":21868,"parent_record_offset":21844,"ancestor_record_offset":20132,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":21892,"parent_record_offset":21868,"ancestor_record_offset":20132,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":21304,"parent_record_offset":21280,"ancestor_record_offset":20132,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":21916,"parent_record_offset":21892,"ancestor_record_offset":20132,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":21232,"parent_record_offset":21076,"ancestor_record_offset":20132,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":21844,"parent_record_offset":21788,"ancestor_record_offset":20132,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":21552,"parent_record_offset":21520,"ancestor_record_offset":20132,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":21012,"parent_record_offset":20984,"ancestor_record_offset":20132,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":21816,"parent_record_offset":21788,"ancestor_record_offset":20132,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":21788,"parent_record_offset":21764,"ancestor_record_offset":20132,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":21600,"parent_record_offset":21576,"ancestor_record_offset":20132,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":21576,"parent_record_offset":21552,"ancestor_record_offset":20132,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":21076,"parent_record_offset":21040,"ancestor_record_offset":20132,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":21040,"parent_record_offset":21012,"ancestor_record_offset":20132,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":21764,"parent_record_offset":21740,"ancestor_record_offset":20132,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":541,"record_offset":21120,"parent_record_offset":21076,"ancestor_record_offset":20132,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":20892,"parent_record_offset":20864,"ancestor_record_offset":20132,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":20920,"parent_record_offset":20892,"ancestor_record_offset":20132,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":20948,"parent_record_offset":20920,"ancestor_record_offset":20132,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":541,"record_offset":20864,"parent_record_offset":20836,"ancestor_record_offset":20132,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":541,"record_offset":20836,"parent_record_offset":20808,"ancestor_record_offset":20132,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":21520,"parent_record_offset":21468,"ancestor_record_offset":20132,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":21468,"parent_record_offset":21440,"ancestor_record_offset":20132,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":20984,"parent_record_offset":20948,"ancestor_record_offset":20132,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":541,"record_offset":21144,"parent_record_offset":21120,"ancestor_record_offset":20132,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":541,"record_offset":21192,"parent_record_offset":21168,"ancestor_record_offset":20132,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":541,"record_offset":21168,"parent_record_offset":21144,"ancestor_record_offset":20132,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":541,"record_offset":21360,"parent_record_offset":20948,"ancestor_record_offset":20132,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":21384,"parent_record_offset":21360,"ancestor_record_offset":20132,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":21668,"parent_record_offset":21468,"ancestor_record_offset":20132,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":21640,"parent_record_offset":21468,"ancestor_record_offset":20132,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":20684,"parent_record_offset":20656,"ancestor_record_offset":20132,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":22240,"parent_record_offset":22052,"ancestor_record_offset":20132,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":20376,"parent_record_offset":20344,"ancestor_record_offset":20132,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":22332,"parent_record_offset":22300,"ancestor_record_offset":20132,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":20588,"parent_record_offset":20556,"ancestor_record_offset":20132,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":20400,"parent_record_offset":20376,"ancestor_record_offset":20132,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":20612,"parent_record_offset":20588,"ancestor_record_offset":20132,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":22264,"parent_record_offset":22240,"ancestor_record_offset":20132,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":22356,"parent_record_offset":22332,"ancestor_record_offset":20132,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":541,"record_offset":20556,"parent_record_offset":20528,"ancestor_record_offset":20132,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":541,"record_offset":20528,"parent_record_offset":20500,"ancestor_record_offset":20132,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":22052,"parent_record_offset":22016,"ancestor_record_offset":20132,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":20656,"parent_record_offset":20464,"ancestor_record_offset":20132,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":20464,"parent_record_offset":20436,"ancestor_record_offset":20132,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":20436,"parent_record_offset":20304,"ancestor_record_offset":20132,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":541,"record_offset":22016,"parent_record_offset":21988,"ancestor_record_offset":20132,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":541,"record_offset":20344,"parent_record_offset":20304,"ancestor_record_offset":20132,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":541,"record_offset":22300,"parent_record_offset":22016,"ancestor_record_offset":20132,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":541,"record_offset":20500,"parent_record_offset":20464,"ancestor_record_offset":20132,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00541.jsonl.gz"}
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
// 完整结构及原始类型记录: flush_screen_log.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::backend::flush_screen_log::LogType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>", "constructor": "tuple$", "arguments": ["alloc::string::String", "country_core::systems::backend::flush_screen_log::LogType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::MutexGuard", "arguments": ["alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: flush_screen_log.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: flush_screen_log.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::flush_screen_log::flush_error_log_to_screen
// 信息 ID: 2b6cc0927f631fb49c961626
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 86d2c9c7ede139c54c7d002b: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> @ 0x1a1590 -> 0xa76bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> > @ 0x2c74f8 -> 0xa76bd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::flush_screen_log::log_error_to_screen
// 信息 ID: 8b7500254fb970a77b04a79c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7f29d7 -> 0xa766d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::maintain_wooden_gate_steps @ 0x7f45e2 -> 0xa766d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::maintain_wooden_platform_stairs @ 0x7f483f -> 0xa766d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0xb25c44 -> 0xa766d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xbff985 -> 0xa766d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1447b38 -> 0xa766d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::flush_screen_log::log_warning_to_screen
// 信息 ID: b473719b5c9cc33bc997e8fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain @ 0x18209a0 -> 0xa76950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
