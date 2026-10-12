// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 89cdd2b6eb98960a888c3282c2ad7ffe797a7d13f6a37054d2a659c53ad30dcc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wooden_arch_frame_systems.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 415, 1461
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wooden_arch_frame_systems::impl$0::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1461,"record_offset":2144,"rva":21013472,"va":5389722592}
// 编译实例 RVAs: 0x140a3e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26893168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389722592,"raw_hex":"e8705b9a01","size":5,"soft_fail":false,"target_va":5416616112,"va":5389722939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20482696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389722592,"raw_hex":"e87875c7fe","size":5,"soft_fail":false,"target_va":5369240448,"va":5389723139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20458210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389722592,"raw_hex":"e81ed5c7fe","size":5,"soft_fail":false,"target_va":5369265120,"va":5389723325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140cd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389722592,"raw_hex":"e826270000","size":5,"soft_fail":false,"target_va":5389733232,"va":5389723205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26628204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389722592,"raw_hex":"e86c509601","size":5,"soft_fail":false,"target_va":5416351056,"va":5389722847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<alloc::vec::drain::Drain<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389722592,"raw_hex":"e8e94a0000","size":5,"soft_fail":false,"target_va":5389742048,"va":5389722866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26898598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389722592,"raw_hex":"e8a6709a01","size":5,"soft_fail":false,"target_va":5416621632,"va":5389723029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26896736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389722592,"raw_hex":"e860699a01","size":5,"soft_fail":false,"target_va":5416619696,"va":5389722955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t26893239","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5389722592,"raw_hex":"e9b75b9a01","size":5,"soft_fail":false,"target_va":5416616304,"va":5389723060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":2852,"parent_record_offset":2144,"ancestor_record_offset":2144,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":2320,"parent_record_offset":2144,"ancestor_record_offset":2144,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":3340,"parent_record_offset":2144,"ancestor_record_offset":2144,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":1461,"record_offset":3172,"parent_record_offset":2144,"ancestor_record_offset":2144,"inlinee":5049,"source_line":155,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1461,"record_offset":3144,"parent_record_offset":2144,"ancestor_record_offset":2144,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":3600,"parent_record_offset":2144,"ancestor_record_offset":2144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1461,"record_offset":2808,"parent_record_offset":2144,"ancestor_record_offset":2144,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1461,"record_offset":2348,"parent_record_offset":2144,"ancestor_record_offset":2144,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1461,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":2144,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1461,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":2144,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1461,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":2144,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":3424,"parent_record_offset":3376,"ancestor_record_offset":2144,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":2888,"parent_record_offset":2852,"ancestor_record_offset":2144,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":3000,"parent_record_offset":2852,"ancestor_record_offset":2144,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":3376,"parent_record_offset":3340,"ancestor_record_offset":2144,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":3028,"parent_record_offset":2852,"ancestor_record_offset":2144,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":1461,"record_offset":3276,"parent_record_offset":3204,"ancestor_record_offset":2144,"inlinee":5060,"source_line":156,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":3564,"parent_record_offset":3376,"ancestor_record_offset":2144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1461,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":2144,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1461,"record_offset":3244,"parent_record_offset":3204,"ancestor_record_offset":2144,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":2144,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":3536,"parent_record_offset":3376,"ancestor_record_offset":2144,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1461,"record_offset":3056,"parent_record_offset":2852,"ancestor_record_offset":2144,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1461,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":2144,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":3204,"parent_record_offset":3172,"ancestor_record_offset":2144,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":2728,"parent_record_offset":2700,"ancestor_record_offset":2144,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":3852,"parent_record_offset":3664,"ancestor_record_offset":2144,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":2420,"parent_record_offset":2388,"ancestor_record_offset":2144,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":3944,"parent_record_offset":3912,"ancestor_record_offset":2144,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":2632,"parent_record_offset":2600,"ancestor_record_offset":2144,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":2144,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":2144,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":2144,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":2144,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":1461,"record_offset":2600,"parent_record_offset":2572,"ancestor_record_offset":2144,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":1461,"record_offset":2572,"parent_record_offset":2544,"ancestor_record_offset":2144,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1461,"record_offset":3664,"parent_record_offset":3628,"ancestor_record_offset":2144,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1461,"record_offset":2700,"parent_record_offset":2508,"ancestor_record_offset":2144,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1461,"record_offset":2508,"parent_record_offset":2480,"ancestor_record_offset":2144,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1461,"record_offset":2480,"parent_record_offset":2348,"ancestor_record_offset":2144,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1461,"record_offset":3628,"parent_record_offset":3600,"ancestor_record_offset":2144,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1461,"record_offset":2388,"parent_record_offset":2348,"ancestor_record_offset":2144,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1461,"record_offset":3912,"parent_record_offset":3628,"ancestor_record_offset":2144,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":1461,"record_offset":2544,"parent_record_offset":2508,"ancestor_record_offset":2144,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wooden_arch_frame_systems::maintain_wooden_arch_frames
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":415,"record_offset":280,"rva":8491296,"va":5377200416}
// 编译实例 RVAs: 0x819120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t39412086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e876615902","size":5,"soft_fail":false,"target_va":5416616112,"va":5377204021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t39412120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e898615902","size":5,"soft_fail":false,"target_va":5416616304,"va":5377204179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::hot
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6262249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8e98d5f00","size":5,"soft_fail":false,"target_va":5383464400,"va":5377202146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1523668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e82cc0e8ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377202351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t-1522752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8c0c3e8ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377201451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8480149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e86b9a7eff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377202336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7964526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8927886ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5377204969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7939698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e88ed986ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5377204813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ce3b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4501657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8674fbbff","size":5,"soft_fail":false,"target_va":5372699568,"va":5377201220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3722160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e85034c7ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377201195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t909869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e82de20d00","size":5,"soft_fail":false,"target_va":5378112288,"va":5377202414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t924540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e87c1b0e00","size":5,"soft_fail":false,"target_va":5378126944,"va":5377202399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098aae0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1510030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e88e0a1700","size":5,"soft_fail":false,"target_va":5378714336,"va":5377204301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2514247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8475d2600","size":5,"soft_fail":false,"target_va":5379716240,"va":5377201988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fab4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7936921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8991b7900","size":5,"soft_fail":false,"target_va":5385139392,"va":5377202466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140cd70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12529376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8e02ebf00","size":5,"soft_fail":false,"target_va":5389733232,"va":5377203851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140f1f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12538190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e84e51bf00","size":5,"soft_fail":false,"target_va":5389742576,"va":5377204381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141415ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12565810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e832bdbf00","size":5,"soft_fail":false,"target_va":5389769424,"va":5377203609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14147f4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12999962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e81a5dc600","size":5,"soft_fail":false,"target_va":5390202048,"va":5377202081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39149577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e809605502","size":5,"soft_fail":false,"target_va":5416353040,"va":5377203458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39854325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8f5206002","size":5,"soft_fail":false,"target_va":5417057056,"va":5377202726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wooden_arch_frame_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39852126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e85e186002","size":5,"soft_fail":false,"target_va":5417057072,"va":5377204941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39416977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e891745902","size":5,"soft_fail":false,"target_va":5416620769,"va":5377203787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39321494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e896ff5702","size":5,"soft_fail":false,"target_va":5416524720,"va":5377203221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::reserve_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7023506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8922b6b00","size":5,"soft_fail":false,"target_va":5384226192,"va":5377202681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7217299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e893206e00","size":5,"soft_fail":false,"target_va":5384418976,"va":5377201672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::num::int_log10::panic_for_nonpositive_argument
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39418524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e89c7a5902","size":5,"soft_fail":false,"target_va":5416622288,"va":5377203759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39417478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e886765902","size":5,"soft_fail":false,"target_va":5416621632,"va":5377204149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t6285537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8e1e85f00","size":5,"soft_fail":false,"target_va":5383487568,"va":5377202026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39415636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377200416,"raw_hex":"e8546f5902","size":5,"soft_fail":false,"target_va":5416619696,"va":5377204055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377200416,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5377202193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":12152,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":6796,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":6936,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":6824,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":415,"record_offset":12180,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":415,"record_offset":3016,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":415,"record_offset":2316,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":12264,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":415,"record_offset":7152,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":415,"record_offset":12236,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wooden_arch_frame_systems::despawn_wooden_arch_frames
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs
//     evidence={"module":415,"record_offset":3300,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26395,"source_line":85,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wooden_arch_frame_systems::maintain_deferred_wooden_arch_frame_buffer_disposal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs
//     evidence={"module":415,"record_offset":492,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26392,"source_line":77,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wooden_arch_frame_systems::spawn_wooden_arch_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs
//     evidence={"module":415,"record_offset":7252,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26398,"source_line":100,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":10528,"parent_record_offset":10504,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":10552,"parent_record_offset":10528,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":10576,"parent_record_offset":10552,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":10504,"parent_record_offset":10472,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":4552,"parent_record_offset":4464,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":415,"record_offset":10472,"parent_record_offset":10444,"ancestor_record_offset":280,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":415,"record_offset":10444,"parent_record_offset":7252,"ancestor_record_offset":280,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":10936,"parent_record_offset":10904,"ancestor_record_offset":280,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":11032,"parent_record_offset":11004,"ancestor_record_offset":280,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":4492,"parent_record_offset":4464,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":11004,"parent_record_offset":10968,"ancestor_record_offset":280,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":7076,"parent_record_offset":7052,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":10968,"parent_record_offset":10936,"ancestor_record_offset":280,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":4440,"parent_record_offset":4416,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":7004,"parent_record_offset":6964,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1324,"parent_record_offset":1240,"ancestor_record_offset":280,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":6964,"parent_record_offset":6936,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":10904,"parent_record_offset":10848,"ancestor_record_offset":280,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1204,"parent_record_offset":492,"ancestor_record_offset":280,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1240,"parent_record_offset":1204,"ancestor_record_offset":280,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1660,"parent_record_offset":1240,"ancestor_record_offset":280,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":280,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1352,"parent_record_offset":1240,"ancestor_record_offset":280,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":280,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":7580,"parent_record_offset":7552,"ancestor_record_offset":280,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":2636,"parent_record_offset":2404,"ancestor_record_offset":280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":415,"record_offset":4344,"parent_record_offset":4320,"ancestor_record_offset":280,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":415,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":280,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":415,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":280,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":415,"record_offset":10056,"parent_record_offset":10000,"ancestor_record_offset":280,"inlinee":26375,"source_line":710,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":415,"record_offset":10028,"parent_record_offset":10000,"ancestor_record_offset":280,"inlinee":26374,"source_line":27762,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":415,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":280,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":415,"record_offset":3356,"parent_record_offset":3300,"ancestor_record_offset":280,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":415,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":280,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":415,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":280,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":415,"record_offset":2380,"parent_record_offset":2340,"ancestor_record_offset":280,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":415,"record_offset":5448,"parent_record_offset":3300,"ancestor_record_offset":280,"inlinee":10329,"source_line":442,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":415,"record_offset":10752,"parent_record_offset":10712,"ancestor_record_offset":280,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":415,"record_offset":10792,"parent_record_offset":10752,"ancestor_record_offset":280,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":415,"record_offset":10656,"parent_record_offset":7252,"ancestor_record_offset":280,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":415,"record_offset":10712,"parent_record_offset":10656,"ancestor_record_offset":280,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":415,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":280,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/command_queue.rs
//     evidence={"module":415,"record_offset":11456,"parent_record_offset":10792,"ancestor_record_offset":280,"inlinee":11374,"source_line":96,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/command_queue.rs
//     evidence={"module":415,"record_offset":10848,"parent_record_offset":10792,"ancestor_record_offset":280,"inlinee":11372,"source_line":172,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$65::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":415,"record_offset":10352,"parent_record_offset":10092,"ancestor_record_offset":280,"inlinee":16295,"source_line":2016,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":415,"record_offset":10628,"parent_record_offset":7252,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":415,"record_offset":1408,"parent_record_offset":1240,"ancestor_record_offset":280,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":415,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":280,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":415,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":280,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":415,"record_offset":3068,"parent_record_offset":3040,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":415,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":415,"record_offset":4000,"parent_record_offset":3564,"ancestor_record_offset":280,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":415,"record_offset":3564,"parent_record_offset":3532,"ancestor_record_offset":280,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":415,"record_offset":3532,"parent_record_offset":3300,"ancestor_record_offset":280,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":415,"record_offset":3640,"parent_record_offset":3612,"ancestor_record_offset":280,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":415,"record_offset":3612,"parent_record_offset":3564,"ancestor_record_offset":280,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":415,"record_offset":3668,"parent_record_offset":3640,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":415,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":2452,"parent_record_offset":2404,"ancestor_record_offset":280,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":4520,"parent_record_offset":4492,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_ilog2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10288,"parent_record_offset":10256,"ancestor_record_offset":280,"inlinee":26404,"source_line":1751,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::ilog2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10256,"parent_record_offset":10092,"ancestor_record_offset":280,"inlinee":26403,"source_line":1639,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10192,"parent_record_offset":10092,"ancestor_record_offset":280,"inlinee":26401,"source_line":3806,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10216,"parent_record_offset":10192,"ancestor_record_offset":280,"inlinee":26402,"source_line":3775,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10164,"parent_record_offset":10092,"ancestor_record_offset":280,"inlinee":26400,"source_line":2300,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10136,"parent_record_offset":10092,"ancestor_record_offset":280,"inlinee":13037,"source_line":2344,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::ilog2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":415,"record_offset":10316,"parent_record_offset":10288,"ancestor_record_offset":280,"inlinee":14770,"source_line":1790,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":415,"record_offset":4064,"parent_record_offset":4000,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":415,"record_offset":3728,"parent_record_offset":3696,"ancestor_record_offset":280,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":280,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":4240,"parent_record_offset":4000,"ancestor_record_offset":280,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":280,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3148,"parent_record_offset":3068,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":280,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":280,"inlinee":10740,"source_line":2481,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3844,"parent_record_offset":3668,"ancestor_record_offset":280,"inlinee":10739,"source_line":2616,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":415,"record_offset":1748,"parent_record_offset":1712,"ancestor_record_offset":280,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":1804,"parent_record_offset":1712,"ancestor_record_offset":280,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":1632,"parent_record_offset":1240,"ancestor_record_offset":280,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=78
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":1464,"parent_record_offset":1240,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":415,"record_offset":1492,"parent_record_offset":1240,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":415,"record_offset":11228,"parent_record_offset":10848,"ancestor_record_offset":280,"inlinee":26405,"source_line":1473,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":1168,"parent_record_offset":1088,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":4176,"parent_record_offset":4088,"ancestor_record_offset":280,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":1136,"parent_record_offset":1088,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":280,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":7116,"parent_record_offset":6964,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":11252,"parent_record_offset":11228,"ancestor_record_offset":280,"inlinee":26406,"source_line":1998,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":415,"record_offset":7420,"parent_record_offset":7252,"ancestor_record_offset":280,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":415,"record_offset":1044,"parent_record_offset":492,"ancestor_record_offset":280,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":415,"record_offset":2508,"parent_record_offset":2404,"ancestor_record_offset":280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":415,"record_offset":2860,"parent_record_offset":2404,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":415,"record_offset":10384,"parent_record_offset":7252,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":415,"record_offset":2564,"parent_record_offset":2536,"ancestor_record_offset":280,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":415,"record_offset":2536,"parent_record_offset":2508,"ancestor_record_offset":280,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":415,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":415,"record_offset":3224,"parent_record_offset":3172,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":415,"record_offset":1088,"parent_record_offset":492,"ancestor_record_offset":280,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":964,"parent_record_offset":936,"ancestor_record_offset":280,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":2152,"parent_record_offset":1956,"ancestor_record_offset":280,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":652,"parent_record_offset":620,"ancestor_record_offset":280,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":2244,"parent_record_offset":2212,"ancestor_record_offset":280,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":868,"parent_record_offset":836,"ancestor_record_offset":280,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":280,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":280,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":280,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":280,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":415,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":280,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":415,"record_offset":3464,"parent_record_offset":3436,"ancestor_record_offset":280,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":415,"record_offset":3492,"parent_record_offset":3464,"ancestor_record_offset":280,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::device_address
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":415,"record_offset":9976,"parent_record_offset":7252,"ancestor_record_offset":280,"inlinee":26371,"source_line":129,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::device_address_impl
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":415,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":280,"inlinee":26372,"source_line":133,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":415,"record_offset":3412,"parent_record_offset":3300,"ancestor_record_offset":280,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":415,"record_offset":836,"parent_record_offset":808,"ancestor_record_offset":280,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":415,"record_offset":808,"parent_record_offset":780,"ancestor_record_offset":280,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":415,"record_offset":1956,"parent_record_offset":1908,"ancestor_record_offset":280,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":415,"record_offset":936,"parent_record_offset":740,"ancestor_record_offset":280,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":415,"record_offset":740,"parent_record_offset":712,"ancestor_record_offset":280,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":415,"record_offset":580,"parent_record_offset":492,"ancestor_record_offset":280,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":415,"record_offset":712,"parent_record_offset":580,"ancestor_record_offset":280,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":415,"record_offset":1908,"parent_record_offset":1876,"ancestor_record_offset":280,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":415,"record_offset":620,"parent_record_offset":580,"ancestor_record_offset":280,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":415,"record_offset":2212,"parent_record_offset":1908,"ancestor_record_offset":280,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":415,"record_offset":780,"parent_record_offset":740,"ancestor_record_offset":280,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wooden_arch_frame_systems::despawn_wooden_arch_frames::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs
//     evidence={"module":415,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":280,"inlinee":26397,"source_line":94,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wooden_arch_frame_systems::maintain_deferred_wooden_arch_frame_buffer_disposal::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs
//     evidence={"module":415,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":280,"inlinee":26394,"source_line":82,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wooden_arch_frame_systems::wooden_arch_frame_mesh_atlas_part_idx
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs
//     evidence={"module":415,"record_offset":10092,"parent_record_offset":7252,"ancestor_record_offset":280,"inlinee":26399,"source_line":164,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wooden_arch_frame_systems::maintain_deferred_wooden_arch_frame_buffer_disposal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6718
// 首条位置: {"module":415,"inlinee":26392}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1204,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":1876,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":415,"record_offset":1044,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":415,"record_offset":1088,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":415,"record_offset":580,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1324,"parent_record_offset":1240,"ancestor_record_offset":492,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1240,"parent_record_offset":1204,"ancestor_record_offset":492,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1660,"parent_record_offset":1240,"ancestor_record_offset":492,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1352,"parent_record_offset":1240,"ancestor_record_offset":492,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":492,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":415,"record_offset":1408,"parent_record_offset":1240,"ancestor_record_offset":492,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":415,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":492,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":415,"record_offset":1748,"parent_record_offset":1712,"ancestor_record_offset":492,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":1804,"parent_record_offset":1712,"ancestor_record_offset":492,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":1632,"parent_record_offset":1240,"ancestor_record_offset":492,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":1464,"parent_record_offset":1240,"ancestor_record_offset":492,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":415,"record_offset":1492,"parent_record_offset":1240,"ancestor_record_offset":492,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":1168,"parent_record_offset":1088,"ancestor_record_offset":492,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":1136,"parent_record_offset":1088,"ancestor_record_offset":492,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":964,"parent_record_offset":936,"ancestor_record_offset":492,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":2152,"parent_record_offset":1956,"ancestor_record_offset":492,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":652,"parent_record_offset":620,"ancestor_record_offset":492,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":2244,"parent_record_offset":2212,"ancestor_record_offset":492,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":868,"parent_record_offset":836,"ancestor_record_offset":492,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":492,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":492,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":492,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":415,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":492,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":415,"record_offset":836,"parent_record_offset":808,"ancestor_record_offset":492,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":415,"record_offset":808,"parent_record_offset":780,"ancestor_record_offset":492,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":415,"record_offset":1956,"parent_record_offset":1908,"ancestor_record_offset":492,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":415,"record_offset":936,"parent_record_offset":740,"ancestor_record_offset":492,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":415,"record_offset":740,"parent_record_offset":712,"ancestor_record_offset":492,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":415,"record_offset":712,"parent_record_offset":580,"ancestor_record_offset":492,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":415,"record_offset":1908,"parent_record_offset":1876,"ancestor_record_offset":492,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":415,"record_offset":620,"parent_record_offset":580,"ancestor_record_offset":492,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":415,"record_offset":2212,"parent_record_offset":1908,"ancestor_record_offset":492,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":415,"record_offset":780,"parent_record_offset":740,"ancestor_record_offset":492,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wooden_arch_frame_systems::maintain_deferred_wooden_arch_frame_buffer_disposal::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs
//     evidence={"module":415,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":492,"inlinee":26394,"source_line":82,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wooden_arch_frame_systems::maintain_deferred_wooden_arch_frame_buffer_disposal::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x671A
// 首条位置: {"module":415,"inlinee":26394}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wooden_arch_frame_systems::despawn_wooden_arch_frames
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x671B
// 首条位置: {"module":415,"inlinee":26395}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":415,"record_offset":3356,"parent_record_offset":3300,"ancestor_record_offset":3300,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":415,"record_offset":5448,"parent_record_offset":3300,"ancestor_record_offset":3300,"inlinee":10329,"source_line":442,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":415,"record_offset":3532,"parent_record_offset":3300,"ancestor_record_offset":3300,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":5476,"parent_record_offset":3300,"ancestor_record_offset":3300,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":415,"record_offset":3412,"parent_record_offset":3300,"ancestor_record_offset":3300,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":3300,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":3300,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":3300,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":4552,"parent_record_offset":4464,"ancestor_record_offset":3300,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":4492,"parent_record_offset":4464,"ancestor_record_offset":3300,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":3300,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":4440,"parent_record_offset":4416,"ancestor_record_offset":3300,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":3300,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":415,"record_offset":4344,"parent_record_offset":4320,"ancestor_record_offset":3300,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":415,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":3300,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":415,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":3300,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":415,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":3300,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":415,"record_offset":4000,"parent_record_offset":3564,"ancestor_record_offset":3300,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":415,"record_offset":3564,"parent_record_offset":3532,"ancestor_record_offset":3300,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":415,"record_offset":3640,"parent_record_offset":3612,"ancestor_record_offset":3300,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":415,"record_offset":3612,"parent_record_offset":3564,"ancestor_record_offset":3300,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":415,"record_offset":3668,"parent_record_offset":3640,"ancestor_record_offset":3300,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":415,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":3300,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":4520,"parent_record_offset":4492,"ancestor_record_offset":3300,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":415,"record_offset":4064,"parent_record_offset":4000,"ancestor_record_offset":3300,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":415,"record_offset":3728,"parent_record_offset":3696,"ancestor_record_offset":3300,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3964,"parent_record_offset":3640,"ancestor_record_offset":3300,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":4240,"parent_record_offset":4000,"ancestor_record_offset":3300,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":3300,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":3300,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":3300,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":3300,"inlinee":10740,"source_line":2481,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":3844,"parent_record_offset":3668,"ancestor_record_offset":3300,"inlinee":10739,"source_line":2616,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":4296,"parent_record_offset":4000,"ancestor_record_offset":3300,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":415,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":3300,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":4148,"parent_record_offset":4088,"ancestor_record_offset":3300,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":4176,"parent_record_offset":4088,"ancestor_record_offset":3300,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":415,"record_offset":4120,"parent_record_offset":4088,"ancestor_record_offset":3300,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3300,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":415,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":3300,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":415,"record_offset":3464,"parent_record_offset":3436,"ancestor_record_offset":3300,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":415,"record_offset":3492,"parent_record_offset":3464,"ancestor_record_offset":3300,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wooden_arch_frame_systems::despawn_wooden_arch_frames::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs
//     evidence={"module":415,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":3300,"inlinee":26397,"source_line":94,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wooden_arch_frame_systems::despawn_wooden_arch_frames::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x671D
// 首条位置: {"module":415,"inlinee":26397}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":3752,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":415,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":3752,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wooden_arch_frame_systems::spawn_wooden_arch_frame
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x671E
// 首条位置: {"module":415,"inlinee":26398}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":415,"record_offset":10444,"parent_record_offset":7252,"ancestor_record_offset":7252,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":415,"record_offset":10656,"parent_record_offset":7252,"ancestor_record_offset":7252,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":415,"record_offset":10628,"parent_record_offset":7252,"ancestor_record_offset":7252,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":415,"record_offset":10412,"parent_record_offset":7252,"ancestor_record_offset":7252,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":415,"record_offset":7420,"parent_record_offset":7252,"ancestor_record_offset":7252,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":415,"record_offset":10384,"parent_record_offset":7252,"ancestor_record_offset":7252,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferHandleExt::device_address
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":415,"record_offset":9976,"parent_record_offset":7252,"ancestor_record_offset":7252,"inlinee":26371,"source_line":129,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wooden_arch_frame_systems::wooden_arch_frame_mesh_atlas_part_idx
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs
//     evidence={"module":415,"record_offset":10092,"parent_record_offset":7252,"ancestor_record_offset":7252,"inlinee":26399,"source_line":164,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":10528,"parent_record_offset":10504,"ancestor_record_offset":7252,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":10552,"parent_record_offset":10528,"ancestor_record_offset":7252,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":7252,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":7988,"parent_record_offset":7964,"ancestor_record_offset":7252,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":10576,"parent_record_offset":10552,"ancestor_record_offset":7252,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":8012,"parent_record_offset":7988,"ancestor_record_offset":7252,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":10504,"parent_record_offset":10472,"ancestor_record_offset":7252,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":415,"record_offset":7940,"parent_record_offset":7884,"ancestor_record_offset":7252,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":415,"record_offset":10472,"parent_record_offset":10444,"ancestor_record_offset":7252,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":11140,"parent_record_offset":11116,"ancestor_record_offset":7252,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":10936,"parent_record_offset":10904,"ancestor_record_offset":7252,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":11032,"parent_record_offset":11004,"ancestor_record_offset":7252,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":7912,"parent_record_offset":7884,"ancestor_record_offset":7252,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":7252,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":11004,"parent_record_offset":10968,"ancestor_record_offset":7252,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":11188,"parent_record_offset":11164,"ancestor_record_offset":7252,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":11164,"parent_record_offset":11140,"ancestor_record_offset":7252,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":10968,"parent_record_offset":10936,"ancestor_record_offset":7252,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":415,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":7252,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":11116,"parent_record_offset":10848,"ancestor_record_offset":7252,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":11088,"parent_record_offset":10848,"ancestor_record_offset":7252,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":10904,"parent_record_offset":10848,"ancestor_record_offset":7252,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":11312,"parent_record_offset":10848,"ancestor_record_offset":7252,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":415,"record_offset":7580,"parent_record_offset":7552,"ancestor_record_offset":7252,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":415,"record_offset":10056,"parent_record_offset":10000,"ancestor_record_offset":7252,"inlinee":26375,"source_line":710,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":415,"record_offset":10028,"parent_record_offset":10000,"ancestor_record_offset":7252,"inlinee":26374,"source_line":27762,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":415,"record_offset":10752,"parent_record_offset":10712,"ancestor_record_offset":7252,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":415,"record_offset":10792,"parent_record_offset":10752,"ancestor_record_offset":7252,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":415,"record_offset":10712,"parent_record_offset":10656,"ancestor_record_offset":7252,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/command_queue.rs
//     evidence={"module":415,"record_offset":11456,"parent_record_offset":10792,"ancestor_record_offset":7252,"inlinee":11374,"source_line":96,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/command_queue.rs
//     evidence={"module":415,"record_offset":10848,"parent_record_offset":10792,"ancestor_record_offset":7252,"inlinee":11372,"source_line":172,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$65::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":415,"record_offset":10352,"parent_record_offset":10092,"ancestor_record_offset":7252,"inlinee":16295,"source_line":2016,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":8584,"parent_record_offset":8556,"ancestor_record_offset":7252,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_ilog2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10288,"parent_record_offset":10256,"ancestor_record_offset":7252,"inlinee":26404,"source_line":1751,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::ilog2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10256,"parent_record_offset":10092,"ancestor_record_offset":7252,"inlinee":26403,"source_line":1639,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10192,"parent_record_offset":10092,"ancestor_record_offset":7252,"inlinee":26401,"source_line":3806,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10216,"parent_record_offset":10192,"ancestor_record_offset":7252,"inlinee":26402,"source_line":3775,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10164,"parent_record_offset":10092,"ancestor_record_offset":7252,"inlinee":26400,"source_line":2300,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10136,"parent_record_offset":10092,"ancestor_record_offset":7252,"inlinee":13037,"source_line":2344,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::ilog2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":415,"record_offset":10316,"parent_record_offset":10288,"ancestor_record_offset":7252,"inlinee":14770,"source_line":1790,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":7252,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":7468,"parent_record_offset":7420,"ancestor_record_offset":7252,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":415,"record_offset":11228,"parent_record_offset":10848,"ancestor_record_offset":7252,"inlinee":26405,"source_line":1473,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":415,"record_offset":11252,"parent_record_offset":11228,"ancestor_record_offset":7252,"inlinee":26406,"source_line":1998,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::device_address_impl
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":415,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":7252,"inlinee":26372,"source_line":133,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wooden_arch_frame_systems::wooden_arch_frame_mesh_atlas_part_idx
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x671F
// 首条位置: {"module":415,"inlinee":26399}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$65::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":415,"record_offset":10352,"parent_record_offset":10092,"ancestor_record_offset":10092,"inlinee":16295,"source_line":2016,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::ilog2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10256,"parent_record_offset":10092,"ancestor_record_offset":10092,"inlinee":26403,"source_line":1639,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10192,"parent_record_offset":10092,"ancestor_record_offset":10092,"inlinee":26401,"source_line":3806,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10164,"parent_record_offset":10092,"ancestor_record_offset":10092,"inlinee":26400,"source_line":2300,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10136,"parent_record_offset":10092,"ancestor_record_offset":10092,"inlinee":13037,"source_line":2344,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_ilog2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10288,"parent_record_offset":10256,"ancestor_record_offset":10092,"inlinee":26404,"source_line":1751,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::one_less_than_next_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":415,"record_offset":10216,"parent_record_offset":10192,"ancestor_record_offset":10092,"inlinee":26402,"source_line":3775,"archive":"_pdb/C/modules/00415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::ilog2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":415,"record_offset":10316,"parent_record_offset":10288,"ancestor_record_offset":10092,"inlinee":14770,"source_line":1790,"archive":"_pdb/C/modules/00415.jsonl.gz"}
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
// 完整结构及原始类型记录: wooden_arch_frame_systems.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow>", "constructor": "tuple$", "arguments": ["system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>", "country_core::resources::walls::wooden_arch_frame::WoodenArchFrame", "country_core::components::drawable::DontCastShadow"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::BundleInfo::write_components<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::info::BundleInfo::write_components", "arguments": ["tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$19::component_ids<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::bundle::impls::impl$19::component_ids", "arguments": ["system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner", "country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>", "country_core::resources::walls::wooden_arch_frame::WoodenArchFrame", "country_core::components::drawable::DontCastShadow", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner>", "constructor": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr", "arguments": ["system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::resources::walls::wooden_arch_frame::WoodenArchFrame", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner,country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::WoodenArchFrameInstanceData>,country_core::resources::walls::wooden_arch_frame::WoodenArchFrame,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wooden_arch_frame_systems.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wooden_arch_frame_systems.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wooden_arch_frame_systems::despawn_wooden_arch_frames
// 信息 ID: df932daf14951d39373f98c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wooden_arch_frame_systems::maintain_wooden_arch_frames; module=415, inline_record=3300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wooden_arch_frame_systems::despawn_wooden_arch_frames::closure$0
// 信息 ID: 8a343608087bbd6029a29f8d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::wooden_arch_frame_systems::despawn_wooden_arch_frames", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wooden_arch_frame_systems::maintain_wooden_arch_frames; module=415, inline_record=3752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wooden_arch_frame_systems::impl$0::drop
// 信息 ID: 2ae2a6c49317ce95d17a0dc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<system_wall_constructor::wooden_arch_frame_systems::WoodenArchFrameBufferOwner> @ 0x140f209 -> 0x140a3e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wooden_arch_frame_systems::maintain_deferred_wooden_arch_frame_buffer_disposal
// 信息 ID: 6835b34cc142e71a70a9e932
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wooden_arch_frame_systems::maintain_wooden_arch_frames; module=415, inline_record=492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wooden_arch_frame_systems::maintain_deferred_wooden_arch_frame_buffer_disposal::closure$0
// 信息 ID: 91551a7b07fa6d477e923149
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::wooden_arch_frame_systems::maintain_deferred_wooden_arch_frame_buffer_disposal", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wooden_arch_frame_systems::maintain_wooden_arch_frames; module=415, inline_record=1376; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wooden_arch_frame_systems::maintain_wooden_arch_frames; module=415, inline_record=1544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wooden_arch_frame_systems::maintain_wooden_arch_frames
// 信息 ID: bb0420af59419a677d8ed0f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f2e99b8ef1bc77d9ae9b5a7b: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>)> > @ 0x288156 -> 0x819120 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>)> @ 0x60b187 -> 0x819120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wooden_arch_frame_systems::spawn_wooden_arch_frame
// 信息 ID: 4653fcc2bacd626e19bb42da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wooden_arch_frame_systems::maintain_wooden_arch_frames; module=415, inline_record=7252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wooden_arch_frame_systems::wooden_arch_frame_mesh_atlas_part_idx
// 信息 ID: ae64a4761be85d96797c4606
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wooden_arch_frame_systems::maintain_wooden_arch_frames; module=415, inline_record=10092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
