// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/place_pillars.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 9528f9fc8e5919e1fc4aea073736a7411442464fd889719ad5097e6653535c53
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: place_pillars.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 394, 399, 401, 410, 1455, 1482, 1484
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
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135CF
// 首条位置: {"module":1455,"inlinee":79311}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135CD
// 首条位置: {"module":1455,"inlinee":79309}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1455,"record_offset":36764,"parent_record_offset":36708,"ancestor_record_offset":36708,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1455,"record_offset":38680,"parent_record_offset":36708,"ancestor_record_offset":36708,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":37136,"parent_record_offset":37112,"ancestor_record_offset":36708,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":37160,"parent_record_offset":37136,"ancestor_record_offset":36708,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":38864,"parent_record_offset":38840,"ancestor_record_offset":36708,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":38888,"parent_record_offset":38864,"ancestor_record_offset":36708,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":37184,"parent_record_offset":37160,"ancestor_record_offset":36708,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":38912,"parent_record_offset":38888,"ancestor_record_offset":36708,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":37112,"parent_record_offset":36964,"ancestor_record_offset":36708,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1455,"record_offset":38840,"parent_record_offset":38752,"ancestor_record_offset":36708,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":36904,"parent_record_offset":36876,"ancestor_record_offset":36708,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":38780,"parent_record_offset":38752,"ancestor_record_offset":36708,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":38752,"parent_record_offset":38728,"ancestor_record_offset":36708,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":36964,"parent_record_offset":36932,"ancestor_record_offset":36708,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":36932,"parent_record_offset":36904,"ancestor_record_offset":36708,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":38728,"parent_record_offset":38704,"ancestor_record_offset":36708,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1455,"record_offset":37000,"parent_record_offset":36964,"ancestor_record_offset":36708,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1455,"record_offset":36796,"parent_record_offset":36764,"ancestor_record_offset":36708,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1455,"record_offset":36828,"parent_record_offset":36796,"ancestor_record_offset":36708,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1455,"record_offset":37580,"parent_record_offset":37516,"ancestor_record_offset":36708,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1455,"record_offset":37552,"parent_record_offset":37516,"ancestor_record_offset":36708,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1455,"record_offset":36876,"parent_record_offset":36828,"ancestor_record_offset":36708,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1455,"record_offset":37516,"parent_record_offset":37484,"ancestor_record_offset":36708,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1455,"record_offset":37024,"parent_record_offset":37000,"ancestor_record_offset":36708,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1455,"record_offset":37072,"parent_record_offset":37048,"ancestor_record_offset":36708,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1455,"record_offset":37276,"parent_record_offset":37240,"ancestor_record_offset":36708,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":1455,"record_offset":37240,"parent_record_offset":36828,"ancestor_record_offset":36708,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1455,"record_offset":37436,"parent_record_offset":36828,"ancestor_record_offset":36708,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1455,"record_offset":37048,"parent_record_offset":37024,"ancestor_record_offset":36708,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1455,"record_offset":37816,"parent_record_offset":37788,"ancestor_record_offset":36708,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1455,"record_offset":38808,"parent_record_offset":38780,"ancestor_record_offset":36708,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1455,"record_offset":38012,"parent_record_offset":37580,"ancestor_record_offset":36708,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1455,"record_offset":38036,"parent_record_offset":38012,"ancestor_record_offset":36708,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1455,"record_offset":38704,"parent_record_offset":38680,"ancestor_record_offset":36708,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1455,"record_offset":37364,"parent_record_offset":37304,"ancestor_record_offset":36708,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1455,"record_offset":37336,"parent_record_offset":37304,"ancestor_record_offset":36708,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1455,"record_offset":37304,"parent_record_offset":37276,"ancestor_record_offset":36708,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::place_pillars::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1455,"record_offset":37404,"parent_record_offset":36828,"ancestor_record_offset":36708,"inlinee":79311,"source_line":43,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::PillarProposals::add_proposal
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65F8
// 首条位置: {"module":394,"inlinee":26104}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":23432,"parent_record_offset":23360,"ancestor_record_offset":23360,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":394,"record_offset":23836,"parent_record_offset":23360,"ancestor_record_offset":23360,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":394,"record_offset":23796,"parent_record_offset":23360,"ancestor_record_offset":23360,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":394,"record_offset":23676,"parent_record_offset":23360,"ancestor_record_offset":23360,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":394,"record_offset":23648,"parent_record_offset":23360,"ancestor_record_offset":23360,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":394,"record_offset":23768,"parent_record_offset":23360,"ancestor_record_offset":23360,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":394,"record_offset":23524,"parent_record_offset":23500,"ancestor_record_offset":23360,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":394,"record_offset":23572,"parent_record_offset":23548,"ancestor_record_offset":23360,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":394,"record_offset":23548,"parent_record_offset":23524,"ancestor_record_offset":23360,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":23500,"parent_record_offset":23460,"ancestor_record_offset":23360,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":23460,"parent_record_offset":23432,"ancestor_record_offset":23360,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":394,"record_offset":23732,"parent_record_offset":23708,"ancestor_record_offset":23360,"inlinee":17499,"source_line":1878,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":394,"record_offset":23708,"parent_record_offset":23676,"ancestor_record_offset":23360,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":394,"record_offset":23612,"parent_record_offset":23460,"ancestor_record_offset":23360,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::PillarProposals::empty
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6653
// 首条位置: {"module":399,"inlinee":26195}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::place_pillars
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":401,"record_offset":280,"rva":8383616,"va":5377092736}
// 编译实例 RVAs: 0x7fec80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-1418967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e82959eaff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377097666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008f730
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7803725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8b3ec88ff","size":5,"soft_fail":false,"target_va":5369296688,"va":5377100408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7554893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8b3b88cff","size":5,"soft_fail":false,"target_va":5369542000,"va":5377096888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ee920
//     owner=unresolved_namespace; count=2
//     target_aliases=248 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7408885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e80bf38eff","size":5,"soft_fail":false,"target_va":5369686304,"va":5377095184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3615255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8e9d5c8ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377094290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2998820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8dc3dd2ff","size":5,"soft_fail":false,"target_va":5374098704,"va":5377097519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1024576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e840a20f00","size":5,"soft_fail":false,"target_va":5378120480,"va":5377095899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7ede0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2615755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8cbe92700","size":5,"soft_fail":false,"target_va":5379714528,"va":5377098768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbeb30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8125410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8e2fb7b00","size":5,"soft_fail":false,"target_va":5385218864,"va":5377093449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f11b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12523693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8ad18bf00","size":5,"soft_fail":false,"target_va":5389619632,"va":5377095934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142a760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12759053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e80db0c200","size":5,"soft_fail":false,"target_va":5389854560,"va":5377095502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142b100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12761562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8dab9c200","size":5,"soft_fail":false,"target_va":5389857024,"va":5377095457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142e960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12775044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e884eec200","size":5,"soft_fail":false,"target_va":5389871456,"va":5377096407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142ebb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12775104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8c0eec200","size":5,"soft_fail":false,"target_va":5389872048,"va":5377096939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39259418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e81a0d5702","size":5,"soft_fail":false,"target_va":5416353040,"va":5377093617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39284413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8bd6e5702","size":5,"soft_fail":false,"target_va":5416380224,"va":5377095806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39523120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e830135b02","size":5,"soft_fail":false,"target_va":5416621408,"va":5377098283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39523668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e854155b02","size":5,"soft_fail":false,"target_va":5416621536,"va":5377097863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t39523714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e882155b02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377097881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t39523421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e85d145b02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377098336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39524113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e811175b02","size":5,"soft_fail":false,"target_va":5416622560,"va":5377098442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39524202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e86a175b02","size":5,"soft_fail":false,"target_va":5416622624,"va":5377098417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39524191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e85f175b02","size":5,"soft_fail":false,"target_va":5416622240,"va":5377098044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8b91e0000","size":5,"soft_fail":false,"target_va":5377100768,"va":5377092898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39522622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e83e115b02","size":5,"soft_fail":false,"target_va":5416621376,"va":5377098749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_arch_pillars_count
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2160344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8d8f62000","size":5,"soft_fail":false,"target_va":5379257664,"va":5377097315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_lintel_pillars_count
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2160401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e811f72000","size":5,"soft_fail":false,"target_va":5379257696,"va":5377097290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2160035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8a3f52000","size":5,"soft_fail":false,"target_va":5379257408,"va":5377097368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39479802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8fa695a02","size":5,"soft_fail":false,"target_va":5416576832,"va":5377097025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39479673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e879695a02","size":5,"soft_fail":false,"target_va":5416576144,"va":5377096466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6888812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e86c1d6900","size":5,"soft_fail":false,"target_va":5383984848,"va":5377096031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6888921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8d91d6900","size":5,"soft_fail":false,"target_va":5383985360,"va":5377096434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6870553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e819d66800","size":5,"soft_fail":false,"target_va":5383963792,"va":5377093234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6879060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e854f76800","size":5,"soft_fail":false,"target_va":5383972224,"va":5377093159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6867177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8e9c86800","size":5,"soft_fail":false,"target_va":5383964912,"va":5377097730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6869939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8b3d36800","size":5,"soft_fail":false,"target_va":5383962816,"va":5377092872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39598710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8763a5c02","size":5,"soft_fail":false,"target_va":5416697008,"va":5377098293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39521041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8110b5b02","size":5,"soft_fail":false,"target_va":5416619360,"va":5377098314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12606499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8235cc000","size":5,"soft_fail":false,"target_va":5389701712,"va":5377095208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12606407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e8c75bc000","size":5,"soft_fail":false,"target_va":5389701728,"va":5377095316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12606587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377092736,"raw_hex":"e87b5cc000","size":5,"soft_fail":false,"target_va":5389701744,"va":5377095152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377092736,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5377095967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":16000,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11483,"source_line":3637,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":10232,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":11896,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":10856,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":4296,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":401,"record_offset":2764,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":14572,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":401,"record_offset":10344,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":3532,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":3476,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":401,"record_offset":10464,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":401,"record_offset":12168,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":401,"record_offset":12664,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":401,"record_offset":8156,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":401,"record_offset":7564,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26084,"source_line":3133,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":8240,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":401,"record_offset":12108,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":401,"record_offset":12204,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11663,"source_line":196,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":401,"record_offset":11076,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26107,"source_line":149,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":401,"record_offset":12636,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":401,"record_offset":14488,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":1956,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":552,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":944,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":401,"record_offset":9292,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":401,"record_offset":8304,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11661,"source_line":1475,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_for_point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":11132,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26183,"source_line":1305,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":11328,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":11300,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":11244,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::start
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":11356,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26198,"source_line":333,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":401,"record_offset":11384,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":401,"record_offset":11048,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26122,"source_line":175,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":401,"record_offset":11104,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26182,"source_line":199,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":401,"record_offset":14956,"parent_record_offset":14932,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":401,"record_offset":14980,"parent_record_offset":14956,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":401,"record_offset":15004,"parent_record_offset":14980,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":401,"record_offset":14932,"parent_record_offset":14840,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":401,"record_offset":5052,"parent_record_offset":5020,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":401,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":280,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":401,"record_offset":14868,"parent_record_offset":14840,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":401,"record_offset":14840,"parent_record_offset":14816,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":401,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":401,"record_offset":5076,"parent_record_offset":5052,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":401,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":280,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":401,"record_offset":14816,"parent_record_offset":14792,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":5020,"parent_record_offset":4344,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":16032,"parent_record_offset":16000,"ancestor_record_offset":280,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":4344,"parent_record_offset":4320,"ancestor_record_offset":280,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":5536,"parent_record_offset":4344,"ancestor_record_offset":280,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":11920,"parent_record_offset":11896,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":4936,"parent_record_offset":4344,"ancestor_record_offset":280,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":5140,"parent_record_offset":4344,"ancestor_record_offset":280,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":280,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$6::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":16164,"parent_record_offset":16032,"ancestor_record_offset":280,"inlinee":10784,"source_line":3638,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":401,"record_offset":3080,"parent_record_offset":2852,"ancestor_record_offset":280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":401,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":280,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":401,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":280,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":401,"record_offset":5192,"parent_record_offset":5168,"ancestor_record_offset":280,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$0::size_hint
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":401,"record_offset":4420,"parent_record_offset":4392,"ancestor_record_offset":280,"inlinee":26082,"source_line":18,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":401,"record_offset":5216,"parent_record_offset":5192,"ancestor_record_offset":280,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::size_hint
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":401,"record_offset":4448,"parent_record_offset":4420,"ancestor_record_offset":280,"inlinee":26083,"source_line":103,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":401,"record_offset":2828,"parent_record_offset":2788,"ancestor_record_offset":280,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":401,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":280,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":401,"record_offset":2284,"parent_record_offset":2256,"ancestor_record_offset":280,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":401,"record_offset":2312,"parent_record_offset":2284,"ancestor_record_offset":280,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":401,"record_offset":2232,"parent_record_offset":2204,"ancestor_record_offset":280,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":401,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":280,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":401,"record_offset":2372,"parent_record_offset":2256,"ancestor_record_offset":280,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":401,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":280,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":10560,"parent_record_offset":10536,"ancestor_record_offset":280,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":280,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":16188,"parent_record_offset":16164,"ancestor_record_offset":280,"inlinee":10786,"source_line":2197,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":280,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":401,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":280,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":401,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":280,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":401,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":280,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":401,"record_offset":7640,"parent_record_offset":7600,"ancestor_record_offset":280,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":280,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":5244,"parent_record_offset":5216,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":401,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":280,"inlinee":10731,"source_line":182,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":280,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":5168,"parent_record_offset":4344,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":401,"record_offset":4392,"parent_record_offset":4344,"ancestor_record_offset":280,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":280,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":280,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":280,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":401,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":280,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":401,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":401,"record_offset":3036,"parent_record_offset":3008,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":401,"record_offset":4864,"parent_record_offset":4476,"ancestor_record_offset":280,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":401,"record_offset":2896,"parent_record_offset":2852,"ancestor_record_offset":280,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":401,"record_offset":14900,"parent_record_offset":14868,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":401,"record_offset":11480,"parent_record_offset":11432,"ancestor_record_offset":280,"inlinee":26188,"source_line":3227,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":401,"record_offset":8848,"parent_record_offset":8820,"ancestor_record_offset":280,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":401,"record_offset":8820,"parent_record_offset":8792,"ancestor_record_offset":280,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":11832,"parent_record_offset":11800,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":11740,"parent_record_offset":11708,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":11648,"parent_record_offset":11620,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":401,"record_offset":7916,"parent_record_offset":7792,"ancestor_record_offset":280,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":401,"record_offset":2104,"parent_record_offset":2064,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":401,"record_offset":5500,"parent_record_offset":5476,"ancestor_record_offset":280,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":401,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":280,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":401,"record_offset":12468,"parent_record_offset":12204,"ancestor_record_offset":280,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":401,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":280,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":401,"record_offset":5476,"parent_record_offset":5168,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":401,"record_offset":5324,"parent_record_offset":5244,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":401,"record_offset":11204,"parent_record_offset":11132,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":401,"record_offset":12500,"parent_record_offset":12204,"ancestor_record_offset":280,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":401,"record_offset":892,"parent_record_offset":864,"ancestor_record_offset":280,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":401,"record_offset":2720,"parent_record_offset":2064,"ancestor_record_offset":280,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":401,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":280,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":401,"record_offset":7880,"parent_record_offset":7856,"ancestor_record_offset":280,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":401,"record_offset":2436,"parent_record_offset":2204,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":401,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":401,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":280,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":401,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":280,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":280,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":280,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":280,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":401,"record_offset":1548,"parent_record_offset":1520,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":401,"record_offset":12072,"parent_record_offset":11920,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":401,"record_offset":1648,"parent_record_offset":1620,"ancestor_record_offset":280,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":401,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":280,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":401,"record_offset":2952,"parent_record_offset":2852,"ancestor_record_offset":280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":401,"record_offset":3320,"parent_record_offset":2852,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":401,"record_offset":12440,"parent_record_offset":12204,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":10884,"parent_record_offset":10856,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":3008,"parent_record_offset":2980,"ancestor_record_offset":280,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":401,"record_offset":2980,"parent_record_offset":2952,"ancestor_record_offset":280,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":401,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":280,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":280,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":280,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":5400,"parent_record_offset":5348,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":4528,"parent_record_offset":4476,"ancestor_record_offset":280,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":401,"record_offset":1268,"parent_record_offset":1152,"ancestor_record_offset":280,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":401,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":280,"inlinee":4734,"source_line":302,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insert_tail
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":401,"record_offset":7792,"parent_record_offset":7696,"ancestor_record_offset":280,"inlinee":9485,"source_line":542,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":401,"record_offset":7696,"parent_record_offset":7600,"ancestor_record_offset":280,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":401,"record_offset":7600,"parent_record_offset":7564,"ancestor_record_offset":280,"inlinee":11895,"source_line":22,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":401,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":280,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":401,"record_offset":980,"parent_record_offset":944,"ancestor_record_offset":280,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":401,"record_offset":1120,"parent_record_offset":1088,"ancestor_record_offset":280,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":401,"record_offset":1088,"parent_record_offset":1052,"ancestor_record_offset":280,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":401,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":280,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":401,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":280,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":401,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":280,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":401,"record_offset":1592,"parent_record_offset":1052,"ancestor_record_offset":280,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":656,"parent_record_offset":624,"ancestor_record_offset":280,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":280,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":784,"parent_record_offset":752,"ancestor_record_offset":280,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":840,"parent_record_offset":784,"ancestor_record_offset":280,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":752,"parent_record_offset":720,"ancestor_record_offset":280,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":596,"parent_record_offset":552,"ancestor_record_offset":280,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":401,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":280,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":401,"record_offset":1152,"parent_record_offset":1120,"ancestor_record_offset":280,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":401,"record_offset":1436,"parent_record_offset":1152,"ancestor_record_offset":280,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":401,"record_offset":1408,"parent_record_offset":1152,"ancestor_record_offset":280,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":401,"record_offset":1212,"parent_record_offset":1152,"ancestor_record_offset":280,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":401,"record_offset":12772,"parent_record_offset":12712,"ancestor_record_offset":280,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":401,"record_offset":12352,"parent_record_offset":12316,"ancestor_record_offset":280,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":401,"record_offset":14512,"parent_record_offset":14488,"ancestor_record_offset":280,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":401,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":280,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":401,"record_offset":12376,"parent_record_offset":12352,"ancestor_record_offset":280,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":401,"record_offset":14536,"parent_record_offset":14512,"ancestor_record_offset":280,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":401,"record_offset":12132,"parent_record_offset":12108,"ancestor_record_offset":280,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":401,"record_offset":12532,"parent_record_offset":12500,"ancestor_record_offset":280,"inlinee":11665,"source_line":213,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":401,"record_offset":12688,"parent_record_offset":12664,"ancestor_record_offset":280,"inlinee":4972,"source_line":598,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":401,"record_offset":9072,"parent_record_offset":9048,"ancestor_record_offset":280,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":401,"record_offset":11708,"parent_record_offset":11384,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":401,"record_offset":11800,"parent_record_offset":11384,"ancestor_record_offset":280,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":401,"record_offset":11620,"parent_record_offset":11384,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":401,"record_offset":8760,"parent_record_offset":8732,"ancestor_record_offset":280,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":401,"record_offset":8792,"parent_record_offset":8760,"ancestor_record_offset":280,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":401,"record_offset":8732,"parent_record_offset":8480,"ancestor_record_offset":280,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":401,"record_offset":8592,"parent_record_offset":8480,"ancestor_record_offset":280,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":401,"record_offset":9144,"parent_record_offset":8480,"ancestor_record_offset":280,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":401,"record_offset":8676,"parent_record_offset":8480,"ancestor_record_offset":280,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":401,"record_offset":8536,"parent_record_offset":8480,"ancestor_record_offset":280,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":401,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":280,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":401,"record_offset":8372,"parent_record_offset":8336,"ancestor_record_offset":280,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":401,"record_offset":9000,"parent_record_offset":8892,"ancestor_record_offset":280,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":401,"record_offset":8404,"parent_record_offset":8372,"ancestor_record_offset":280,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":401,"record_offset":8940,"parent_record_offset":8916,"ancestor_record_offset":280,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":401,"record_offset":9236,"parent_record_offset":8480,"ancestor_record_offset":280,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":401,"record_offset":8916,"parent_record_offset":8892,"ancestor_record_offset":280,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":401,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":280,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":401,"record_offset":8432,"parent_record_offset":8372,"ancestor_record_offset":280,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":401,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":280,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":401,"record_offset":8564,"parent_record_offset":8480,"ancestor_record_offset":280,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":401,"record_offset":8892,"parent_record_offset":8480,"ancestor_record_offset":280,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":401,"record_offset":12316,"parent_record_offset":12204,"ancestor_record_offset":280,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":401,"record_offset":12712,"parent_record_offset":12688,"ancestor_record_offset":280,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":401,"record_offset":12408,"parent_record_offset":12316,"ancestor_record_offset":280,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":401,"record_offset":12744,"parent_record_offset":12712,"ancestor_record_offset":280,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":1992,"parent_record_offset":1956,"ancestor_record_offset":280,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":15124,"parent_record_offset":15092,"ancestor_record_offset":280,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":15580,"parent_record_offset":15376,"ancestor_record_offset":280,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":401,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":280,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":2204,"parent_record_offset":2064,"ancestor_record_offset":280,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":401,"record_offset":2408,"parent_record_offset":2204,"ancestor_record_offset":280,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":401,"record_offset":8336,"parent_record_offset":8304,"ancestor_record_offset":280,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":401,"record_offset":2160,"parent_record_offset":2132,"ancestor_record_offset":280,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":401,"record_offset":2064,"parent_record_offset":2028,"ancestor_record_offset":280,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":401,"record_offset":2028,"parent_record_offset":1992,"ancestor_record_offset":280,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":401,"record_offset":11268,"parent_record_offset":11244,"ancestor_record_offset":280,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":401,"record_offset":11432,"parent_record_offset":11384,"ancestor_record_offset":280,"inlinee":26187,"source_line":226,"archive":"_pdb/C/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::place_pillars::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13734
// 首条位置: {"module":1484,"inlinee":79668}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::place_pillars::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1484,"record_offset":280,"rva":21161600,"va":5389870720}
// 编译实例 RVAs: 0x142e680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11743925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389870720,"raw_hex":"e84bcd4cff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389870864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11368283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389870720,"raw_hex":"e8a58852ff","size":5,"soft_fail":false,"target_va":5378502608,"va":5389870886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6545806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389870720,"raw_hex":"e8721e9cff","size":5,"soft_fail":false,"target_va":5383325024,"va":5389870825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-175373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389870720,"raw_hex":"e8f352fdff","size":5,"soft_fail":false,"target_va":5389695968,"va":5389871336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11389650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389870720,"raw_hex":"e82e3552ff","size":5,"soft_fail":false,"target_va":5378481680,"va":5389871325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1484,"record_offset":544,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":1484,"record_offset":452,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79663,"source_line":158,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::impl$16::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":1484,"record_offset":612,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9822,"source_line":843,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":572,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":1356,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] rhapsody_core::shader::math::Affine3Packed::from_mat3_translation
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-core/src/lib.rs
//     evidence={"module":1484,"record_offset":512,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79664,"source_line":32,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1484,"record_offset":484,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1484,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":832,"parent_record_offset":772,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":1484,"record_offset":680,"parent_record_offset":652,"ancestor_record_offset":280,"inlinee":9826,"source_line":625,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":1484,"record_offset":652,"parent_record_offset":612,"ancestor_record_offset":280,"inlinee":9824,"source_line":893,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":1440,"parent_record_offset":1408,"ancestor_record_offset":280,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":1408,"parent_record_offset":1380,"ancestor_record_offset":280,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":280,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":716,"parent_record_offset":680,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":1012,"parent_record_offset":984,"ancestor_record_offset":280,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1484,"record_offset":1472,"parent_record_offset":1440,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::emit_pillar_at_point<system_wall_constructor::place_pillars::place_pillars::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1484,"record_offset":1604,"rva":21162336,"va":5389871456}
// 编译实例 RVAs: 0x142e960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11744927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389871456,"raw_hex":"e861c94cff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389871866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c6a170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8145310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389871456,"raw_hex":"e862b683ff","size":5,"soft_fail":false,"target_va":5381726576,"va":5389871881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6546613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389871456,"raw_hex":"e84b1b9cff","size":5,"soft_fail":false,"target_va":5383325024,"va":5389871632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-177944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389871456,"raw_hex":"e8e848fdff","size":5,"soft_fail":false,"target_va":5389694032,"va":5389871971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6463608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389871456,"raw_hex":"e8885f9dff","size":5,"soft_fail":false,"target_va":5383408256,"va":5389871859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1484,"record_offset":1952,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1484,"record_offset":2096,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":79470,"source_line":149,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":1484,"record_offset":2268,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":79474,"source_line":565,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1484,"record_offset":2036,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":1844,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":1896,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::place_pillars::place_pillars::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1484,"record_offset":3580,"parent_record_offset":1604,"ancestor_record_offset":1604,"inlinee":79668,"source_line":181,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1484,"record_offset":2584,"parent_record_offset":2488,"ancestor_record_offset":1604,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1484,"record_offset":2456,"parent_record_offset":2392,"ancestor_record_offset":1604,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1484,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1604,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":1604,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":1604,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1484,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":1604,"inlinee":79471,"source_line":160,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1484,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":1604,"inlinee":79479,"source_line":707,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1484,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":1604,"inlinee":79472,"source_line":78,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$16::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1484,"record_offset":2300,"parent_record_offset":2268,"ancestor_record_offset":1604,"inlinee":79476,"source_line":918,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$22::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":1484,"record_offset":2336,"parent_record_offset":2300,"ancestor_record_offset":1604,"inlinee":79478,"source_line":968,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1484,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1604,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1484,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":1604,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$19::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1484,"record_offset":2812,"parent_record_offset":2780,"ancestor_record_offset":1604,"inlinee":79481,"source_line":1453,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1484,"record_offset":3284,"parent_record_offset":3120,"ancestor_record_offset":1604,"inlinee":27159,"source_line":1577,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::xxx
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1484,"record_offset":2392,"parent_record_offset":2360,"ancestor_record_offset":1604,"inlinee":79665,"source_line":120,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1484,"record_offset":2488,"parent_record_offset":2360,"ancestor_record_offset":1604,"inlinee":79483,"source_line":180,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::zzz
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":1484,"record_offset":2616,"parent_record_offset":2360,"ancestor_record_offset":1604,"inlinee":79666,"source_line":245,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>
// 记录形式: compiled_procedure; instances=1
// 源码行: 330
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1484,"record_offset":3612,"rva":21162928,"va":5389872048}
// 编译实例 RVAs: 0x142ebb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-14193543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389872048,"raw_hex":"e8796c27ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5389872226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-14194294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389872048,"raw_hex":"e88a6927ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389872993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21150009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389872048,"raw_hex":"e8c746bdfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5389872196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27185169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389872048,"raw_hex":"e811d09e01","size":5,"soft_fail":false,"target_va":5417057664,"va":5389872490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26747131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389872048,"raw_hex":"e8fb209801","size":5,"soft_fail":false,"target_va":5416620769,"va":5389873633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26493060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389872048,"raw_hex":"e884409401","size":5,"soft_fail":false,"target_va":5416366080,"va":5389873015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26747123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389872048,"raw_hex":"e8f3209801","size":5,"soft_fail":false,"target_va":5416620794,"va":5389873666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6490130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389872048,"raw_hex":"e8eef79cff","size":5,"soft_fail":false,"target_va":5383383152,"va":5389873277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::place_pillars::place_pillars::closure$4
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389872048,"raw_hex":"e89ff5ffff","size":5,"soft_fail":false,"target_va":5389870720,"va":5389873372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6598529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389872048,"raw_hex":"e87f509bff","size":5,"soft_fail":false,"target_va":5383274080,"va":5389872604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1484,"record_offset":3920,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":5976,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1484,"record_offset":4504,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1484,"record_offset":4100,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1484,"record_offset":6264,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1484,"record_offset":4532,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1484,"record_offset":5688,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1484,"record_offset":4308,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":4392,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":7108,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":6660,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":3888,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":7036,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":7236,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1484,"record_offset":7324,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1484,"record_offset":4448,"parent_record_offset":3612,"ancestor_record_offset":3612,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":3612,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":4028,"parent_record_offset":4004,"ancestor_record_offset":3612,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":5872,"parent_record_offset":5848,"ancestor_record_offset":3612,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":3612,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":3612,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":3612,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":3980,"parent_record_offset":3948,"ancestor_record_offset":3612,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1484,"record_offset":5848,"parent_record_offset":5760,"ancestor_record_offset":3612,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1484,"record_offset":3948,"parent_record_offset":3920,"ancestor_record_offset":3612,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1484,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":3612,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1484,"record_offset":4736,"parent_record_offset":4704,"ancestor_record_offset":3612,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1484,"record_offset":5788,"parent_record_offset":5760,"ancestor_record_offset":3612,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1484,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":3612,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1484,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":3612,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1484,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":3612,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1484,"record_offset":4804,"parent_record_offset":4768,"ancestor_record_offset":3612,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1484,"record_offset":4768,"parent_record_offset":4736,"ancestor_record_offset":3612,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1484,"record_offset":5736,"parent_record_offset":5712,"ancestor_record_offset":3612,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":6032,"parent_record_offset":6000,"ancestor_record_offset":3612,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":3612,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":6000,"parent_record_offset":5976,"ancestor_record_offset":3612,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":4672,"parent_record_offset":4628,"ancestor_record_offset":3612,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":4704,"parent_record_offset":4672,"ancestor_record_offset":3612,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":4564,"parent_record_offset":4532,"ancestor_record_offset":3612,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1484,"record_offset":5476,"parent_record_offset":5432,"ancestor_record_offset":3612,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1484,"record_offset":5580,"parent_record_offset":5476,"ancestor_record_offset":3612,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1484,"record_offset":5112,"parent_record_offset":4628,"ancestor_record_offset":3612,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1484,"record_offset":4596,"parent_record_offset":4564,"ancestor_record_offset":3612,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1484,"record_offset":4628,"parent_record_offset":4596,"ancestor_record_offset":3612,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1484,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":3612,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1484,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":3612,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1484,"record_offset":6448,"parent_record_offset":6424,"ancestor_record_offset":3612,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1484,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":3612,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1484,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":3612,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1484,"record_offset":4972,"parent_record_offset":4628,"ancestor_record_offset":3612,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1484,"record_offset":5256,"parent_record_offset":5208,"ancestor_record_offset":3612,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1484,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":3612,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1484,"record_offset":5432,"parent_record_offset":5256,"ancestor_record_offset":3612,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1484,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":3612,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1484,"record_offset":5816,"parent_record_offset":5788,"ancestor_record_offset":3612,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":6744,"parent_record_offset":6716,"ancestor_record_offset":3612,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":5376,"parent_record_offset":5332,"ancestor_record_offset":3612,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1484,"record_offset":5332,"parent_record_offset":5288,"ancestor_record_offset":3612,"inlinee":26853,"source_line":27,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":7264,"parent_record_offset":7236,"ancestor_record_offset":3612,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":7068,"parent_record_offset":7036,"ancestor_record_offset":3612,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1484,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":3612,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1484,"record_offset":5068,"parent_record_offset":5044,"ancestor_record_offset":3612,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1484,"record_offset":5712,"parent_record_offset":5688,"ancestor_record_offset":3612,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1484,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":3612,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1484,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":3612,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1484,"record_offset":5624,"parent_record_offset":5208,"ancestor_record_offset":3612,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1484,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":3612,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1484,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":3612,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1484,"record_offset":5532,"parent_record_offset":5476,"ancestor_record_offset":3612,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1484,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":3612,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":5208,"parent_record_offset":5184,"ancestor_record_offset":3612,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":6332,"parent_record_offset":6296,"ancestor_record_offset":3612,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1484,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":3612,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1484,"record_offset":7136,"parent_record_offset":7108,"ancestor_record_offset":3612,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1484,"record_offset":6424,"parent_record_offset":6296,"ancestor_record_offset":3612,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1484,"record_offset":6296,"parent_record_offset":6264,"ancestor_record_offset":3612,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1484,"record_offset":7172,"parent_record_offset":7136,"ancestor_record_offset":3612,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::place_pillars::assemble_stone_pillar::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1484,"record_offset":5288,"parent_record_offset":5256,"ancestor_record_offset":3612,"inlinee":79363,"source_line":397,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::assemble_stone_pillar::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 397
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13603
// 首条位置: {"module":1455,"inlinee":79363}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1484,"record_offset":5332,"parent_record_offset":5288,"ancestor_record_offset":5288,"inlinee":26853,"source_line":27,"archive":"_pdb/C/modules/01484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1484,"record_offset":5376,"parent_record_offset":5332,"ancestor_record_offset":5288,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01484.jsonl.gz"}
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
// 完整结构及原始类型记录: place_pillars.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::place_pillars::PillarPlacementProposal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::place_pillars::PillarPlacementProposal,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::place_pillars::PillarPlacementProposal", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::place_pillars::PillarProposals
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: place_pillars.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: place_pillars.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::PillarProposals::add_proposal
// 信息 ID: 776b102f3720bf47ce3b6e4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=394, inline_record=23360; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=394, inline_record=25236; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=410, inline_record=14172; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=410, inline_record=15956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::PillarProposals::empty
// 信息 ID: 1d9e736871b54ebcc2b736d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=4716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::assemble_stone_pillar::closure$0
// 信息 ID: 2e8d3555db29a67ebab1cb70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::place_pillars::assemble_stone_pillar", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>; module=1484, inline_record=5288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>
// 信息 ID: 3ee84d0d4999bfaa293c55dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::place_pillars::assemble_stone_pillar -> ["system_wall_constructor::place_pillars::place_pillars::closure_env$4"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ffceb -> 0x142ebb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::emit_pillar_at_point<system_wall_constructor::place_pillars::place_pillars::closure_env$3>
// 信息 ID: ff0f8ceaa5f92404c9f4ffbe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::place_pillars::emit_pillar_at_point -> ["system_wall_constructor::place_pillars::place_pillars::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ffad7 -> 0x142e960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::impl$1::clone
// 信息 ID: 1116fd779feedcf5b48e8a1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>; module=1455, inline_record=37404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::impl$2::clone
// 信息 ID: d1a061c4b2705821644705b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>; module=1455, inline_record=36708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::place_pillars
// 信息 ID: 1a9fbb3f8d9251a3e0527e8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e2b7dda4432a668770d3e61e: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)> > @ 0x29296b -> 0x7fec80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)> @ 0x6776fe -> 0x7fec80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::place_pillars::closure$3
// 信息 ID: 1f9650864528619994db7de5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::place_pillars::place_pillars", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::place_pillars::place_pillars::closure_env$3 <- system_wall_constructor::place_pillars::emit_pillar_at_point<system_wall_constructor::place_pillars::place_pillars::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::emit_pillar_at_point<system_wall_constructor::place_pillars::place_pillars::closure_env$3>; module=1484, inline_record=3580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::place_pillars::closure$4
// 信息 ID: dbbead5e6fe016b679e20ca6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::place_pillars::place_pillars", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::place_pillars::place_pillars::closure_env$4 <- system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4> @ 0x142f0dc -> 0x142e680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4> @ 0x142f161 -> 0x142e680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
