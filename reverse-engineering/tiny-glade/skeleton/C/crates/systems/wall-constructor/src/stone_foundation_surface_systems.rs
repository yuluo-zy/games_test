// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 857acf34be7df5ea76e8deb557896aa15339973eb6715d81f6e36a7b70d049a1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stone_foundation_surface_systems.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 423, 1455, 1461, 1482, 1500, 3125, 3159, 4370
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
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::impl$0::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1461,"record_offset":4016,"rva":21014240,"va":5389723360}
// 编译实例 RVAs: 0x140a6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26892400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389723360,"raw_hex":"e870589a01","size":5,"soft_fail":false,"target_va":5416616112,"va":5389723707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20483464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389723360,"raw_hex":"e87872c7fe","size":5,"soft_fail":false,"target_va":5369240448,"va":5389723907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20458978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389723360,"raw_hex":"e81ed2c7fe","size":5,"soft_fail":false,"target_va":5369265120,"va":5389724093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140cd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389723360,"raw_hex":"e826240000","size":5,"soft_fail":false,"target_va":5389733232,"va":5389723973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26627436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389723360,"raw_hex":"e86c4d9601","size":5,"soft_fail":false,"target_va":5416351056,"va":5389723615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<alloc::vec::drain::Drain<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389723360,"raw_hex":"e8e9470000","size":5,"soft_fail":false,"target_va":5389742048,"va":5389723634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26897830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389723360,"raw_hex":"e8a66d9a01","size":5,"soft_fail":false,"target_va":5416621632,"va":5389723797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26895968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389723360,"raw_hex":"e860669a01","size":5,"soft_fail":false,"target_va":5416619696,"va":5389723723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t26892471","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5389723360,"raw_hex":"e9b7589a01","size":5,"soft_fail":false,"target_va":5416616304,"va":5389723828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":4732,"parent_record_offset":4016,"ancestor_record_offset":4016,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":4200,"parent_record_offset":4016,"ancestor_record_offset":4016,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":5220,"parent_record_offset":4016,"ancestor_record_offset":4016,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":1461,"record_offset":5052,"parent_record_offset":4016,"ancestor_record_offset":4016,"inlinee":5049,"source_line":155,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1461,"record_offset":5024,"parent_record_offset":4016,"ancestor_record_offset":4016,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":5480,"parent_record_offset":4016,"ancestor_record_offset":4016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1461,"record_offset":4688,"parent_record_offset":4016,"ancestor_record_offset":4016,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1461,"record_offset":4228,"parent_record_offset":4016,"ancestor_record_offset":4016,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1461,"record_offset":4792,"parent_record_offset":4768,"ancestor_record_offset":4016,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1461,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":4016,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1461,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":4016,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":5304,"parent_record_offset":5256,"ancestor_record_offset":4016,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":4768,"parent_record_offset":4732,"ancestor_record_offset":4016,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":4880,"parent_record_offset":4732,"ancestor_record_offset":4016,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":5256,"parent_record_offset":5220,"ancestor_record_offset":4016,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":4908,"parent_record_offset":4732,"ancestor_record_offset":4016,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":1461,"record_offset":5156,"parent_record_offset":5084,"ancestor_record_offset":4016,"inlinee":5060,"source_line":156,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":5444,"parent_record_offset":5256,"ancestor_record_offset":4016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1461,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":4016,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1461,"record_offset":5124,"parent_record_offset":5084,"ancestor_record_offset":4016,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":4016,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":5416,"parent_record_offset":5256,"ancestor_record_offset":4016,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1461,"record_offset":4936,"parent_record_offset":4732,"ancestor_record_offset":4016,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1461,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":4016,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":5084,"parent_record_offset":5052,"ancestor_record_offset":4016,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":4608,"parent_record_offset":4580,"ancestor_record_offset":4016,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":5732,"parent_record_offset":5544,"ancestor_record_offset":4016,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":4300,"parent_record_offset":4268,"ancestor_record_offset":4016,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":5824,"parent_record_offset":5792,"ancestor_record_offset":4016,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":4512,"parent_record_offset":4480,"ancestor_record_offset":4016,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":4324,"parent_record_offset":4300,"ancestor_record_offset":4016,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":4016,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":4016,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":4016,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":1461,"record_offset":4480,"parent_record_offset":4452,"ancestor_record_offset":4016,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":1461,"record_offset":4452,"parent_record_offset":4424,"ancestor_record_offset":4016,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1461,"record_offset":5544,"parent_record_offset":5508,"ancestor_record_offset":4016,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1461,"record_offset":4580,"parent_record_offset":4388,"ancestor_record_offset":4016,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1461,"record_offset":4388,"parent_record_offset":4360,"ancestor_record_offset":4016,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1461,"record_offset":4360,"parent_record_offset":4228,"ancestor_record_offset":4016,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1461,"record_offset":5508,"parent_record_offset":5480,"ancestor_record_offset":4016,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1461,"record_offset":4268,"parent_record_offset":4228,"ancestor_record_offset":4016,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1461,"record_offset":5792,"parent_record_offset":5508,"ancestor_record_offset":4016,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":1461,"record_offset":4424,"parent_record_offset":4388,"ancestor_record_offset":4016,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/01461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":423,"record_offset":3896,"rva":8629424,"va":5377338544}
// 编译实例 RVAs: 0x83acb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t39265929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e889265702","size":5,"soft_fail":false,"target_va":5416616112,"va":5377350178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t39266013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8dd265702","size":5,"soft_fail":false,"target_va":5416616304,"va":5377350286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-1663812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8bc9ce6ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377342495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=35
//     evidence={"assembly":"\tcallq\t-1663505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8ef9de6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377342204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_realloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1662201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e807a3e6ff","size":5,"soft_fail":false,"target_va":5375678720,"va":5377340916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=18
//     target_aliases=1699 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8620289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8ff767cff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377342476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8113105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e82f3484ff","size":5,"soft_fail":false,"target_va":5369240448,"va":5377353548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8115125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e84b2c84ff","size":5,"soft_fail":false,"target_va":5369240528,"va":5377355648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8091605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e82b8884ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5377356720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140306210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5461027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8ddabacff","size":5,"soft_fail":false,"target_va":5371879952,"va":5377340974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3860568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8a817c5ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377339603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3765389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8738bc6ff","size":5,"soft_fail":false,"target_va":5373575632,"va":5377341016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f7b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t762903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e817a40b00","size":5,"soft_fail":false,"target_va":5378112288,"va":5377349380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t780708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8a4e90b00","size":5,"soft_fail":false,"target_va":5378120480,"va":5377339767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t777571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e863dd0b00","size":5,"soft_fail":false,"target_va":5378126944,"va":5377349368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2375871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8bf402400","size":5,"soft_fail":false,"target_va":5379716240,"va":5377340364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5981787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e85b465b00","size":5,"soft_fail":false,"target_va":5383325024,"va":5377343232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f98a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12307970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e802cebb00","size":5,"soft_fail":false,"target_va":5389654176,"va":5377346201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140b270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12370889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8c9c3bc00","size":5,"soft_fail":false,"target_va":5389726320,"va":5377355426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140cd70
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12392749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e82d19bd00","size":5,"soft_fail":false,"target_va":5389733232,"va":5377340478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140eb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12384789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e815fabc00","size":5,"soft_fail":false,"target_va":5389740944,"va":5377356150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141415ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12419586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e80282bd00","size":5,"soft_fail":false,"target_va":5389769424,"va":5377349833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141423180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12475231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e85f5bbe00","size":5,"soft_fail":false,"target_va":5389824384,"va":5377349148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142b2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12514627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e843f5be00","size":5,"soft_fail":false,"target_va":5389857472,"va":5377342840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146ce00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12784347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8db12c300","size":5,"soft_fail":false,"target_va":5390126592,"va":5377342240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39003316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8b4245302","size":5,"soft_fail":false,"target_va":5416353040,"va":5377349719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39703547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8fbd35d02","size":5,"soft_fail":false,"target_va":5417057072,"va":5377353520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39715973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e885045e02","size":5,"soft_fail":false,"target_va":5417057664,"va":5377341686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39717375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8ff095e02","size":5,"soft_fail":false,"target_va":5417057984,"va":5377340604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39270419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e813385702","size":5,"soft_fail":false,"target_va":5416620769,"va":5377350345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=17
//     evidence={"assembly":"\tcallq\t39024620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8ec775302","size":5,"soft_fail":false,"target_va":5416366080,"va":5377341455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t39270330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8ba375702","size":5,"soft_fail":false,"target_va":5416620794,"va":5377350459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::reserve_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6876763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e85bee6800","size":5,"soft_fail":false,"target_va":5384226192,"va":5377349424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39270970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e83a3a5702","size":5,"soft_fail":false,"target_va":5416621536,"va":5377350561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39271118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8ce3a5702","size":5,"soft_fail":false,"target_va":5416621600,"va":5377350477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39271672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8f83c5702","size":5,"soft_fail":false,"target_va":5416622208,"va":5377350531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39271115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8cb3a5702","size":5,"soft_fail":false,"target_va":5416621762,"va":5377350642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39271614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8be3c5702","size":5,"soft_fail":false,"target_va":5416622240,"va":5377350621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39271359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8bf3b5702","size":5,"soft_fail":false,"target_va":5416621632,"va":5377350268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12801266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8f254c300","size":5,"soft_fail":false,"target_va":5390147408,"va":5377346137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3856927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e81fda3a00","size":5,"soft_fail":false,"target_va":5381197456,"va":5377340524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wooden_stone_foundation_surface_top_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3893528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e818693b00","size":5,"soft_fail":false,"target_va":5381234096,"va":5377340563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wooden_stone_foundation_surface_top_y_at
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3891284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e854603b00","size":5,"soft_fail":false,"target_va":5381234176,"va":5377342887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3562518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8165c3600","size":5,"soft_fail":false,"target_va":5380902448,"va":5377339925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39269503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e87f345702","size":5,"soft_fail":false,"target_va":5416619696,"va":5377350188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_wooden_walls::construct_wall_boards::arch_openings_to_aabb2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12938726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8e66dc500","size":5,"soft_fail":false,"target_va":5390279392,"va":5377340661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::plaster_systems::split_wall_into_sides
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-197404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8e4fcfcff","size":5,"soft_fail":false,"target_va":5377143712,"va":5377341111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::stone_foundation_surface_systems::append_stone_foundation_surface_face
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e840250000","size":5,"soft_fail":false,"target_va":5377356832,"va":5377347291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::stone_foundation_surface_systems::despawn_stone_foundation_surface
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8e1faffff","size":5,"soft_fail":false,"target_va":5377337824,"va":5377339130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5961069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e86df55a00","size":5,"soft_fail":false,"target_va":5383304736,"va":5377343662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5970239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e83f195b00","size":5,"soft_fail":false,"target_va":5383313440,"va":5377343196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5937380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377338544,"raw_hex":"e8e4985a00","size":5,"soft_fail":false,"target_va":5383282224,"va":5377344839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":423,"record_offset":6612,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":423,"record_offset":5916,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::has_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":423,"record_offset":8836,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":26244,"source_line":143,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":7896,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh_and_entities
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":8924,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":26584,"source_line":123,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::stone_foundation_surface_systems::maintain_deferred_stone_foundation_surface_buffer_disposal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":4096,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":26581,"source_line":98,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":3896,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":3896,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13728,"parent_record_offset":13700,"ancestor_record_offset":3896,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13756,"parent_record_offset":13728,"ancestor_record_offset":3896,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13480,"parent_record_offset":13456,"ancestor_record_offset":3896,"inlinee":6570,"source_line":353,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13504,"parent_record_offset":13480,"ancestor_record_offset":3896,"inlinee":6571,"source_line":264,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":3896,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13784,"parent_record_offset":13756,"ancestor_record_offset":3896,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17668,"parent_record_offset":17520,"ancestor_record_offset":3896,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13700,"parent_record_offset":13672,"ancestor_record_offset":3896,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13456,"parent_record_offset":12812,"ancestor_record_offset":3896,"inlinee":6569,"source_line":471,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13528,"parent_record_offset":13504,"ancestor_record_offset":3896,"inlinee":6572,"source_line":134,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":423,"record_offset":36464,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":423,"record_offset":36308,"parent_record_offset":36280,"ancestor_record_offset":3896,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":423,"record_offset":36280,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":423,"record_offset":38024,"parent_record_offset":38000,"ancestor_record_offset":3896,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":3896,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":37072,"parent_record_offset":37044,"ancestor_record_offset":3896,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17460,"parent_record_offset":17428,"ancestor_record_offset":3896,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":37156,"parent_record_offset":37132,"ancestor_record_offset":3896,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":16568,"parent_record_offset":16540,"ancestor_record_offset":3896,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":13672,"parent_record_offset":13644,"ancestor_record_offset":3896,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":37132,"parent_record_offset":37100,"ancestor_record_offset":3896,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":3896,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":9856,"parent_record_offset":9832,"ancestor_record_offset":3896,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":37100,"parent_record_offset":37072,"ancestor_record_offset":3896,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17520,"parent_record_offset":17488,"ancestor_record_offset":3896,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17488,"parent_record_offset":17460,"ancestor_record_offset":3896,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":3896,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17556,"parent_record_offset":17520,"ancestor_record_offset":3896,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":36488,"parent_record_offset":36464,"ancestor_record_offset":3896,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":16324,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":16356,"parent_record_offset":16324,"ancestor_record_offset":3896,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":3896,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12188,"parent_record_offset":12148,"ancestor_record_offset":3896,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":9808,"parent_record_offset":9784,"ancestor_record_offset":3896,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":3896,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12120,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27472,"parent_record_offset":21232,"ancestor_record_offset":3896,"inlinee":11483,"source_line":3637,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":16956,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":36512,"parent_record_offset":36488,"ancestor_record_offset":3896,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":4932,"parent_record_offset":4848,"ancestor_record_offset":3896,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":9104,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14428,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14452,"parent_record_offset":14428,"ancestor_record_offset":3896,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":37044,"parent_record_offset":37012,"ancestor_record_offset":3896,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":4812,"parent_record_offset":4096,"ancestor_record_offset":3896,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":4848,"parent_record_offset":4812,"ancestor_record_offset":3896,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":5268,"parent_record_offset":4848,"ancestor_record_offset":3896,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17396,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17428,"parent_record_offset":17396,"ancestor_record_offset":3896,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":19964,"parent_record_offset":19904,"ancestor_record_offset":3896,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12724,"parent_record_offset":12680,"ancestor_record_offset":3896,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12596,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17800,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":4960,"parent_record_offset":4848,"ancestor_record_offset":3896,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":5320,"parent_record_offset":5296,"ancestor_record_offset":3896,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$6::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27604,"parent_record_offset":27504,"ancestor_record_offset":3896,"inlinee":10784,"source_line":3638,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":6232,"parent_record_offset":6004,"ancestor_record_offset":3896,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":12812,"parent_record_offset":12768,"ancestor_record_offset":3896,"inlinee":10798,"source_line":249,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":12768,"parent_record_offset":12724,"ancestor_record_offset":3896,"inlinee":8793,"source_line":232,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$3::collect_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":12876,"parent_record_offset":12812,"ancestor_record_offset":3896,"inlinee":10803,"source_line":389,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::needs_realloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":13428,"parent_record_offset":12812,"ancestor_record_offset":3896,"inlinee":13390,"source_line":192,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_drop::impl$2::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_drop.rs
//     evidence={"module":423,"record_offset":13592,"parent_record_offset":13568,"ancestor_record_offset":3896,"inlinee":13389,"source_line":41,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":27892,"parent_record_offset":27868,"ancestor_record_offset":3896,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":27940,"parent_record_offset":27916,"ancestor_record_offset":3896,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":13220,"parent_record_offset":13184,"ancestor_record_offset":3896,"inlinee":10805,"source_line":362,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":25292,"parent_record_offset":21232,"ancestor_record_offset":3896,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":423,"record_offset":9540,"parent_record_offset":9256,"ancestor_record_offset":3896,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":423,"record_offset":37620,"parent_record_offset":37596,"ancestor_record_offset":3896,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":423,"record_offset":37596,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9304,"parent_record_offset":9256,"ancestor_record_offset":3896,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":3896,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9444,"parent_record_offset":9412,"ancestor_record_offset":3896,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9468,"parent_record_offset":9444,"ancestor_record_offset":3896,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":423,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":3896,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":423,"record_offset":6640,"parent_record_offset":6612,"ancestor_record_offset":3896,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":423,"record_offset":5980,"parent_record_offset":5940,"ancestor_record_offset":3896,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":10516,"parent_record_offset":10492,"ancestor_record_offset":3896,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":9960,"parent_record_offset":9256,"ancestor_record_offset":3896,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":3896,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":10492,"parent_record_offset":9256,"ancestor_record_offset":3896,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":10540,"parent_record_offset":10516,"ancestor_record_offset":3896,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":423,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":3896,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":423,"record_offset":10084,"parent_record_offset":10060,"ancestor_record_offset":3896,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":423,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":3896,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":423,"record_offset":10264,"parent_record_offset":10240,"ancestor_record_offset":3896,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10036,"parent_record_offset":10008,"ancestor_record_offset":3896,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10008,"parent_record_offset":9984,"ancestor_record_offset":3896,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10564,"parent_record_offset":10540,"ancestor_record_offset":3896,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":9728,"parent_record_offset":9256,"ancestor_record_offset":3896,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":423,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":3896,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":3896,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":423,"record_offset":36984,"parent_record_offset":36956,"ancestor_record_offset":3896,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":423,"record_offset":37012,"parent_record_offset":36984,"ancestor_record_offset":3896,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":423,"record_offset":36924,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":423,"record_offset":36956,"parent_record_offset":36924,"ancestor_record_offset":3896,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":423,"record_offset":9224,"parent_record_offset":9196,"ancestor_record_offset":3896,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":423,"record_offset":9256,"parent_record_offset":9224,"ancestor_record_offset":3896,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":423,"record_offset":5940,"parent_record_offset":5916,"ancestor_record_offset":3896,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":3896,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":423,"record_offset":15824,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":423,"record_offset":15852,"parent_record_offset":15824,"ancestor_record_offset":3896,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":423,"record_offset":15924,"parent_record_offset":15880,"ancestor_record_offset":3896,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":423,"record_offset":9576,"parent_record_offset":9540,"ancestor_record_offset":3896,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19272,"parent_record_offset":19160,"ancestor_record_offset":3896,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19808,"parent_record_offset":19784,"ancestor_record_offset":3896,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":27628,"parent_record_offset":27604,"ancestor_record_offset":3896,"inlinee":10786,"source_line":2197,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":27652,"parent_record_offset":27628,"ancestor_record_offset":3896,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19832,"parent_record_offset":19808,"ancestor_record_offset":3896,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":13016,"parent_record_offset":12972,"ancestor_record_offset":3896,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":3896,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":3896,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":423,"record_offset":36896,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":423,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":3896,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":423,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":3896,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":3896,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":12568,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":26356,"parent_record_offset":21232,"ancestor_record_offset":3896,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":19236,"parent_record_offset":19212,"ancestor_record_offset":3896,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":15388,"parent_record_offset":15364,"ancestor_record_offset":3896,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":12536,"parent_record_offset":12512,"ancestor_record_offset":3896,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":20416,"parent_record_offset":20392,"ancestor_record_offset":3896,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":423,"record_offset":5016,"parent_record_offset":4848,"ancestor_record_offset":3896,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":423,"record_offset":8664,"parent_record_offset":8028,"ancestor_record_offset":3896,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":423,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":3896,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":423,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":3896,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":423,"record_offset":6672,"parent_record_offset":6640,"ancestor_record_offset":3896,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":423,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":3896,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":423,"record_offset":28324,"parent_record_offset":21232,"ancestor_record_offset":3896,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::filter_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18948,"parent_record_offset":18880,"ancestor_record_offset":3896,"inlinee":11536,"source_line":79,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18856,"parent_record_offset":18832,"ancestor_record_offset":3896,"inlinee":11531,"source_line":161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18444,"parent_record_offset":18416,"ancestor_record_offset":3896,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":13112,"parent_record_offset":12876,"ancestor_record_offset":3896,"inlinee":10804,"source_line":132,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":18832,"parent_record_offset":18380,"ancestor_record_offset":3896,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":14924,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":19112,"parent_record_offset":18948,"ancestor_record_offset":3896,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$19::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":13184,"parent_record_offset":13148,"ancestor_record_offset":3896,"inlinee":6629,"source_line":629,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":13148,"parent_record_offset":13112,"ancestor_record_offset":3896,"inlinee":6627,"source_line":606,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":13048,"parent_record_offset":12972,"ancestor_record_offset":3896,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":12972,"parent_record_offset":12932,"ancestor_record_offset":3896,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":12932,"parent_record_offset":12876,"ancestor_record_offset":3896,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":423,"record_offset":25076,"parent_record_offset":25052,"ancestor_record_offset":3896,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":12680,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":18348,"parent_record_offset":18316,"ancestor_record_offset":3896,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":19136,"parent_record_offset":19112,"ancestor_record_offset":3896,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":18380,"parent_record_offset":18348,"ancestor_record_offset":3896,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":3896,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":423,"record_offset":34192,"parent_record_offset":34164,"ancestor_record_offset":3896,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":423,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":3896,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":423,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":3896,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":423,"record_offset":15948,"parent_record_offset":15924,"ancestor_record_offset":3896,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":17604,"parent_record_offset":17580,"ancestor_record_offset":3896,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":6188,"parent_record_offset":6160,"ancestor_record_offset":3896,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":6048,"parent_record_offset":6004,"ancestor_record_offset":3896,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":13072,"parent_record_offset":13048,"ancestor_record_offset":3896,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":16600,"parent_record_offset":16568,"ancestor_record_offset":3896,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":423,"record_offset":8396,"parent_record_offset":8368,"ancestor_record_offset":3896,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":423,"record_offset":8368,"parent_record_offset":8340,"ancestor_record_offset":3896,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":423,"record_offset":18700,"parent_record_offset":18620,"ancestor_record_offset":3896,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":18668,"parent_record_offset":18644,"ancestor_record_offset":3896,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":19636,"parent_record_offset":19612,"ancestor_record_offset":3896,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":423,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":3896,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":18724,"parent_record_offset":18700,"ancestor_record_offset":3896,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":19212,"parent_record_offset":19188,"ancestor_record_offset":3896,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":10176,"parent_record_offset":10112,"ancestor_record_offset":3896,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":18596,"parent_record_offset":18472,"ancestor_record_offset":3896,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":15064,"parent_record_offset":15040,"ancestor_record_offset":3896,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":423,"record_offset":16036,"parent_record_offset":15880,"ancestor_record_offset":3896,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":423,"record_offset":15880,"parent_record_offset":15852,"ancestor_record_offset":3896,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":423,"record_offset":19784,"parent_record_offset":19760,"ancestor_record_offset":3896,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":423,"record_offset":19760,"parent_record_offset":19728,"ancestor_record_offset":3896,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":9160,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":3896,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":10200,"parent_record_offset":10176,"ancestor_record_offset":3896,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":18280,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":9412,"parent_record_offset":9328,"ancestor_record_offset":3896,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":28268,"parent_record_offset":21232,"ancestor_record_offset":3896,"inlinee":14503,"source_line":1754,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":28292,"parent_record_offset":28268,"ancestor_record_offset":3896,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":12452,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":12476,"parent_record_offset":12452,"ancestor_record_offset":3896,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":12392,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":10240,"parent_record_offset":10060,"ancestor_record_offset":3896,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":9512,"parent_record_offset":9256,"ancestor_record_offset":3896,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":6752,"parent_record_offset":6672,"ancestor_record_offset":3896,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":18092,"parent_record_offset":18056,"ancestor_record_offset":3896,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":11660,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":5356,"parent_record_offset":5320,"ancestor_record_offset":3896,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":15972,"parent_record_offset":15948,"ancestor_record_offset":3896,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":5412,"parent_record_offset":5320,"ancestor_record_offset":3896,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":5240,"parent_record_offset":4848,"ancestor_record_offset":3896,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":5072,"parent_record_offset":4848,"ancestor_record_offset":3896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":5100,"parent_record_offset":4848,"ancestor_record_offset":3896,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":3896,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":4776,"parent_record_offset":4696,"ancestor_record_offset":3896,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":13256,"parent_record_offset":13220,"ancestor_record_offset":3896,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":4744,"parent_record_offset":4696,"ancestor_record_offset":3896,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":13300,"parent_record_offset":13256,"ancestor_record_offset":3896,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":13360,"parent_record_offset":12876,"ancestor_record_offset":3896,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":423,"record_offset":4652,"parent_record_offset":4096,"ancestor_record_offset":3896,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":6104,"parent_record_offset":6004,"ancestor_record_offset":3896,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":6456,"parent_record_offset":6004,"ancestor_record_offset":3896,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":28448,"parent_record_offset":28420,"ancestor_record_offset":3896,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":19384,"parent_record_offset":19340,"ancestor_record_offset":3896,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":9380,"parent_record_offset":9356,"ancestor_record_offset":3896,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":10320,"parent_record_offset":10288,"ancestor_record_offset":3896,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":19416,"parent_record_offset":19384,"ancestor_record_offset":3896,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":3896,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":6132,"parent_record_offset":6104,"ancestor_record_offset":3896,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":3896,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":3896,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":28492,"parent_record_offset":21232,"ancestor_record_offset":3896,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":18472,"parent_record_offset":18444,"ancestor_record_offset":3896,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":18880,"parent_record_offset":18856,"ancestor_record_offset":3896,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":6828,"parent_record_offset":6776,"ancestor_record_offset":3896,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":4696,"parent_record_offset":4096,"ancestor_record_offset":3896,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":23060,"parent_record_offset":21232,"ancestor_record_offset":3896,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":28360,"parent_record_offset":28324,"ancestor_record_offset":3896,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":423,"record_offset":16388,"parent_record_offset":16356,"ancestor_record_offset":3896,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":4572,"parent_record_offset":4544,"ancestor_record_offset":3896,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":5752,"parent_record_offset":5560,"ancestor_record_offset":3896,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":4264,"parent_record_offset":4232,"ancestor_record_offset":3896,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":5844,"parent_record_offset":5812,"ancestor_record_offset":3896,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":4476,"parent_record_offset":4444,"ancestor_record_offset":3896,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":4288,"parent_record_offset":4264,"ancestor_record_offset":3896,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":4500,"parent_record_offset":4476,"ancestor_record_offset":3896,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":5776,"parent_record_offset":5752,"ancestor_record_offset":3896,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":5868,"parent_record_offset":5844,"ancestor_record_offset":3896,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":423,"record_offset":8620,"parent_record_offset":8596,"ancestor_record_offset":3896,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":423,"record_offset":8888,"parent_record_offset":8864,"ancestor_record_offset":3896,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::supports_wooden_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":423,"record_offset":8864,"parent_record_offset":8836,"ancestor_record_offset":3896,"inlinee":26245,"source_line":137,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":423,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":3896,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":423,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":3896,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":423,"record_offset":9600,"parent_record_offset":9576,"ancestor_record_offset":3896,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20656,"parent_record_offset":20620,"ancestor_record_offset":3896,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20620,"parent_record_offset":20580,"ancestor_record_offset":3896,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":3896,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20556,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20528,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20360,"parent_record_offset":20328,"ancestor_record_offset":3896,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20328,"parent_record_offset":20296,"ancestor_record_offset":3896,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20296,"parent_record_offset":20272,"ancestor_record_offset":3896,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20272,"parent_record_offset":20248,"ancestor_record_offset":3896,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":19612,"parent_record_offset":19340,"ancestor_record_offset":3896,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":19668,"parent_record_offset":19340,"ancestor_record_offset":3896,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":19556,"parent_record_offset":19340,"ancestor_record_offset":3896,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":423,"record_offset":8308,"parent_record_offset":8280,"ancestor_record_offset":3896,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":423,"record_offset":8340,"parent_record_offset":8308,"ancestor_record_offset":3896,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":423,"record_offset":8280,"parent_record_offset":8028,"ancestor_record_offset":3896,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":423,"record_offset":8140,"parent_record_offset":8028,"ancestor_record_offset":3896,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":423,"record_offset":8692,"parent_record_offset":8028,"ancestor_record_offset":3896,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":423,"record_offset":8224,"parent_record_offset":8028,"ancestor_record_offset":3896,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":423,"record_offset":8084,"parent_record_offset":8028,"ancestor_record_offset":3896,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":423,"record_offset":8572,"parent_record_offset":8548,"ancestor_record_offset":3896,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":423,"record_offset":7920,"parent_record_offset":7896,"ancestor_record_offset":3896,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":423,"record_offset":8548,"parent_record_offset":8440,"ancestor_record_offset":3896,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":423,"record_offset":7952,"parent_record_offset":7920,"ancestor_record_offset":3896,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":8488,"parent_record_offset":8464,"ancestor_record_offset":3896,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":8784,"parent_record_offset":8028,"ancestor_record_offset":3896,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":3896,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":3896,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":7980,"parent_record_offset":7920,"ancestor_record_offset":3896,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":3896,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":8112,"parent_record_offset":8028,"ancestor_record_offset":3896,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":8440,"parent_record_offset":8028,"ancestor_record_offset":3896,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":423,"record_offset":10596,"parent_record_offset":10564,"ancestor_record_offset":3896,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":423,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":3896,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14292,"parent_record_offset":14256,"ancestor_record_offset":3896,"inlinee":26594,"source_line":1716,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14140,"parent_record_offset":14108,"ancestor_record_offset":3896,"inlinee":26590,"source_line":952,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14224,"parent_record_offset":14108,"ancestor_record_offset":3896,"inlinee":26591,"source_line":919,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14188,"parent_record_offset":14164,"ancestor_record_offset":3896,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14164,"parent_record_offset":14140,"ancestor_record_offset":3896,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":34164,"parent_record_offset":34140,"ancestor_record_offset":3896,"inlinee":4820,"source_line":2118,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":34052,"parent_record_offset":34028,"ancestor_record_offset":3896,"inlinee":26609,"source_line":2228,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14256,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14108,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":26589,"source_line":2290,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":10684,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":37652,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":15364,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":12512,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":20392,"parent_record_offset":20360,"ancestor_record_offset":3896,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":423,"record_offset":4444,"parent_record_offset":4416,"ancestor_record_offset":3896,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":423,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":3896,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":423,"record_offset":5560,"parent_record_offset":5516,"ancestor_record_offset":3896,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":423,"record_offset":4544,"parent_record_offset":4352,"ancestor_record_offset":3896,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":423,"record_offset":4352,"parent_record_offset":4324,"ancestor_record_offset":3896,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":423,"record_offset":4192,"parent_record_offset":4096,"ancestor_record_offset":3896,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":423,"record_offset":4324,"parent_record_offset":4192,"ancestor_record_offset":3896,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":423,"record_offset":5516,"parent_record_offset":5484,"ancestor_record_offset":3896,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":423,"record_offset":4232,"parent_record_offset":4192,"ancestor_record_offset":3896,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":423,"record_offset":5812,"parent_record_offset":5516,"ancestor_record_offset":3896,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":423,"record_offset":4388,"parent_record_offset":4352,"ancestor_record_offset":3896,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":21232,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":26604,"source_line":427,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":28520,"parent_record_offset":28492,"ancestor_record_offset":3896,"inlinee":26607,"source_line":554,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":11688,"parent_record_offset":8924,"ancestor_record_offset":3896,"inlinee":26587,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":18316,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":26599,"source_line":271,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":3896,"inlinee":26601,"source_line":274,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":15088,"parent_record_offset":15064,"ancestor_record_offset":3896,"inlinee":26596,"source_line":294,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":17200,"parent_record_offset":16956,"ancestor_record_offset":3896,"inlinee":26597,"source_line":318,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh_and_entities::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":9196,"parent_record_offset":9160,"ancestor_record_offset":3896,"inlinee":26586,"source_line":138,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::maintain_deferred_stone_foundation_surface_buffer_disposal::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":3896,"inlinee":26583,"source_line":105,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::stone_foundation_surface_vertex_from_wallspace
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":23452,"parent_record_offset":21232,"ancestor_record_offset":3896,"inlinee":26605,"source_line":575,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":14036,"parent_record_offset":13864,"ancestor_record_offset":3896,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":13864,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":26086,"source_line":1147,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":17976,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":26598,"source_line":1171,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19876,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":26602,"source_line":816,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19904,"parent_record_offset":19876,"ancestor_record_offset":3896,"inlinee":26603,"source_line":812,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":12364,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19456,"parent_record_offset":19340,"ancestor_record_offset":3896,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18248,"parent_record_offset":17976,"ancestor_record_offset":3896,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18220,"parent_record_offset":17976,"ancestor_record_offset":3896,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18024,"parent_record_offset":17976,"ancestor_record_offset":3896,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18056,"parent_record_offset":18024,"ancestor_record_offset":3896,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18156,"parent_record_offset":18128,"ancestor_record_offset":3896,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18128,"parent_record_offset":17976,"ancestor_record_offset":3896,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":19728,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":15112,"parent_record_offset":15088,"ancestor_record_offset":3896,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19340,"parent_record_offset":11688,"ancestor_record_offset":3896,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":423,"record_offset":20248,"parent_record_offset":19876,"ancestor_record_offset":3896,"inlinee":26418,"source_line":99,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::maintain_deferred_stone_foundation_surface_buffer_disposal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67D5
// 首条位置: {"module":423,"inlinee":26581}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":4812,"parent_record_offset":4096,"ancestor_record_offset":4096,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":5484,"parent_record_offset":4096,"ancestor_record_offset":4096,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":423,"record_offset":4652,"parent_record_offset":4096,"ancestor_record_offset":4096,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":4696,"parent_record_offset":4096,"ancestor_record_offset":4096,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":423,"record_offset":4192,"parent_record_offset":4096,"ancestor_record_offset":4096,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":4932,"parent_record_offset":4848,"ancestor_record_offset":4096,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":4848,"parent_record_offset":4812,"ancestor_record_offset":4096,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":5268,"parent_record_offset":4848,"ancestor_record_offset":4096,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":4960,"parent_record_offset":4848,"ancestor_record_offset":4096,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":5320,"parent_record_offset":5296,"ancestor_record_offset":4096,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":423,"record_offset":5016,"parent_record_offset":4848,"ancestor_record_offset":4096,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":423,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":4096,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":5356,"parent_record_offset":5320,"ancestor_record_offset":4096,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":5412,"parent_record_offset":5320,"ancestor_record_offset":4096,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":5240,"parent_record_offset":4848,"ancestor_record_offset":4096,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":5072,"parent_record_offset":4848,"ancestor_record_offset":4096,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":5100,"parent_record_offset":4848,"ancestor_record_offset":4096,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":4776,"parent_record_offset":4696,"ancestor_record_offset":4096,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":4744,"parent_record_offset":4696,"ancestor_record_offset":4096,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":4572,"parent_record_offset":4544,"ancestor_record_offset":4096,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":5752,"parent_record_offset":5560,"ancestor_record_offset":4096,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":4264,"parent_record_offset":4232,"ancestor_record_offset":4096,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":5844,"parent_record_offset":5812,"ancestor_record_offset":4096,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":4476,"parent_record_offset":4444,"ancestor_record_offset":4096,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":4288,"parent_record_offset":4264,"ancestor_record_offset":4096,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":4500,"parent_record_offset":4476,"ancestor_record_offset":4096,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":5776,"parent_record_offset":5752,"ancestor_record_offset":4096,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":423,"record_offset":5868,"parent_record_offset":5844,"ancestor_record_offset":4096,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":423,"record_offset":4444,"parent_record_offset":4416,"ancestor_record_offset":4096,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":423,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":4096,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":423,"record_offset":5560,"parent_record_offset":5516,"ancestor_record_offset":4096,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":423,"record_offset":4544,"parent_record_offset":4352,"ancestor_record_offset":4096,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":423,"record_offset":4352,"parent_record_offset":4324,"ancestor_record_offset":4096,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":423,"record_offset":4324,"parent_record_offset":4192,"ancestor_record_offset":4096,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":423,"record_offset":5516,"parent_record_offset":5484,"ancestor_record_offset":4096,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":423,"record_offset":4232,"parent_record_offset":4192,"ancestor_record_offset":4096,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":423,"record_offset":5812,"parent_record_offset":5516,"ancestor_record_offset":4096,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":423,"record_offset":4388,"parent_record_offset":4352,"ancestor_record_offset":4096,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::maintain_deferred_stone_foundation_surface_buffer_disposal::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":4096,"inlinee":26583,"source_line":105,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::maintain_deferred_stone_foundation_surface_buffer_disposal::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67D7
// 首条位置: {"module":423,"inlinee":26583}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::despawn_stone_foundation_surface
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":423,"record_offset":280,"rva":8628704,"va":5377337824}
// 编译实例 RVAs: 0x83a9e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-1659406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377337824,"raw_hex":"e8f2ade6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377338105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ce3b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4638347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377337824,"raw_hex":"e87539b9ff","size":5,"soft_fail":false,"target_va":5372699568,"va":5377337910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3858858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377337824,"raw_hex":"e8561ec5ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377337893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098aae0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1375815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377337824,"raw_hex":"e847fe1400","size":5,"soft_fail":false,"target_va":5378714336,"va":5377338516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7080693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377337824,"raw_hex":"e8f50a6c00","size":5,"soft_fail":false,"target_va":5384418976,"va":5377338278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":423,"record_offset":472,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":423,"record_offset":2548,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":10329,"source_line":442,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":423,"record_offset":648,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":2576,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":528,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7238,"source_line":1350,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":1264,"parent_record_offset":1208,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":1236,"parent_record_offset":1208,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":280,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":280,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":280,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":280,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":423,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":280,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":423,"record_offset":736,"parent_record_offset":680,"ancestor_record_offset":280,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":423,"record_offset":680,"parent_record_offset":648,"ancestor_record_offset":280,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":423,"record_offset":1440,"parent_record_offset":1412,"ancestor_record_offset":280,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":423,"record_offset":1412,"parent_record_offset":680,"ancestor_record_offset":280,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":1496,"parent_record_offset":1440,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":423,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":2360,"parent_record_offset":2324,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":808,"parent_record_offset":736,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":1672,"parent_record_offset":1640,"ancestor_record_offset":280,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":776,"parent_record_offset":736,"ancestor_record_offset":280,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":984,"parent_record_offset":736,"ancestor_record_offset":280,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":280,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":1640,"parent_record_offset":1496,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":280,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":280,"inlinee":10740,"source_line":2481,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":1524,"parent_record_offset":1496,"ancestor_record_offset":280,"inlinee":10739,"source_line":2616,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":1040,"parent_record_offset":736,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":892,"parent_record_offset":832,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":920,"parent_record_offset":832,"ancestor_record_offset":280,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":864,"parent_record_offset":832,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":280,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":423,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":280,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":423,"record_offset":580,"parent_record_offset":552,"ancestor_record_offset":280,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":423,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":280,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::despawn_stone_foundation_surface::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":280,"inlinee":26579,"source_line":117,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::despawn_stone_foundation_surface::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67D3
// 首条位置: {"module":423,"inlinee":26579}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":1696,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":1696,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh_and_entities
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67D8
// 首条位置: {"module":423,"inlinee":26584}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":423,"record_offset":36464,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":423,"record_offset":36280,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":9104,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":36672,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":36548,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":423,"record_offset":37596,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":423,"record_offset":36924,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":423,"record_offset":36896,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":9160,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":11632,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":11660,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":37224,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":10684,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":423,"record_offset":37652,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":11688,"parent_record_offset":8924,"ancestor_record_offset":8924,"inlinee":26587,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":8924,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":8924,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13728,"parent_record_offset":13700,"ancestor_record_offset":8924,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13756,"parent_record_offset":13728,"ancestor_record_offset":8924,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13480,"parent_record_offset":13456,"ancestor_record_offset":8924,"inlinee":6570,"source_line":353,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13504,"parent_record_offset":13480,"ancestor_record_offset":8924,"inlinee":6571,"source_line":264,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":8924,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13784,"parent_record_offset":13756,"ancestor_record_offset":8924,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17668,"parent_record_offset":17520,"ancestor_record_offset":8924,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13700,"parent_record_offset":13672,"ancestor_record_offset":8924,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13456,"parent_record_offset":12812,"ancestor_record_offset":8924,"inlinee":6569,"source_line":471,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13528,"parent_record_offset":13504,"ancestor_record_offset":8924,"inlinee":6572,"source_line":134,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":423,"record_offset":36308,"parent_record_offset":36280,"ancestor_record_offset":8924,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":423,"record_offset":38024,"parent_record_offset":38000,"ancestor_record_offset":8924,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":8924,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":37072,"parent_record_offset":37044,"ancestor_record_offset":8924,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17460,"parent_record_offset":17428,"ancestor_record_offset":8924,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":37156,"parent_record_offset":37132,"ancestor_record_offset":8924,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":16568,"parent_record_offset":16540,"ancestor_record_offset":8924,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":13672,"parent_record_offset":13644,"ancestor_record_offset":8924,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":37132,"parent_record_offset":37100,"ancestor_record_offset":8924,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":8924,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":9856,"parent_record_offset":9832,"ancestor_record_offset":8924,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":37100,"parent_record_offset":37072,"ancestor_record_offset":8924,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17520,"parent_record_offset":17488,"ancestor_record_offset":8924,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17488,"parent_record_offset":17460,"ancestor_record_offset":8924,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":8924,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17556,"parent_record_offset":17520,"ancestor_record_offset":8924,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":36488,"parent_record_offset":36464,"ancestor_record_offset":8924,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":16324,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":16356,"parent_record_offset":16324,"ancestor_record_offset":8924,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":8924,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12188,"parent_record_offset":12148,"ancestor_record_offset":8924,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":9808,"parent_record_offset":9784,"ancestor_record_offset":8924,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":9784,"parent_record_offset":9752,"ancestor_record_offset":8924,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12120,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27472,"parent_record_offset":21232,"ancestor_record_offset":8924,"inlinee":11483,"source_line":3637,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":16956,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":36512,"parent_record_offset":36488,"ancestor_record_offset":8924,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17060,"parent_record_offset":16956,"ancestor_record_offset":8924,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14400,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14428,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14452,"parent_record_offset":14428,"ancestor_record_offset":8924,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":37044,"parent_record_offset":37012,"ancestor_record_offset":8924,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17364,"parent_record_offset":16956,"ancestor_record_offset":8924,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17396,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17428,"parent_record_offset":17396,"ancestor_record_offset":8924,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":19964,"parent_record_offset":19904,"ancestor_record_offset":8924,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12724,"parent_record_offset":12680,"ancestor_record_offset":8924,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12596,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17800,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$6::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27604,"parent_record_offset":27504,"ancestor_record_offset":8924,"inlinee":10784,"source_line":3638,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":8924,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":12812,"parent_record_offset":12768,"ancestor_record_offset":8924,"inlinee":10798,"source_line":249,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":12768,"parent_record_offset":12724,"ancestor_record_offset":8924,"inlinee":8793,"source_line":232,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$3::collect_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":12876,"parent_record_offset":12812,"ancestor_record_offset":8924,"inlinee":10803,"source_line":389,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::needs_realloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":13428,"parent_record_offset":12812,"ancestor_record_offset":8924,"inlinee":13390,"source_line":192,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_drop::impl$2::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_drop.rs
//     evidence={"module":423,"record_offset":13592,"parent_record_offset":13568,"ancestor_record_offset":8924,"inlinee":13389,"source_line":41,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":27892,"parent_record_offset":27868,"ancestor_record_offset":8924,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":27940,"parent_record_offset":27916,"ancestor_record_offset":8924,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":13220,"parent_record_offset":13184,"ancestor_record_offset":8924,"inlinee":10805,"source_line":362,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":25292,"parent_record_offset":21232,"ancestor_record_offset":8924,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":423,"record_offset":9540,"parent_record_offset":9256,"ancestor_record_offset":8924,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":423,"record_offset":37620,"parent_record_offset":37596,"ancestor_record_offset":8924,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9304,"parent_record_offset":9256,"ancestor_record_offset":8924,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":8924,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9444,"parent_record_offset":9412,"ancestor_record_offset":8924,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9468,"parent_record_offset":9444,"ancestor_record_offset":8924,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":10516,"parent_record_offset":10492,"ancestor_record_offset":8924,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":9960,"parent_record_offset":9256,"ancestor_record_offset":8924,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":8924,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":10492,"parent_record_offset":9256,"ancestor_record_offset":8924,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":10540,"parent_record_offset":10516,"ancestor_record_offset":8924,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":423,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":8924,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":423,"record_offset":10084,"parent_record_offset":10060,"ancestor_record_offset":8924,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":423,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":8924,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":423,"record_offset":10264,"parent_record_offset":10240,"ancestor_record_offset":8924,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10036,"parent_record_offset":10008,"ancestor_record_offset":8924,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10008,"parent_record_offset":9984,"ancestor_record_offset":8924,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10564,"parent_record_offset":10540,"ancestor_record_offset":8924,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":9728,"parent_record_offset":9256,"ancestor_record_offset":8924,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":423,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":8924,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":8924,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":423,"record_offset":36984,"parent_record_offset":36956,"ancestor_record_offset":8924,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":423,"record_offset":37012,"parent_record_offset":36984,"ancestor_record_offset":8924,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":423,"record_offset":36956,"parent_record_offset":36924,"ancestor_record_offset":8924,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":423,"record_offset":9224,"parent_record_offset":9196,"ancestor_record_offset":8924,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":423,"record_offset":9256,"parent_record_offset":9224,"ancestor_record_offset":8924,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":8924,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":423,"record_offset":15824,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":423,"record_offset":15852,"parent_record_offset":15824,"ancestor_record_offset":8924,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":423,"record_offset":15924,"parent_record_offset":15880,"ancestor_record_offset":8924,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":423,"record_offset":9576,"parent_record_offset":9540,"ancestor_record_offset":8924,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19272,"parent_record_offset":19160,"ancestor_record_offset":8924,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19808,"parent_record_offset":19784,"ancestor_record_offset":8924,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":27628,"parent_record_offset":27604,"ancestor_record_offset":8924,"inlinee":10786,"source_line":2197,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":27652,"parent_record_offset":27628,"ancestor_record_offset":8924,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19832,"parent_record_offset":19808,"ancestor_record_offset":8924,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":13016,"parent_record_offset":12972,"ancestor_record_offset":8924,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":11392,"parent_record_offset":11368,"ancestor_record_offset":8924,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":8924,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":423,"record_offset":10960,"parent_record_offset":10936,"ancestor_record_offset":8924,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":423,"record_offset":11044,"parent_record_offset":11020,"ancestor_record_offset":8924,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":8924,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":12568,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":26356,"parent_record_offset":21232,"ancestor_record_offset":8924,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":19236,"parent_record_offset":19212,"ancestor_record_offset":8924,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":15388,"parent_record_offset":15364,"ancestor_record_offset":8924,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":12536,"parent_record_offset":12512,"ancestor_record_offset":8924,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":20416,"parent_record_offset":20392,"ancestor_record_offset":8924,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":423,"record_offset":11460,"parent_record_offset":10824,"ancestor_record_offset":8924,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":423,"record_offset":17628,"parent_record_offset":17604,"ancestor_record_offset":8924,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":423,"record_offset":28324,"parent_record_offset":21232,"ancestor_record_offset":8924,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::filter_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18948,"parent_record_offset":18880,"ancestor_record_offset":8924,"inlinee":11536,"source_line":79,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18856,"parent_record_offset":18832,"ancestor_record_offset":8924,"inlinee":11531,"source_line":161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18444,"parent_record_offset":18416,"ancestor_record_offset":8924,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":13112,"parent_record_offset":12876,"ancestor_record_offset":8924,"inlinee":10804,"source_line":132,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":18832,"parent_record_offset":18380,"ancestor_record_offset":8924,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":14924,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":19112,"parent_record_offset":18948,"ancestor_record_offset":8924,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$19::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":13184,"parent_record_offset":13148,"ancestor_record_offset":8924,"inlinee":6629,"source_line":629,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":13148,"parent_record_offset":13112,"ancestor_record_offset":8924,"inlinee":6627,"source_line":606,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":13048,"parent_record_offset":12972,"ancestor_record_offset":8924,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":12972,"parent_record_offset":12932,"ancestor_record_offset":8924,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":12932,"parent_record_offset":12876,"ancestor_record_offset":8924,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":423,"record_offset":25076,"parent_record_offset":25052,"ancestor_record_offset":8924,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":12680,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":18348,"parent_record_offset":18316,"ancestor_record_offset":8924,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":19136,"parent_record_offset":19112,"ancestor_record_offset":8924,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":18380,"parent_record_offset":18348,"ancestor_record_offset":8924,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":8924,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":423,"record_offset":34192,"parent_record_offset":34164,"ancestor_record_offset":8924,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":423,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":8924,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":423,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":8924,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":423,"record_offset":15948,"parent_record_offset":15924,"ancestor_record_offset":8924,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":17604,"parent_record_offset":17580,"ancestor_record_offset":8924,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":13072,"parent_record_offset":13048,"ancestor_record_offset":8924,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":16600,"parent_record_offset":16568,"ancestor_record_offset":8924,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":423,"record_offset":11192,"parent_record_offset":11164,"ancestor_record_offset":8924,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":423,"record_offset":11164,"parent_record_offset":11136,"ancestor_record_offset":8924,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":423,"record_offset":18700,"parent_record_offset":18620,"ancestor_record_offset":8924,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":18668,"parent_record_offset":18644,"ancestor_record_offset":8924,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":19636,"parent_record_offset":19612,"ancestor_record_offset":8924,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":423,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":8924,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":18724,"parent_record_offset":18700,"ancestor_record_offset":8924,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":19212,"parent_record_offset":19188,"ancestor_record_offset":8924,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":10176,"parent_record_offset":10112,"ancestor_record_offset":8924,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":18596,"parent_record_offset":18472,"ancestor_record_offset":8924,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":15064,"parent_record_offset":15040,"ancestor_record_offset":8924,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":423,"record_offset":16036,"parent_record_offset":15880,"ancestor_record_offset":8924,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":423,"record_offset":15880,"parent_record_offset":15852,"ancestor_record_offset":8924,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":423,"record_offset":19784,"parent_record_offset":19760,"ancestor_record_offset":8924,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":423,"record_offset":19760,"parent_record_offset":19728,"ancestor_record_offset":8924,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":10200,"parent_record_offset":10176,"ancestor_record_offset":8924,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":18280,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":9412,"parent_record_offset":9328,"ancestor_record_offset":8924,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":28268,"parent_record_offset":21232,"ancestor_record_offset":8924,"inlinee":14503,"source_line":1754,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":28292,"parent_record_offset":28268,"ancestor_record_offset":8924,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":12452,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":12476,"parent_record_offset":12452,"ancestor_record_offset":8924,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":12392,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":10240,"parent_record_offset":10060,"ancestor_record_offset":8924,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":9512,"parent_record_offset":9256,"ancestor_record_offset":8924,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":18092,"parent_record_offset":18056,"ancestor_record_offset":8924,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":15972,"parent_record_offset":15948,"ancestor_record_offset":8924,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":10984,"parent_record_offset":10960,"ancestor_record_offset":8924,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=74
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":12328,"parent_record_offset":12148,"ancestor_record_offset":8924,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":9680,"parent_record_offset":9648,"ancestor_record_offset":8924,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":11308,"parent_record_offset":11284,"ancestor_record_offset":8924,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":15008,"parent_record_offset":14948,"ancestor_record_offset":8924,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":13256,"parent_record_offset":13220,"ancestor_record_offset":8924,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":14980,"parent_record_offset":14948,"ancestor_record_offset":8924,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":13300,"parent_record_offset":13256,"ancestor_record_offset":8924,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":13360,"parent_record_offset":12876,"ancestor_record_offset":8924,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":423,"record_offset":14064,"parent_record_offset":14036,"ancestor_record_offset":8924,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":9356,"parent_record_offset":9328,"ancestor_record_offset":8924,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":13896,"parent_record_offset":13864,"ancestor_record_offset":8924,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":28448,"parent_record_offset":28420,"ancestor_record_offset":8924,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":19384,"parent_record_offset":19340,"ancestor_record_offset":8924,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":9380,"parent_record_offset":9356,"ancestor_record_offset":8924,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":10320,"parent_record_offset":10288,"ancestor_record_offset":8924,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":19416,"parent_record_offset":19384,"ancestor_record_offset":8924,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":8924,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":8924,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":28492,"parent_record_offset":21232,"ancestor_record_offset":8924,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":18472,"parent_record_offset":18444,"ancestor_record_offset":8924,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":18880,"parent_record_offset":18856,"ancestor_record_offset":8924,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":14948,"parent_record_offset":14924,"ancestor_record_offset":8924,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":23060,"parent_record_offset":21232,"ancestor_record_offset":8924,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":28360,"parent_record_offset":28324,"ancestor_record_offset":8924,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":423,"record_offset":16388,"parent_record_offset":16356,"ancestor_record_offset":8924,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":423,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":8924,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":423,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":8924,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":423,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":8924,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":423,"record_offset":9600,"parent_record_offset":9576,"ancestor_record_offset":8924,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20656,"parent_record_offset":20620,"ancestor_record_offset":8924,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20620,"parent_record_offset":20580,"ancestor_record_offset":8924,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":8924,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20556,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20528,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20360,"parent_record_offset":20328,"ancestor_record_offset":8924,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20328,"parent_record_offset":20296,"ancestor_record_offset":8924,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20296,"parent_record_offset":20272,"ancestor_record_offset":8924,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20272,"parent_record_offset":20248,"ancestor_record_offset":8924,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":19612,"parent_record_offset":19340,"ancestor_record_offset":8924,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":19668,"parent_record_offset":19340,"ancestor_record_offset":8924,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":19556,"parent_record_offset":19340,"ancestor_record_offset":8924,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":423,"record_offset":11104,"parent_record_offset":11076,"ancestor_record_offset":8924,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":423,"record_offset":11136,"parent_record_offset":11104,"ancestor_record_offset":8924,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":423,"record_offset":11076,"parent_record_offset":10824,"ancestor_record_offset":8924,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":423,"record_offset":10936,"parent_record_offset":10824,"ancestor_record_offset":8924,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":423,"record_offset":11488,"parent_record_offset":10824,"ancestor_record_offset":8924,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":423,"record_offset":11020,"parent_record_offset":10824,"ancestor_record_offset":8924,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":423,"record_offset":10880,"parent_record_offset":10824,"ancestor_record_offset":8924,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":423,"record_offset":11368,"parent_record_offset":11344,"ancestor_record_offset":8924,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":423,"record_offset":10712,"parent_record_offset":10684,"ancestor_record_offset":8924,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":423,"record_offset":11344,"parent_record_offset":11236,"ancestor_record_offset":8924,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":423,"record_offset":10748,"parent_record_offset":10712,"ancestor_record_offset":8924,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":11284,"parent_record_offset":11260,"ancestor_record_offset":8924,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":11580,"parent_record_offset":10824,"ancestor_record_offset":8924,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":11260,"parent_record_offset":11236,"ancestor_record_offset":8924,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":8924,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":10776,"parent_record_offset":10712,"ancestor_record_offset":8924,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":10824,"parent_record_offset":10800,"ancestor_record_offset":8924,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":10908,"parent_record_offset":10824,"ancestor_record_offset":8924,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":423,"record_offset":11236,"parent_record_offset":10824,"ancestor_record_offset":8924,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":423,"record_offset":10596,"parent_record_offset":10564,"ancestor_record_offset":8924,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":423,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":8924,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14292,"parent_record_offset":14256,"ancestor_record_offset":8924,"inlinee":26594,"source_line":1716,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14140,"parent_record_offset":14108,"ancestor_record_offset":8924,"inlinee":26590,"source_line":952,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14224,"parent_record_offset":14108,"ancestor_record_offset":8924,"inlinee":26591,"source_line":919,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14188,"parent_record_offset":14164,"ancestor_record_offset":8924,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14164,"parent_record_offset":14140,"ancestor_record_offset":8924,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":34164,"parent_record_offset":34140,"ancestor_record_offset":8924,"inlinee":4820,"source_line":2118,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":34052,"parent_record_offset":34028,"ancestor_record_offset":8924,"inlinee":26609,"source_line":2228,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14256,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14108,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":26589,"source_line":2290,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":15364,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":12512,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":20392,"parent_record_offset":20360,"ancestor_record_offset":8924,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":21232,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":26604,"source_line":427,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":28520,"parent_record_offset":28492,"ancestor_record_offset":8924,"inlinee":26607,"source_line":554,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":18316,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":26599,"source_line":271,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":8924,"inlinee":26601,"source_line":274,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":15088,"parent_record_offset":15064,"ancestor_record_offset":8924,"inlinee":26596,"source_line":294,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":17200,"parent_record_offset":16956,"ancestor_record_offset":8924,"inlinee":26597,"source_line":318,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh_and_entities::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":9196,"parent_record_offset":9160,"ancestor_record_offset":8924,"inlinee":26586,"source_line":138,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::stone_foundation_surface_vertex_from_wallspace
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":23452,"parent_record_offset":21232,"ancestor_record_offset":8924,"inlinee":26605,"source_line":575,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":14036,"parent_record_offset":13864,"ancestor_record_offset":8924,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":13864,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":26086,"source_line":1147,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":17976,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":26598,"source_line":1171,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19876,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":26602,"source_line":816,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19904,"parent_record_offset":19876,"ancestor_record_offset":8924,"inlinee":26603,"source_line":812,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":12364,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19456,"parent_record_offset":19340,"ancestor_record_offset":8924,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18248,"parent_record_offset":17976,"ancestor_record_offset":8924,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18220,"parent_record_offset":17976,"ancestor_record_offset":8924,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18024,"parent_record_offset":17976,"ancestor_record_offset":8924,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18056,"parent_record_offset":18024,"ancestor_record_offset":8924,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18156,"parent_record_offset":18128,"ancestor_record_offset":8924,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18128,"parent_record_offset":17976,"ancestor_record_offset":8924,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":19728,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":15112,"parent_record_offset":15088,"ancestor_record_offset":8924,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19340,"parent_record_offset":11688,"ancestor_record_offset":8924,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":423,"record_offset":20248,"parent_record_offset":19876,"ancestor_record_offset":8924,"inlinee":26418,"source_line":99,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh_and_entities::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67DA
// 首条位置: {"module":423,"inlinee":26586}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":423,"record_offset":9224,"parent_record_offset":9196,"ancestor_record_offset":9196,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":9196,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":9196,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":9856,"parent_record_offset":9832,"ancestor_record_offset":9196,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":9808,"parent_record_offset":9784,"ancestor_record_offset":9196,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":9784,"parent_record_offset":9752,"ancestor_record_offset":9196,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":9196,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":423,"record_offset":9540,"parent_record_offset":9256,"ancestor_record_offset":9196,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9304,"parent_record_offset":9256,"ancestor_record_offset":9196,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":9196,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9444,"parent_record_offset":9412,"ancestor_record_offset":9196,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":423,"record_offset":9468,"parent_record_offset":9444,"ancestor_record_offset":9196,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":10516,"parent_record_offset":10492,"ancestor_record_offset":9196,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":9960,"parent_record_offset":9256,"ancestor_record_offset":9196,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":9196,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":10492,"parent_record_offset":9256,"ancestor_record_offset":9196,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":423,"record_offset":10540,"parent_record_offset":10516,"ancestor_record_offset":9196,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":423,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":9196,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":423,"record_offset":10084,"parent_record_offset":10060,"ancestor_record_offset":9196,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":423,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":9196,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":423,"record_offset":10264,"parent_record_offset":10240,"ancestor_record_offset":9196,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10036,"parent_record_offset":10008,"ancestor_record_offset":9196,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10008,"parent_record_offset":9984,"ancestor_record_offset":9196,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10564,"parent_record_offset":10540,"ancestor_record_offset":9196,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":9728,"parent_record_offset":9256,"ancestor_record_offset":9196,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":423,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":9196,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":423,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":9196,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":423,"record_offset":9256,"parent_record_offset":9224,"ancestor_record_offset":9196,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":423,"record_offset":9576,"parent_record_offset":9540,"ancestor_record_offset":9196,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":10176,"parent_record_offset":10112,"ancestor_record_offset":9196,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":10200,"parent_record_offset":10176,"ancestor_record_offset":9196,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":9412,"parent_record_offset":9328,"ancestor_record_offset":9196,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":10240,"parent_record_offset":10060,"ancestor_record_offset":9196,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":9512,"parent_record_offset":9256,"ancestor_record_offset":9196,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":9680,"parent_record_offset":9648,"ancestor_record_offset":9196,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":9356,"parent_record_offset":9328,"ancestor_record_offset":9196,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":9380,"parent_record_offset":9356,"ancestor_record_offset":9196,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":10320,"parent_record_offset":10288,"ancestor_record_offset":9196,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":423,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":9196,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":423,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":9196,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":423,"record_offset":9600,"parent_record_offset":9576,"ancestor_record_offset":9196,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":423,"record_offset":10596,"parent_record_offset":10564,"ancestor_record_offset":9196,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":423,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":9196,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67DB
// 首条位置: {"module":423,"inlinee":26587}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":16324,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12120,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":16956,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14400,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14428,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17396,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12596,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17800,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":423,"record_offset":15824,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":12568,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":14924,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":15648,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":12680,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":18280,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":12452,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":12392,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":16468,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":14840,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":15732,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20556,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20528,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14256,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14108,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":26589,"source_line":2290,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":15364,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":12512,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":21232,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":26604,"source_line":427,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":18316,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":26599,"source_line":271,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":13864,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":26086,"source_line":1147,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":17976,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":26598,"source_line":1171,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19876,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":26602,"source_line":816,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":12364,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":19728,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19340,"parent_record_offset":11688,"ancestor_record_offset":11688,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":11688,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":11688,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13728,"parent_record_offset":13700,"ancestor_record_offset":11688,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13756,"parent_record_offset":13728,"ancestor_record_offset":11688,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13480,"parent_record_offset":13456,"ancestor_record_offset":11688,"inlinee":6570,"source_line":353,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13504,"parent_record_offset":13480,"ancestor_record_offset":11688,"inlinee":6571,"source_line":264,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":11688,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13784,"parent_record_offset":13756,"ancestor_record_offset":11688,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":17668,"parent_record_offset":17520,"ancestor_record_offset":11688,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13700,"parent_record_offset":13672,"ancestor_record_offset":11688,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13456,"parent_record_offset":12812,"ancestor_record_offset":11688,"inlinee":6569,"source_line":471,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":13528,"parent_record_offset":13504,"ancestor_record_offset":11688,"inlinee":6572,"source_line":134,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":12236,"parent_record_offset":12212,"ancestor_record_offset":11688,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17460,"parent_record_offset":17428,"ancestor_record_offset":11688,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":16568,"parent_record_offset":16540,"ancestor_record_offset":11688,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":13672,"parent_record_offset":13644,"ancestor_record_offset":11688,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":12284,"parent_record_offset":12260,"ancestor_record_offset":11688,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":12260,"parent_record_offset":12236,"ancestor_record_offset":11688,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17520,"parent_record_offset":17488,"ancestor_record_offset":11688,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17488,"parent_record_offset":17460,"ancestor_record_offset":11688,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":11688,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":17556,"parent_record_offset":17520,"ancestor_record_offset":11688,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":25112,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":16356,"parent_record_offset":16324,"ancestor_record_offset":11688,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":11688,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12188,"parent_record_offset":12148,"ancestor_record_offset":11688,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27472,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":11483,"source_line":3637,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27504,"parent_record_offset":27472,"ancestor_record_offset":11688,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17060,"parent_record_offset":16956,"ancestor_record_offset":11688,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":28240,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":23172,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":14452,"parent_record_offset":14428,"ancestor_record_offset":11688,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17364,"parent_record_offset":16956,"ancestor_record_offset":11688,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":21556,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":17428,"parent_record_offset":17396,"ancestor_record_offset":11688,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":19964,"parent_record_offset":19904,"ancestor_record_offset":11688,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":12724,"parent_record_offset":12680,"ancestor_record_offset":11688,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":25428,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$6::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27604,"parent_record_offset":27504,"ancestor_record_offset":11688,"inlinee":10784,"source_line":3638,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":12812,"parent_record_offset":12768,"ancestor_record_offset":11688,"inlinee":10798,"source_line":249,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":12768,"parent_record_offset":12724,"ancestor_record_offset":11688,"inlinee":8793,"source_line":232,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$3::collect_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":12876,"parent_record_offset":12812,"ancestor_record_offset":11688,"inlinee":10803,"source_line":389,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::needs_realloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":13428,"parent_record_offset":12812,"ancestor_record_offset":11688,"inlinee":13390,"source_line":192,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_drop::impl$2::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_drop.rs
//     evidence={"module":423,"record_offset":13592,"parent_record_offset":13568,"ancestor_record_offset":11688,"inlinee":13389,"source_line":41,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":27892,"parent_record_offset":27868,"ancestor_record_offset":11688,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":27940,"parent_record_offset":27916,"ancestor_record_offset":11688,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":13220,"parent_record_offset":13184,"ancestor_record_offset":11688,"inlinee":10805,"source_line":362,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":25292,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":11688,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":423,"record_offset":15852,"parent_record_offset":15824,"ancestor_record_offset":11688,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":423,"record_offset":15924,"parent_record_offset":15880,"ancestor_record_offset":11688,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19272,"parent_record_offset":19160,"ancestor_record_offset":11688,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19808,"parent_record_offset":19784,"ancestor_record_offset":11688,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":27628,"parent_record_offset":27604,"ancestor_record_offset":11688,"inlinee":10786,"source_line":2197,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":27652,"parent_record_offset":27628,"ancestor_record_offset":11688,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19832,"parent_record_offset":19808,"ancestor_record_offset":11688,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":13016,"parent_record_offset":12972,"ancestor_record_offset":11688,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":11688,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":11688,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":18192,"parent_record_offset":17976,"ancestor_record_offset":11688,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":26356,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":19236,"parent_record_offset":19212,"ancestor_record_offset":11688,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":15388,"parent_record_offset":15364,"ancestor_record_offset":11688,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":12536,"parent_record_offset":12512,"ancestor_record_offset":11688,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":20416,"parent_record_offset":20392,"ancestor_record_offset":11688,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":423,"record_offset":16428,"parent_record_offset":16388,"ancestor_record_offset":11688,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":423,"record_offset":17628,"parent_record_offset":17604,"ancestor_record_offset":11688,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":423,"record_offset":28324,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::filter_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18948,"parent_record_offset":18880,"ancestor_record_offset":11688,"inlinee":11536,"source_line":79,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18856,"parent_record_offset":18832,"ancestor_record_offset":11688,"inlinee":11531,"source_line":161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18444,"parent_record_offset":18416,"ancestor_record_offset":11688,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":13112,"parent_record_offset":12876,"ancestor_record_offset":11688,"inlinee":10804,"source_line":132,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":18832,"parent_record_offset":18380,"ancestor_record_offset":11688,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":18416,"parent_record_offset":18380,"ancestor_record_offset":11688,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":19112,"parent_record_offset":18948,"ancestor_record_offset":11688,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$19::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":13184,"parent_record_offset":13148,"ancestor_record_offset":11688,"inlinee":6629,"source_line":629,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":13148,"parent_record_offset":13112,"ancestor_record_offset":11688,"inlinee":6627,"source_line":606,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":13048,"parent_record_offset":12972,"ancestor_record_offset":11688,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":12972,"parent_record_offset":12932,"ancestor_record_offset":11688,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":12932,"parent_record_offset":12876,"ancestor_record_offset":11688,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":423,"record_offset":25076,"parent_record_offset":25052,"ancestor_record_offset":11688,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":13980,"parent_record_offset":13864,"ancestor_record_offset":11688,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":18348,"parent_record_offset":18316,"ancestor_record_offset":11688,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":19136,"parent_record_offset":19112,"ancestor_record_offset":11688,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":18380,"parent_record_offset":18348,"ancestor_record_offset":11688,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":11688,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":423,"record_offset":34192,"parent_record_offset":34164,"ancestor_record_offset":11688,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":423,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":11688,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":423,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":11688,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":423,"record_offset":15948,"parent_record_offset":15924,"ancestor_record_offset":11688,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":17604,"parent_record_offset":17580,"ancestor_record_offset":11688,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":13072,"parent_record_offset":13048,"ancestor_record_offset":11688,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":16600,"parent_record_offset":16568,"ancestor_record_offset":11688,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":423,"record_offset":18700,"parent_record_offset":18620,"ancestor_record_offset":11688,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":18668,"parent_record_offset":18644,"ancestor_record_offset":11688,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":19636,"parent_record_offset":19612,"ancestor_record_offset":11688,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":423,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":11688,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":18724,"parent_record_offset":18700,"ancestor_record_offset":11688,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":19212,"parent_record_offset":19188,"ancestor_record_offset":11688,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":12420,"parent_record_offset":12392,"ancestor_record_offset":11688,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":18596,"parent_record_offset":18472,"ancestor_record_offset":11688,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":15064,"parent_record_offset":15040,"ancestor_record_offset":11688,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":423,"record_offset":16036,"parent_record_offset":15880,"ancestor_record_offset":11688,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":423,"record_offset":15880,"parent_record_offset":15852,"ancestor_record_offset":11688,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":423,"record_offset":19784,"parent_record_offset":19760,"ancestor_record_offset":11688,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":423,"record_offset":19760,"parent_record_offset":19728,"ancestor_record_offset":11688,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":28268,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":14503,"source_line":1754,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":28292,"parent_record_offset":28268,"ancestor_record_offset":11688,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":12476,"parent_record_offset":12452,"ancestor_record_offset":11688,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":15040,"parent_record_offset":14924,"ancestor_record_offset":11688,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":18092,"parent_record_offset":18056,"ancestor_record_offset":11688,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":15972,"parent_record_offset":15948,"ancestor_record_offset":11688,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":17328,"parent_record_offset":16956,"ancestor_record_offset":11688,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=57
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":12328,"parent_record_offset":12148,"ancestor_record_offset":11688,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":13396,"parent_record_offset":12876,"ancestor_record_offset":11688,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":15008,"parent_record_offset":14948,"ancestor_record_offset":11688,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":13256,"parent_record_offset":13220,"ancestor_record_offset":11688,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":14980,"parent_record_offset":14948,"ancestor_record_offset":11688,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":13300,"parent_record_offset":13256,"ancestor_record_offset":11688,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":13360,"parent_record_offset":12876,"ancestor_record_offset":11688,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":423,"record_offset":14064,"parent_record_offset":14036,"ancestor_record_offset":11688,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":13896,"parent_record_offset":13864,"ancestor_record_offset":11688,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":28448,"parent_record_offset":28420,"ancestor_record_offset":11688,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":19384,"parent_record_offset":19340,"ancestor_record_offset":11688,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":19416,"parent_record_offset":19384,"ancestor_record_offset":11688,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":11688,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":11688,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":28492,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":18472,"parent_record_offset":18444,"ancestor_record_offset":11688,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":18880,"parent_record_offset":18856,"ancestor_record_offset":11688,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":14948,"parent_record_offset":14924,"ancestor_record_offset":11688,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":23060,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":28360,"parent_record_offset":28324,"ancestor_record_offset":11688,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":423,"record_offset":16388,"parent_record_offset":16356,"ancestor_record_offset":11688,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20656,"parent_record_offset":20620,"ancestor_record_offset":11688,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20620,"parent_record_offset":20580,"ancestor_record_offset":11688,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":423,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":11688,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20360,"parent_record_offset":20328,"ancestor_record_offset":11688,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20328,"parent_record_offset":20296,"ancestor_record_offset":11688,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20296,"parent_record_offset":20272,"ancestor_record_offset":11688,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":20272,"parent_record_offset":20248,"ancestor_record_offset":11688,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":19612,"parent_record_offset":19340,"ancestor_record_offset":11688,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":19668,"parent_record_offset":19340,"ancestor_record_offset":11688,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":19556,"parent_record_offset":19340,"ancestor_record_offset":11688,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14292,"parent_record_offset":14256,"ancestor_record_offset":11688,"inlinee":26594,"source_line":1716,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14140,"parent_record_offset":14108,"ancestor_record_offset":11688,"inlinee":26590,"source_line":952,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14224,"parent_record_offset":14108,"ancestor_record_offset":11688,"inlinee":26591,"source_line":919,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14188,"parent_record_offset":14164,"ancestor_record_offset":11688,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":14164,"parent_record_offset":14140,"ancestor_record_offset":11688,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":34164,"parent_record_offset":34140,"ancestor_record_offset":11688,"inlinee":4820,"source_line":2118,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":34052,"parent_record_offset":34028,"ancestor_record_offset":11688,"inlinee":26609,"source_line":2228,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":423,"record_offset":34100,"parent_record_offset":34076,"ancestor_record_offset":11688,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":20392,"parent_record_offset":20360,"ancestor_record_offset":11688,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":28520,"parent_record_offset":28492,"ancestor_record_offset":11688,"inlinee":26607,"source_line":554,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":11688,"inlinee":26601,"source_line":274,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":15088,"parent_record_offset":15064,"ancestor_record_offset":11688,"inlinee":26596,"source_line":294,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":17200,"parent_record_offset":16956,"ancestor_record_offset":11688,"inlinee":26597,"source_line":318,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::stone_foundation_surface_vertex_from_wallspace
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":23452,"parent_record_offset":21232,"ancestor_record_offset":11688,"inlinee":26605,"source_line":575,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":14036,"parent_record_offset":13864,"ancestor_record_offset":11688,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19904,"parent_record_offset":19876,"ancestor_record_offset":11688,"inlinee":26603,"source_line":812,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":423,"record_offset":19456,"parent_record_offset":19340,"ancestor_record_offset":11688,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18248,"parent_record_offset":17976,"ancestor_record_offset":11688,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18220,"parent_record_offset":17976,"ancestor_record_offset":11688,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18024,"parent_record_offset":17976,"ancestor_record_offset":11688,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18056,"parent_record_offset":18024,"ancestor_record_offset":11688,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18156,"parent_record_offset":18128,"ancestor_record_offset":11688,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":423,"record_offset":18128,"parent_record_offset":17976,"ancestor_record_offset":11688,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":15112,"parent_record_offset":15088,"ancestor_record_offset":11688,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":423,"record_offset":20248,"parent_record_offset":19876,"ancestor_record_offset":11688,"inlinee":26418,"source_line":99,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 271
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67E7
// 首条位置: {"module":423,"inlinee":26599}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":18348,"parent_record_offset":18316,"ancestor_record_offset":18316,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19272,"parent_record_offset":19160,"ancestor_record_offset":18316,"inlinee":16325,"source_line":499,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":18316,"inlinee":15789,"source_line":1712,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":19236,"parent_record_offset":19212,"ancestor_record_offset":18316,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::filter_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18948,"parent_record_offset":18880,"ancestor_record_offset":18316,"inlinee":11536,"source_line":79,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18856,"parent_record_offset":18832,"ancestor_record_offset":18316,"inlinee":11531,"source_line":161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":423,"record_offset":18444,"parent_record_offset":18416,"ancestor_record_offset":18316,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":18832,"parent_record_offset":18380,"ancestor_record_offset":18316,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":18416,"parent_record_offset":18380,"ancestor_record_offset":18316,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":19112,"parent_record_offset":18948,"ancestor_record_offset":18316,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":19136,"parent_record_offset":19112,"ancestor_record_offset":18316,"inlinee":15788,"source_line":3329,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":18380,"parent_record_offset":18348,"ancestor_record_offset":18316,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":423,"record_offset":18700,"parent_record_offset":18620,"ancestor_record_offset":18316,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":18668,"parent_record_offset":18644,"ancestor_record_offset":18316,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":423,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":18316,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":18724,"parent_record_offset":18700,"ancestor_record_offset":18316,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":19212,"parent_record_offset":19188,"ancestor_record_offset":18316,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":18596,"parent_record_offset":18472,"ancestor_record_offset":18316,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":423,"record_offset":18796,"parent_record_offset":18772,"ancestor_record_offset":18316,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":18772,"parent_record_offset":18416,"ancestor_record_offset":18316,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":18564,"parent_record_offset":18504,"ancestor_record_offset":18316,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":18536,"parent_record_offset":18504,"ancestor_record_offset":18316,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":18472,"parent_record_offset":18444,"ancestor_record_offset":18316,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":18880,"parent_record_offset":18856,"ancestor_record_offset":18316,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":18504,"parent_record_offset":18472,"ancestor_record_offset":18316,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":18316,"inlinee":26601,"source_line":274,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 274
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67E9
// 首条位置: {"module":423,"inlinee":26601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":423,"record_offset":18700,"parent_record_offset":18620,"ancestor_record_offset":18620,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":423,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":18620,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":18668,"parent_record_offset":18644,"ancestor_record_offset":18620,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":18724,"parent_record_offset":18700,"ancestor_record_offset":18620,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67E4
// 首条位置: {"module":423,"inlinee":26596}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":423,"record_offset":15112,"parent_record_offset":15088,"ancestor_record_offset":15088,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=30
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1380F
// 首条位置: {"module":1500,"inlinee":79887}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1500,"record_offset":100036,"parent_record_offset":100012,"ancestor_record_offset":100012,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 318
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67E5
// 首条位置: {"module":423,"inlinee":26597}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":17200,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 427
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67EC
// 首条位置: {"module":423,"inlinee":26604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":25112,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27472,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":11483,"source_line":3637,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":28240,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":23172,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":21556,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":25428,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":25292,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":26328,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":26356,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":423,"record_offset":28324,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":25028,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":28268,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":14503,"source_line":1754,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":26888,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":28492,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":23396,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":23060,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::stone_foundation_surface_systems::stone_foundation_surface_vertex_from_wallspace
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":23452,"parent_record_offset":21232,"ancestor_record_offset":21232,"inlinee":26605,"source_line":575,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":21848,"parent_record_offset":21824,"ancestor_record_offset":21232,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":21872,"parent_record_offset":21848,"ancestor_record_offset":21232,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":27076,"parent_record_offset":27052,"ancestor_record_offset":21232,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":21232,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":21896,"parent_record_offset":21872,"ancestor_record_offset":21232,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":27124,"parent_record_offset":27100,"ancestor_record_offset":21232,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":21824,"parent_record_offset":21680,"ancestor_record_offset":21232,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":27052,"parent_record_offset":26960,"ancestor_record_offset":21232,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":23272,"parent_record_offset":23240,"ancestor_record_offset":21232,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":21620,"parent_record_offset":21588,"ancestor_record_offset":21232,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":26988,"parent_record_offset":26960,"ancestor_record_offset":21232,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":26960,"parent_record_offset":26936,"ancestor_record_offset":21232,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":23320,"parent_record_offset":23296,"ancestor_record_offset":21232,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":23296,"parent_record_offset":23272,"ancestor_record_offset":21232,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":21680,"parent_record_offset":21648,"ancestor_record_offset":21232,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":21648,"parent_record_offset":21620,"ancestor_record_offset":21232,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":26936,"parent_record_offset":26912,"ancestor_record_offset":21232,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":21712,"parent_record_offset":21680,"ancestor_record_offset":21232,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":423,"record_offset":25148,"parent_record_offset":25112,"ancestor_record_offset":21232,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":23240,"parent_record_offset":23200,"ancestor_record_offset":21232,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27504,"parent_record_offset":27472,"ancestor_record_offset":21232,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":23200,"parent_record_offset":23172,"ancestor_record_offset":21232,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27832,"parent_record_offset":27504,"ancestor_record_offset":21232,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":21588,"parent_record_offset":21556,"ancestor_record_offset":21232,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":25052,"parent_record_offset":25028,"ancestor_record_offset":21232,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$6::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":27604,"parent_record_offset":27504,"ancestor_record_offset":21232,"inlinee":10784,"source_line":3638,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":27892,"parent_record_offset":27868,"ancestor_record_offset":21232,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":27940,"parent_record_offset":27916,"ancestor_record_offset":21232,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":21736,"parent_record_offset":21712,"ancestor_record_offset":21232,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":27628,"parent_record_offset":27604,"ancestor_record_offset":21232,"inlinee":10786,"source_line":2197,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":27652,"parent_record_offset":27628,"ancestor_record_offset":21232,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":423,"record_offset":25224,"parent_record_offset":25184,"ancestor_record_offset":21232,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":423,"record_offset":21784,"parent_record_offset":21760,"ancestor_record_offset":21232,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":423,"record_offset":25076,"parent_record_offset":25052,"ancestor_record_offset":21232,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":21232,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":27020,"parent_record_offset":26988,"ancestor_record_offset":21232,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":23604,"parent_record_offset":23576,"ancestor_record_offset":21232,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":23516,"parent_record_offset":23488,"ancestor_record_offset":21232,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":28292,"parent_record_offset":28268,"ancestor_record_offset":21232,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":27796,"parent_record_offset":27504,"ancestor_record_offset":21232,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":26912,"parent_record_offset":26888,"ancestor_record_offset":21232,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":23132,"parent_record_offset":23108,"ancestor_record_offset":21232,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":25368,"parent_record_offset":25292,"ancestor_record_offset":21232,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":23420,"parent_record_offset":23396,"ancestor_record_offset":21232,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":25392,"parent_record_offset":25368,"ancestor_record_offset":21232,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":23360,"parent_record_offset":23200,"ancestor_record_offset":21232,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":23084,"parent_record_offset":23060,"ancestor_record_offset":21232,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":28448,"parent_record_offset":28420,"ancestor_record_offset":21232,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":28396,"parent_record_offset":28360,"ancestor_record_offset":21232,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":21232,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":23108,"parent_record_offset":23084,"ancestor_record_offset":21232,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":28360,"parent_record_offset":28324,"ancestor_record_offset":21232,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":423,"record_offset":25184,"parent_record_offset":25148,"ancestor_record_offset":21232,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":23488,"parent_record_offset":23452,"ancestor_record_offset":21232,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":23576,"parent_record_offset":23452,"ancestor_record_offset":21232,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":28520,"parent_record_offset":28492,"ancestor_record_offset":21232,"inlinee":26607,"source_line":554,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 493
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135FB
// 首条位置: {"module":1455,"inlinee":79355}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 495
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135FC
// 首条位置: {"module":1455,"inlinee":79356}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1455,"record_offset":127008,"parent_record_offset":126984,"ancestor_record_offset":126984,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 554
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67EF
// 首条位置: {"module":423,"inlinee":26607}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::stone_foundation_surface_vertex_from_wallspace
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 575
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67ED
// 首条位置: {"module":423,"inlinee":26605}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":23488,"parent_record_offset":23452,"ancestor_record_offset":23452,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":23576,"parent_record_offset":23452,"ancestor_record_offset":23452,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":23604,"parent_record_offset":23576,"ancestor_record_offset":23452,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":23516,"parent_record_offset":23488,"ancestor_record_offset":23452,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::append_stone_foundation_surface_face
// 记录形式: compiled_procedure; instances=1
// 源码行: 602
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":423,"record_offset":39160,"rva":8647712,"va":5377356832}
// 编译实例 RVAs: 0x83f420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..buffer..Buffer$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..buffer..BufferSchema$GT$$GT$::hot
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6107000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e8782f5d00","size":5,"soft_fail":false,"target_va":5383464400,"va":5377357395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1678372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e8dc63e6ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377357055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-1679880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e8f85de6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377358579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8634849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e81f3e7cff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377357036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7999674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e846ef85ff","size":5,"soft_fail":false,"target_va":5369358064,"va":5377357733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140cd70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12374052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e824d0bc00","size":5,"soft_fail":false,"target_va":5389733232,"va":5377359175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14147f4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12844706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e8a2fec300","size":5,"soft_fail":false,"target_va":5390202048,"va":5377357337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38992802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e8a2fb5202","size":5,"soft_fail":false,"target_va":5416350496,"va":5377357689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38995863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e897075302","size":5,"soft_fail":false,"target_va":5416353040,"va":5377357172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: stone_foundation_surface_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39699848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e888c55d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5377357203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39167250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e812a55502","size":5,"soft_fail":false,"target_va":5416524720,"va":5377357465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39263782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e8261e5702","size":5,"soft_fail":false,"target_va":5416620794,"va":5377357007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t39262941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e8dd1a5702","size":5,"soft_fail":false,"target_va":5416621762,"va":5377358816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39263525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e8251d5702","size":5,"soft_fail":false,"target_va":5416622320,"va":5377358790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t6130271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377356832,"raw_hex":"e85f8a5d00","size":5,"soft_fail":false,"target_va":5383487568,"va":5377357292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377356832,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5377357443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":39840,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":40956,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":39412,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":42624,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":41628,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":39384,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":423,"record_offset":41176,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":41656,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":41316,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":41204,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferHandleExt::device_address
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":423,"record_offset":40840,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":26371,"source_line":129,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::stone_foundation_surface_systems::calculate_stone_foundation_surface_strip_bounds
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs
//     evidence={"module":423,"record_offset":42724,"parent_record_offset":39160,"ancestor_record_offset":39160,"inlinee":26610,"source_line":671,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":39732,"parent_record_offset":39708,"ancestor_record_offset":39160,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":39756,"parent_record_offset":39732,"ancestor_record_offset":39160,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":41912,"parent_record_offset":41888,"ancestor_record_offset":39160,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":41936,"parent_record_offset":41912,"ancestor_record_offset":39160,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":39780,"parent_record_offset":39756,"ancestor_record_offset":39160,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":41960,"parent_record_offset":41936,"ancestor_record_offset":39160,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":39708,"parent_record_offset":39524,"ancestor_record_offset":39160,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":41888,"parent_record_offset":41832,"ancestor_record_offset":39160,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":40276,"parent_record_offset":40232,"ancestor_record_offset":39160,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":40132,"parent_record_offset":40104,"ancestor_record_offset":39160,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":39468,"parent_record_offset":39440,"ancestor_record_offset":39160,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":40616,"parent_record_offset":40592,"ancestor_record_offset":39160,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":41860,"parent_record_offset":41832,"ancestor_record_offset":39160,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":41832,"parent_record_offset":41808,"ancestor_record_offset":39160,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":40192,"parent_record_offset":40160,"ancestor_record_offset":39160,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":40324,"parent_record_offset":40300,"ancestor_record_offset":39160,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":40300,"parent_record_offset":40276,"ancestor_record_offset":39160,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":40160,"parent_record_offset":40132,"ancestor_record_offset":39160,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":39524,"parent_record_offset":39492,"ancestor_record_offset":39160,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":39492,"parent_record_offset":39468,"ancestor_record_offset":39160,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":41808,"parent_record_offset":41784,"ancestor_record_offset":39160,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":423,"record_offset":39564,"parent_record_offset":39524,"ancestor_record_offset":39160,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":40064,"parent_record_offset":39868,"ancestor_record_offset":39160,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":40232,"parent_record_offset":40064,"ancestor_record_offset":39160,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":40364,"parent_record_offset":40064,"ancestor_record_offset":39160,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":40984,"parent_record_offset":40956,"ancestor_record_offset":39160,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":40104,"parent_record_offset":40064,"ancestor_record_offset":39160,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":39440,"parent_record_offset":39412,"ancestor_record_offset":39160,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":423,"record_offset":39868,"parent_record_offset":39840,"ancestor_record_offset":39160,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":423,"record_offset":40920,"parent_record_offset":40864,"ancestor_record_offset":39160,"inlinee":26375,"source_line":710,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":423,"record_offset":40892,"parent_record_offset":40864,"ancestor_record_offset":39160,"inlinee":26374,"source_line":27762,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":39588,"parent_record_offset":39564,"ancestor_record_offset":39160,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":43104,"parent_record_offset":43052,"ancestor_record_offset":39160,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":43668,"parent_record_offset":42724,"ancestor_record_offset":39160,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":42984,"parent_record_offset":42960,"ancestor_record_offset":39160,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":423,"record_offset":39668,"parent_record_offset":39612,"ancestor_record_offset":39160,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":43052,"parent_record_offset":43016,"ancestor_record_offset":39160,"inlinee":11648,"source_line":1164,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":43016,"parent_record_offset":42724,"ancestor_record_offset":39160,"inlinee":11646,"source_line":1259,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":39612,"parent_record_offset":39588,"ancestor_record_offset":39160,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":39640,"parent_record_offset":39612,"ancestor_record_offset":39160,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":44176,"parent_record_offset":44148,"ancestor_record_offset":39160,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":43220,"parent_record_offset":43184,"ancestor_record_offset":39160,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":43420,"parent_record_offset":43392,"ancestor_record_offset":39160,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":43520,"parent_record_offset":43480,"ancestor_record_offset":39160,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":423,"record_offset":43144,"parent_record_offset":43052,"ancestor_record_offset":39160,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":40016,"parent_record_offset":39988,"ancestor_record_offset":39160,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":41684,"parent_record_offset":41656,"ancestor_record_offset":39160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":40668,"parent_record_offset":40460,"ancestor_record_offset":39160,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":423,"record_offset":39988,"parent_record_offset":39960,"ancestor_record_offset":39160,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":41372,"parent_record_offset":41316,"ancestor_record_offset":39160,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":39960,"parent_record_offset":39932,"ancestor_record_offset":39160,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":41344,"parent_record_offset":41316,"ancestor_record_offset":39160,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":41140,"parent_record_offset":40984,"ancestor_record_offset":39160,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":423,"record_offset":41228,"parent_record_offset":41204,"ancestor_record_offset":39160,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":42652,"parent_record_offset":42624,"ancestor_record_offset":39160,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":423,"record_offset":42680,"parent_record_offset":42652,"ancestor_record_offset":39160,"inlinee":11933,"source_line":579,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":39904,"parent_record_offset":39868,"ancestor_record_offset":39160,"inlinee":7436,"source_line":137,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::make_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":39932,"parent_record_offset":39904,"ancestor_record_offset":39160,"inlinee":7437,"source_line":89,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":423,"record_offset":41252,"parent_record_offset":41228,"ancestor_record_offset":39160,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":42896,"parent_record_offset":42724,"ancestor_record_offset":39160,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":43624,"parent_record_offset":43588,"ancestor_record_offset":39160,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":43588,"parent_record_offset":42724,"ancestor_record_offset":39160,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":43392,"parent_record_offset":42724,"ancestor_record_offset":39160,"inlinee":11805,"source_line":1536,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":43184,"parent_record_offset":42724,"ancestor_record_offset":39160,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":43480,"parent_record_offset":42724,"ancestor_record_offset":39160,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::device_address_impl
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":423,"record_offset":40864,"parent_record_offset":40840,"ancestor_record_offset":39160,"inlinee":26372,"source_line":133,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":42960,"parent_record_offset":42724,"ancestor_record_offset":39160,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::stone_foundation_surface_systems::calculate_stone_foundation_surface_strip_bounds
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 671
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67F2
// 首条位置: {"module":423,"inlinee":26610}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":43668,"parent_record_offset":42724,"ancestor_record_offset":42724,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":43016,"parent_record_offset":42724,"ancestor_record_offset":42724,"inlinee":11646,"source_line":1259,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":42896,"parent_record_offset":42724,"ancestor_record_offset":42724,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":43588,"parent_record_offset":42724,"ancestor_record_offset":42724,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":43392,"parent_record_offset":42724,"ancestor_record_offset":42724,"inlinee":11805,"source_line":1536,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":43184,"parent_record_offset":42724,"ancestor_record_offset":42724,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":43480,"parent_record_offset":42724,"ancestor_record_offset":42724,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":423,"record_offset":42960,"parent_record_offset":42724,"ancestor_record_offset":42724,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":43104,"parent_record_offset":43052,"ancestor_record_offset":42724,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":42984,"parent_record_offset":42960,"ancestor_record_offset":42724,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":43052,"parent_record_offset":43016,"ancestor_record_offset":42724,"inlinee":11648,"source_line":1164,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":43220,"parent_record_offset":43184,"ancestor_record_offset":42724,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":43420,"parent_record_offset":43392,"ancestor_record_offset":42724,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":423,"record_offset":43520,"parent_record_offset":43480,"ancestor_record_offset":42724,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":423,"record_offset":43144,"parent_record_offset":43052,"ancestor_record_offset":42724,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":42924,"parent_record_offset":42896,"ancestor_record_offset":42724,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":423,"record_offset":43624,"parent_record_offset":43588,"ancestor_record_offset":42724,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00423.jsonl.gz"}
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
// 完整结构及原始类型记录: stone_foundation_surface_systems.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner>", "constructor": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr", "arguments": ["system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>", "constructor": "core::slice::iter::Windows", "arguments": ["system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>", "system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::filter::Filter<core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0>", "system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["f32", "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stone_foundation_surface_systems.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stone_foundation_surface_systems.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<f32>,ref$<f32>)
// 候选参数类型: ["ref$<f32>", "ref$<f32>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>)
// 候选参数类型: ["ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>", "ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::append_stone_foundation_surface_face
// 信息 ID: 4fb9b7582cd4d8765f48357e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83cedb -> 0x83f420 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83cf2b -> 0x83f420 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83d127 -> 0x83f420 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83d17e -> 0x83f420 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83d363 -> 0x83f420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::calculate_stone_foundation_surface_strip_bounds
// 信息 ID: 09f012d311eaf7e31d008e20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::append_stone_foundation_surface_face; module=423, inline_record=42724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::despawn_stone_foundation_surface
// 信息 ID: d7f3bea0971e5d4021176d37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83aefa -> 0x83a9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83b0b9 -> 0x83a9e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::despawn_stone_foundation_surface::closure$0
// 信息 ID: 395cb1c068036531ef49c890
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::despawn_stone_foundation_surface", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::despawn_stone_foundation_surface; module=423, inline_record=1696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band
// 信息 ID: c485b011e2c6c2030bcf6e42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=21232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure$0
// 信息 ID: c29367b14875547df46b36d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$1> >; module=1455, inline_record=126876; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$1> >; module=1455, inline_record=127692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure$1
// 信息 ID: 7081e22df4945ad0fc576305
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Windows<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfacePoint>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$0>,system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure_env$1> >; module=1455, inline_record=126984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band::closure$2
// 信息 ID: fd54242429619fa0a992a0ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::emit_stone_foundation_surface_band", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=28520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh
// 信息 ID: 5b873761eb2175cab3528a4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=11688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1
// 信息 ID: 024e16638b4f0ea83e243615
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=18316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$1::closure$0
// 信息 ID: e75afe2a586b83ddd166b881
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=18620; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=18972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$2
// 信息 ID: 62433cb821bcaff8cf879b0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=15088; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$3
// 信息 ID: 979ffe257eef4320c8abf7ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3 <- core::slice::sort::stable::driftsort_main<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3>,alloc::vec::Vec<f32,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3 <- core::slice::sort::stable::drift::sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3 <- core::slice::sort::stable::quicksort::quicksort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 26 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; module=1500, inline_record=100012; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; module=1500, inline_record=100124; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; module=1500, inline_record=100300; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; module=1500, inline_record=100424; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; module=1500, inline_record=100660; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; module=1500, inline_record=100972; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; module=1500, inline_record=101084; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure_env$3> >; module=1500, inline_record=101260; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh::closure$4
// 信息 ID: cdfd29c41e50c2a488e5106a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=17200; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=17256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh_and_entities
// 信息 ID: 5ee00ef566fa0c19811ccc77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=8924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh_and_entities::closure$0
// 信息 ID: 623cb4fc3a32be0a1852f38f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::generate_stone_foundation_surface_mesh_and_entities", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=9196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::impl$0::drop
// 信息 ID: e2388f4cf2875c84b30e1fde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<system_wall_constructor::stone_foundation_surface_systems::StoneFoundationSurfaceBufferOwner> @ 0x140af79 -> 0x140a6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::maintain_deferred_stone_foundation_surface_buffer_disposal
// 信息 ID: a27a522708d840d429321f5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=4096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::maintain_deferred_stone_foundation_surface_buffer_disposal::closure$0
// 信息 ID: 48d62fd25168133fb7702898
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::stone_foundation_surface_systems::maintain_deferred_stone_foundation_surface_buffer_disposal", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=4984; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=5152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces
// 信息 ID: df905a40959aa7d6248d8202
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] db5205f46452b2dc2b954a75: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>)> > @ 0x27e360 -> 0x83acb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::RecalculateStoneFoundationSurfaceCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::stone_foundation_surface::DeleteStoneFoundationSurfaceCmd>)> @ 0x67f867 -> 0x83acb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::stone_foundation_surface_systems::stone_foundation_surface_vertex_from_wallspace
// 信息 ID: 8ad363f0c506f212bb0416c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=23452; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=23864; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=24188; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=24560; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=24936; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=26384; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces; module=423, inline_record=26796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
