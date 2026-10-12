// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/critters/src/boids.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: de05d906ae2d39abd3da2c26bd0aa936cb144970453e6aeb2d8360d1d067f4e5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: boids.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 344, 2858, 4165, 4178, 4840, 4841
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lazy_static; // namespace candidate
//   package identities: [{"name":"lazy_static","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_critters; // namespace candidate
//   package identities: [{"name":"lazy_static","source":"PDB_registry_path_only","version":"1.4.0"},{"name":"system-critters","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::boids::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B328
// 首条位置: {"module":2858,"inlinee":111400}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$102::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4165,"record_offset":5188,"parent_record_offset":5156,"ancestor_record_offset":5156,"inlinee":9872,"source_line":20,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::impl$6::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x546F
// 首条位置: {"module":344,"inlinee":21615}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::push
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x214F2
// 首条位置: {"module":4165,"inlinee":136434}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4165,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":3976,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4165,"record_offset":4096,"parent_record_offset":4064,"ancestor_record_offset":3976,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4165,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":3976,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4165,"record_offset":4120,"parent_record_offset":4096,"ancestor_record_offset":3976,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4165,"record_offset":4064,"parent_record_offset":4028,"ancestor_record_offset":3976,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4165,"record_offset":4028,"parent_record_offset":4004,"ancestor_record_offset":3976,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4165,"record_offset":4184,"parent_record_offset":4028,"ancestor_record_offset":3976,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::drain
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21542
// 首条位置: {"module":4178,"inlinee":136514}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4178,"record_offset":3684,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4178,"record_offset":3712,"parent_record_offset":3656,"ancestor_record_offset":3656,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4178,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":3656,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4178,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":3656,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4178,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":3656,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4178,"record_offset":3748,"parent_record_offset":3712,"ancestor_record_offset":3656,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4178,"record_offset":3860,"parent_record_offset":3712,"ancestor_record_offset":3656,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4178,"record_offset":3888,"parent_record_offset":3712,"ancestor_record_offset":3656,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4178,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3656,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4178,"record_offset":3916,"parent_record_offset":3712,"ancestor_record_offset":3656,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4178,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":3656,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::len
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x214F5
// 首条位置: {"module":4165,"inlinee":136437}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::clone_state
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x214F3
// 首条位置: {"module":4165,"inlinee":136435}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4165,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":4444,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4165,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":4444,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4165,"record_offset":4908,"parent_record_offset":4884,"ancestor_record_offset":4444,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4165,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":4444,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4165,"record_offset":4860,"parent_record_offset":4676,"ancestor_record_offset":4444,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4178,"record_offset":14612,"parent_record_offset":14588,"ancestor_record_offset":14436,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4165,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":4444,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4178,"record_offset":14660,"parent_record_offset":14636,"ancestor_record_offset":14436,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4178,"record_offset":14636,"parent_record_offset":14612,"ancestor_record_offset":14436,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4165,"record_offset":4676,"parent_record_offset":4644,"ancestor_record_offset":4444,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4165,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":4444,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4165,"record_offset":4716,"parent_record_offset":4676,"ancestor_record_offset":4444,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4165,"record_offset":4552,"parent_record_offset":4492,"ancestor_record_offset":4444,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4178,"record_offset":14588,"parent_record_offset":14564,"ancestor_record_offset":14436,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04178.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4165,"record_offset":4524,"parent_record_offset":4492,"ancestor_record_offset":4444,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4165,"record_offset":4596,"parent_record_offset":4552,"ancestor_record_offset":4444,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4165,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":4444,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4165,"record_offset":4740,"parent_record_offset":4716,"ancestor_record_offset":4444,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4165,"record_offset":4820,"parent_record_offset":4764,"ancestor_record_offset":4444,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":4165,"record_offset":5028,"parent_record_offset":4988,"ancestor_record_offset":4444,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":4165,"record_offset":4988,"parent_record_offset":4552,"ancestor_record_offset":4444,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":4165,"record_offset":5220,"parent_record_offset":4552,"ancestor_record_offset":4444,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4165,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":4444,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4165,"record_offset":4792,"parent_record_offset":4764,"ancestor_record_offset":4444,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4165,"record_offset":5116,"parent_record_offset":5056,"ancestor_record_offset":4444,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4165,"record_offset":5088,"parent_record_offset":5056,"ancestor_record_offset":4444,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4165,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":4444,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$102::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4165,"record_offset":5188,"parent_record_offset":5156,"ancestor_record_offset":4444,"inlinee":9872,"source_line":20,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":4165,"record_offset":5156,"parent_record_offset":4552,"ancestor_record_offset":4444,"inlinee":111400,"source_line":61,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::iter_mut
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x214F4
// 首条位置: {"module":4165,"inlinee":136436}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":4165,"record_offset":5316,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4165,"record_offset":5412,"parent_record_offset":5388,"ancestor_record_offset":5292,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4165,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":5292,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":4165,"record_offset":5340,"parent_record_offset":5316,"ancestor_record_offset":5292,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":4165,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":5292,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/04165.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::remove<system_critters::fish::fish::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4841,"record_offset":15944,"rva":45264128,"va":5413973248}
// 编译实例 RVAs: 0x2b2ad00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::swap_remove::assert_failed
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2647342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413973248,"raw_hex":"e82e652800","size":5,"soft_fail":false,"target_va":5416620930,"va":5413973583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-38294858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413973248,"raw_hex":"e8b6aab7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413973557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-29744675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413973248,"raw_hex":"e8dd213afe","size":5,"soft_fail":false,"target_va":5384228800,"va":5413973470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-29554552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413973248,"raw_hex":"e888083dfe","size":5,"soft_fail":false,"target_va":5384418976,"va":5413973523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4841,"record_offset":16332,"parent_record_offset":15944,"ancestor_record_offset":15944,"inlinee":8600,"source_line":2197,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":4841,"record_offset":16216,"parent_record_offset":15944,"ancestor_record_offset":15944,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4841,"record_offset":16736,"parent_record_offset":15944,"ancestor_record_offset":15944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4841,"record_offset":16132,"parent_record_offset":15944,"ancestor_record_offset":15944,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] system_critters::fish::fish::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":4841,"record_offset":16676,"parent_record_offset":15944,"ancestor_record_offset":15944,"inlinee":145587,"source_line":283,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4841,"record_offset":17016,"parent_record_offset":16992,"ancestor_record_offset":15944,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4841,"record_offset":17040,"parent_record_offset":17016,"ancestor_record_offset":15944,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4841,"record_offset":17064,"parent_record_offset":17040,"ancestor_record_offset":15944,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4841,"record_offset":16992,"parent_record_offset":16904,"ancestor_record_offset":15944,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4841,"record_offset":16932,"parent_record_offset":16904,"ancestor_record_offset":15944,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4841,"record_offset":16904,"parent_record_offset":16880,"ancestor_record_offset":15944,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4841,"record_offset":16880,"parent_record_offset":16856,"ancestor_record_offset":15944,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4841,"record_offset":16376,"parent_record_offset":16332,"ancestor_record_offset":15944,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4841,"record_offset":16468,"parent_record_offset":16332,"ancestor_record_offset":15944,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4841,"record_offset":16784,"parent_record_offset":16760,"ancestor_record_offset":15944,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4841,"record_offset":16832,"parent_record_offset":16808,"ancestor_record_offset":15944,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4841,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":15944,"inlinee":23059,"source_line":381,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4841,"record_offset":16704,"parent_record_offset":16676,"ancestor_record_offset":15944,"inlinee":26130,"source_line":490,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4841,"record_offset":16960,"parent_record_offset":16932,"ancestor_record_offset":15944,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4841,"record_offset":16404,"parent_record_offset":16332,"ancestor_record_offset":15944,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4841,"record_offset":16760,"parent_record_offset":16736,"ancestor_record_offset":15944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4841,"record_offset":16180,"parent_record_offset":16156,"ancestor_record_offset":15944,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4841,"record_offset":16268,"parent_record_offset":16240,"ancestor_record_offset":15944,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4841,"record_offset":16296,"parent_record_offset":16240,"ancestor_record_offset":15944,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4841,"record_offset":16156,"parent_record_offset":16132,"ancestor_record_offset":15944,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::remove<system_critters::wren_boids::sim_wren_boids::closure_env$6>
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4841,"record_offset":17540,"rva":45264560,"va":5413973680}
// 编译实例 RVAs: 0x2b2aeb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::swap_remove::assert_failed
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2646910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413973680,"raw_hex":"e87e632800","size":5,"soft_fail":false,"target_va":5416620930,"va":5413974015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-38295290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413973680,"raw_hex":"e806a9b7fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5413973989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-29745107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413973680,"raw_hex":"e82d203afe","size":5,"soft_fail":false,"target_va":5384228800,"va":5413973902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-29554984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413973680,"raw_hex":"e8d8063dfe","size":5,"soft_fail":false,"target_va":5384418976,"va":5413973955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4841,"record_offset":17944,"parent_record_offset":17540,"ancestor_record_offset":17540,"inlinee":8600,"source_line":2197,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":4841,"record_offset":17828,"parent_record_offset":17540,"ancestor_record_offset":17540,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4841,"record_offset":18348,"parent_record_offset":17540,"ancestor_record_offset":17540,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4841,"record_offset":17744,"parent_record_offset":17540,"ancestor_record_offset":17540,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] system_critters::wren_boids::sim_wren_boids::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/wren_boids.rs
//     evidence={"module":4841,"record_offset":18288,"parent_record_offset":17540,"ancestor_record_offset":17540,"inlinee":145588,"source_line":464,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4841,"record_offset":18628,"parent_record_offset":18604,"ancestor_record_offset":17540,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4841,"record_offset":18652,"parent_record_offset":18628,"ancestor_record_offset":17540,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4841,"record_offset":18676,"parent_record_offset":18652,"ancestor_record_offset":17540,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4841,"record_offset":18604,"parent_record_offset":18516,"ancestor_record_offset":17540,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4841,"record_offset":18544,"parent_record_offset":18516,"ancestor_record_offset":17540,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4841,"record_offset":18516,"parent_record_offset":18492,"ancestor_record_offset":17540,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4841,"record_offset":18492,"parent_record_offset":18468,"ancestor_record_offset":17540,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4841,"record_offset":17988,"parent_record_offset":17944,"ancestor_record_offset":17540,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4841,"record_offset":18080,"parent_record_offset":17944,"ancestor_record_offset":17540,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4841,"record_offset":18396,"parent_record_offset":18372,"ancestor_record_offset":17540,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4841,"record_offset":18444,"parent_record_offset":18420,"ancestor_record_offset":17540,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":4841,"record_offset":17852,"parent_record_offset":17828,"ancestor_record_offset":17540,"inlinee":23059,"source_line":381,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":4841,"record_offset":18316,"parent_record_offset":18288,"ancestor_record_offset":17540,"inlinee":26130,"source_line":490,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4841,"record_offset":18572,"parent_record_offset":18544,"ancestor_record_offset":17540,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4841,"record_offset":18016,"parent_record_offset":17944,"ancestor_record_offset":17540,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4841,"record_offset":18372,"parent_record_offset":18348,"ancestor_record_offset":17540,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4841,"record_offset":17792,"parent_record_offset":17768,"ancestor_record_offset":17540,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4841,"record_offset":17880,"parent_record_offset":17852,"ancestor_record_offset":17540,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4841,"record_offset":17908,"parent_record_offset":17852,"ancestor_record_offset":17540,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4841,"record_offset":17768,"parent_record_offset":17744,"ancestor_record_offset":17540,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::new
// 记录形式: inlinee_declaration; instances=1
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x238A3
// 首条位置: {"module":4840,"inlinee":145571}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::unobstructed_dir
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x238B1
// 首条位置: {"module":4841,"inlinee":145585}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::impls::impl$0::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4841,"record_offset":5508,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":43376,"source_line":262,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4841,"record_offset":5744,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":4841,"record_offset":4996,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":4676,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":16662,"source_line":639,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":4841,"record_offset":4764,"parent_record_offset":4620,"ancestor_record_offset":4620,"inlinee":145576,"source_line":135,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5236,"parent_record_offset":4996,"ancestor_record_offset":4620,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5144,"parent_record_offset":5088,"ancestor_record_offset":4620,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5036,"parent_record_offset":4996,"ancestor_record_offset":4620,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5324,"parent_record_offset":5264,"ancestor_record_offset":4620,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5384,"parent_record_offset":5264,"ancestor_record_offset":4620,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":5676,"parent_record_offset":5648,"ancestor_record_offset":4620,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":4620,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4841,"record_offset":5772,"parent_record_offset":5744,"ancestor_record_offset":4620,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4841,"record_offset":5800,"parent_record_offset":5744,"ancestor_record_offset":4620,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4841,"record_offset":4920,"parent_record_offset":4896,"ancestor_record_offset":4620,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4841,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":4620,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":4841,"record_offset":5264,"parent_record_offset":4996,"ancestor_record_offset":4620,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":4704,"parent_record_offset":4676,"ancestor_record_offset":4620,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":5648,"parent_record_offset":5532,"ancestor_record_offset":4620,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4841,"record_offset":5088,"parent_record_offset":5064,"ancestor_record_offset":4620,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4841,"record_offset":5064,"parent_record_offset":4996,"ancestor_record_offset":4620,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":4841,"record_offset":4828,"parent_record_offset":4796,"ancestor_record_offset":4620,"inlinee":21516,"source_line":26,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":4841,"record_offset":4860,"parent_record_offset":4828,"ancestor_record_offset":4620,"inlinee":21517,"source_line":149,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":4841,"record_offset":4896,"parent_record_offset":4860,"ancestor_record_offset":4620,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":4841,"record_offset":4796,"parent_record_offset":4764,"ancestor_record_offset":4620,"inlinee":145574,"source_line":140,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":4841,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":4620,"inlinee":111380,"source_line":196,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::apply_rules
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x238B0
// 首条位置: {"module":4841,"inlinee":145584}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":7016,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":4032,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":4004,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":4841,"record_offset":1560,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4841,"record_offset":5836,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":6680,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":32190,"source_line":969,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3748,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":1884,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3268,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":1820,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2744,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":4144,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3016,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::div_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2468,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":14579,"source_line":1396,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2288,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":7128,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":1444,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":1712,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2376,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2128,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4841,"record_offset":4060,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4841,"record_offset":2008,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::Boid::unobstructed_dir
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":4841,"record_offset":4620,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":145585,"source_line":156,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":4841,"record_offset":4412,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":111380,"source_line":196,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4841,"record_offset":2224,"parent_record_offset":1168,"ancestor_record_offset":1168,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5236,"parent_record_offset":4996,"ancestor_record_offset":1168,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5144,"parent_record_offset":5088,"ancestor_record_offset":1168,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5036,"parent_record_offset":4996,"ancestor_record_offset":1168,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5324,"parent_record_offset":5264,"ancestor_record_offset":1168,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5384,"parent_record_offset":5264,"ancestor_record_offset":1168,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":2252,"parent_record_offset":2224,"ancestor_record_offset":1168,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":4841,"record_offset":7060,"parent_record_offset":7016,"ancestor_record_offset":1168,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4841,"record_offset":7088,"parent_record_offset":7060,"ancestor_record_offset":1168,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":1168,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":2036,"parent_record_offset":2008,"ancestor_record_offset":1168,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":2952,"parent_record_offset":2920,"ancestor_record_offset":1168,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=51
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":2316,"parent_record_offset":2288,"ancestor_record_offset":1168,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":2160,"parent_record_offset":2128,"ancestor_record_offset":1168,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":1484,"parent_record_offset":1444,"ancestor_record_offset":1168,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":2408,"parent_record_offset":2376,"ancestor_record_offset":1168,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":2500,"parent_record_offset":2468,"ancestor_record_offset":1168,"inlinee":14581,"source_line":985,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":1752,"parent_record_offset":1712,"ancestor_record_offset":1168,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$0::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4841,"record_offset":5508,"parent_record_offset":4620,"ancestor_record_offset":1168,"inlinee":43376,"source_line":262,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4841,"record_offset":5772,"parent_record_offset":5744,"ancestor_record_offset":1168,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4841,"record_offset":1660,"parent_record_offset":1616,"ancestor_record_offset":1168,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4841,"record_offset":1616,"parent_record_offset":1560,"ancestor_record_offset":1168,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4841,"record_offset":4920,"parent_record_offset":4896,"ancestor_record_offset":1168,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4841,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":1168,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":4841,"record_offset":6768,"parent_record_offset":6680,"ancestor_record_offset":1168,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":4841,"record_offset":4996,"parent_record_offset":4620,"ancestor_record_offset":1168,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":4841,"record_offset":5264,"parent_record_offset":4996,"ancestor_record_offset":1168,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":1848,"parent_record_offset":1820,"ancestor_record_offset":1168,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3120,"parent_record_offset":3092,"ancestor_record_offset":1168,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3092,"parent_record_offset":3052,"ancestor_record_offset":1168,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":6712,"parent_record_offset":6680,"ancestor_record_offset":1168,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3052,"parent_record_offset":3016,"ancestor_record_offset":1168,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":4676,"parent_record_offset":4620,"ancestor_record_offset":1168,"inlinee":16662,"source_line":639,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2772,"parent_record_offset":2744,"ancestor_record_offset":1168,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2920,"parent_record_offset":2744,"ancestor_record_offset":1168,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3780,"parent_record_offset":3748,"ancestor_record_offset":1168,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4841,"record_offset":5088,"parent_record_offset":5064,"ancestor_record_offset":1168,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4841,"record_offset":5064,"parent_record_offset":4996,"ancestor_record_offset":1168,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":4841,"record_offset":4828,"parent_record_offset":4796,"ancestor_record_offset":1168,"inlinee":21516,"source_line":26,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4841,"record_offset":3192,"parent_record_offset":3156,"ancestor_record_offset":1168,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":4841,"record_offset":4860,"parent_record_offset":4828,"ancestor_record_offset":1168,"inlinee":21517,"source_line":149,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":4841,"record_offset":4896,"parent_record_offset":4860,"ancestor_record_offset":1168,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":4841,"record_offset":4764,"parent_record_offset":4620,"ancestor_record_offset":1168,"inlinee":145576,"source_line":135,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":4841,"record_offset":4796,"parent_record_offset":4764,"ancestor_record_offset":1168,"inlinee":145574,"source_line":140,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":4841,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":1168,"inlinee":111380,"source_line":196,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4841,"record_offset":956,"rva":45256896,"va":5413966016}
// 编译实例 RVAs: 0x2b290c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::once::futex::Once>::call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2647637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413966016,"raw_hex":"e855662800","size":5,"soft_fail":false,"target_va":5416617264,"va":5413969622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8c720
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: boids.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32100542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413966016,"raw_hex":"e8422f16fe","size":5,"soft_fail":false,"target_va":5381867296,"va":5413967833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2652680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413966016,"raw_hex":"e8087a2800","size":5,"soft_fail":false,"target_va":5416622240,"va":5413969555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-29357818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413966016,"raw_hex":"e8060940fe","size":5,"soft_fail":false,"target_va":5384610144,"va":5413967957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::boids::Boid::random_vel_change
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413966016,"raw_hex":"e818faffff","size":5,"soft_fail":false,"target_va":5413964672,"va":5413966179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30687320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413966016,"raw_hex":"e8a8bf2bfe","size":5,"soft_fail":false,"target_va":5383281568,"va":5413968883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30687754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413966016,"raw_hex":"e8f6bd2bfe","size":5,"soft_fail":false,"target_va":5383278464,"va":5413966213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_vec3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30687522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413966016,"raw_hex":"e8debe2bfe","size":5,"soft_fail":false,"target_va":5383280144,"va":5413967661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":7504,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":145586,"source_line":949,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":7220,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":7844,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":27905,"source_line":519,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":8024,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":16662,"source_line":639,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":7936,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":8712,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":8512,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":8600,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::Boid::apply_rules
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":4841,"record_offset":1168,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":145584,"source_line":186,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":4841,"record_offset":8304,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":111380,"source_line":196,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4841,"record_offset":7444,"parent_record_offset":956,"ancestor_record_offset":956,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5236,"parent_record_offset":4996,"ancestor_record_offset":956,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5144,"parent_record_offset":5088,"ancestor_record_offset":956,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5036,"parent_record_offset":4996,"ancestor_record_offset":956,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5324,"parent_record_offset":5264,"ancestor_record_offset":956,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":5384,"parent_record_offset":5264,"ancestor_record_offset":956,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":7900,"parent_record_offset":7872,"ancestor_record_offset":956,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":2252,"parent_record_offset":2224,"ancestor_record_offset":956,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":4841,"record_offset":7060,"parent_record_offset":7016,"ancestor_record_offset":956,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4841,"record_offset":7088,"parent_record_offset":7060,"ancestor_record_offset":956,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":7872,"parent_record_offset":7844,"ancestor_record_offset":956,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":4032,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":4004,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":956,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":2036,"parent_record_offset":2008,"ancestor_record_offset":956,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":4841,"record_offset":1560,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":2952,"parent_record_offset":2920,"ancestor_record_offset":956,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":2316,"parent_record_offset":2288,"ancestor_record_offset":956,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":2160,"parent_record_offset":2128,"ancestor_record_offset":956,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":8624,"parent_record_offset":8600,"ancestor_record_offset":956,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":1484,"parent_record_offset":1444,"ancestor_record_offset":956,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":2408,"parent_record_offset":2376,"ancestor_record_offset":956,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":2500,"parent_record_offset":2468,"ancestor_record_offset":956,"inlinee":14581,"source_line":985,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":1752,"parent_record_offset":1712,"ancestor_record_offset":956,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$0::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4841,"record_offset":5508,"parent_record_offset":4620,"ancestor_record_offset":956,"inlinee":43376,"source_line":262,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4841,"record_offset":5836,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4841,"record_offset":5772,"parent_record_offset":5744,"ancestor_record_offset":956,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4841,"record_offset":1660,"parent_record_offset":1616,"ancestor_record_offset":956,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4841,"record_offset":1616,"parent_record_offset":1560,"ancestor_record_offset":956,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4841,"record_offset":4920,"parent_record_offset":4896,"ancestor_record_offset":956,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4841,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":956,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":4841,"record_offset":6768,"parent_record_offset":6680,"ancestor_record_offset":956,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":4841,"record_offset":4996,"parent_record_offset":4620,"ancestor_record_offset":956,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":4841,"record_offset":5264,"parent_record_offset":4996,"ancestor_record_offset":956,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":6680,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":32190,"source_line":969,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3748,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":1848,"parent_record_offset":1820,"ancestor_record_offset":956,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3120,"parent_record_offset":3092,"ancestor_record_offset":956,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3092,"parent_record_offset":3052,"ancestor_record_offset":956,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":1820,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2744,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":4144,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3052,"parent_record_offset":3016,"ancestor_record_offset":956,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":3016,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":4676,"parent_record_offset":4620,"ancestor_record_offset":956,"inlinee":16662,"source_line":639,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::div_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2468,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":14579,"source_line":1396,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2288,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2920,"parent_record_offset":2744,"ancestor_record_offset":956,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":1444,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":1712,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2376,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":2128,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4841,"record_offset":5088,"parent_record_offset":5064,"ancestor_record_offset":956,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4841,"record_offset":5064,"parent_record_offset":4996,"ancestor_record_offset":956,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":4841,"record_offset":4828,"parent_record_offset":4796,"ancestor_record_offset":956,"inlinee":21516,"source_line":26,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4841,"record_offset":4060,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4841,"record_offset":2008,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":4841,"record_offset":4860,"parent_record_offset":4828,"ancestor_record_offset":956,"inlinee":21517,"source_line":149,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":4841,"record_offset":4896,"parent_record_offset":4860,"ancestor_record_offset":956,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::Boid::unobstructed_dir
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":4841,"record_offset":4620,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":145585,"source_line":156,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":4841,"record_offset":4764,"parent_record_offset":4620,"ancestor_record_offset":956,"inlinee":145576,"source_line":135,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":4841,"record_offset":4796,"parent_record_offset":4764,"ancestor_record_offset":956,"inlinee":145574,"source_line":140,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":4841,"record_offset":4412,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":111380,"source_line":196,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4841,"record_offset":2224,"parent_record_offset":1168,"ancestor_record_offset":956,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4841,"record_offset":8804,"rva":45260512,"va":5413969632}
// 编译实例 RVAs: 0x2b29ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::once::futex::Once>::call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2644023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413969632,"raw_hex":"e837582800","size":5,"soft_fail":false,"target_va":5416617264,"va":5413973236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142933110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: boids.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2062011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413969632,"raw_hex":"e84589e0ff","size":5,"soft_fail":false,"target_va":5411909904,"va":5413971910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2649066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413969632,"raw_hex":"e8ea6b2800","size":5,"soft_fail":false,"target_va":5416622240,"va":5413973169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-29361463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413969632,"raw_hex":"e8c9fa3ffe","size":5,"soft_fail":false,"target_va":5384610144,"va":5413971602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::boids::Boid::random_vel_change
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413969632,"raw_hex":"e8e3ebffff","size":5,"soft_fail":false,"target_va":5413964672,"va":5413969816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::wren_boids::sim_wren_boids::closure$3
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413969632,"raw_hex":"e8efe8ffff","size":5,"soft_fail":false,"target_va":5413965600,"va":5413971500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30690944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413969632,"raw_hex":"e880b12bfe","size":5,"soft_fail":false,"target_va":5383281568,"va":5413972507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30691392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413969632,"raw_hex":"e8c0af2bfe","size":5,"soft_fail":false,"target_va":5383278464,"va":5413969851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_vec3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30691198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413969632,"raw_hex":"e882b02bfe","size":5,"soft_fail":false,"target_va":5383280144,"va":5413971337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":14848,"parent_record_offset":8804,"ancestor_record_offset":8804,"inlinee":145586,"source_line":949,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":14560,"parent_record_offset":8804,"ancestor_record_offset":8804,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":15188,"parent_record_offset":8804,"ancestor_record_offset":8804,"inlinee":27905,"source_line":519,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":15368,"parent_record_offset":8804,"ancestor_record_offset":8804,"inlinee":16662,"source_line":639,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":15280,"parent_record_offset":8804,"ancestor_record_offset":8804,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":15852,"parent_record_offset":8804,"ancestor_record_offset":8804,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":15652,"parent_record_offset":8804,"ancestor_record_offset":8804,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":15740,"parent_record_offset":8804,"ancestor_record_offset":8804,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::Boid::apply_rules
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":4841,"record_offset":9028,"parent_record_offset":8804,"ancestor_record_offset":8804,"inlinee":145584,"source_line":186,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4841,"record_offset":14788,"parent_record_offset":8804,"ancestor_record_offset":8804,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":12812,"parent_record_offset":12572,"ancestor_record_offset":8804,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":12720,"parent_record_offset":12664,"ancestor_record_offset":8804,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":12612,"parent_record_offset":12572,"ancestor_record_offset":8804,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":12900,"parent_record_offset":12840,"ancestor_record_offset":8804,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4841,"record_offset":12960,"parent_record_offset":12840,"ancestor_record_offset":8804,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":15244,"parent_record_offset":15216,"ancestor_record_offset":8804,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":10116,"parent_record_offset":10088,"ancestor_record_offset":8804,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":4841,"record_offset":14400,"parent_record_offset":14356,"ancestor_record_offset":8804,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4841,"record_offset":14428,"parent_record_offset":14400,"ancestor_record_offset":8804,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":15216,"parent_record_offset":15188,"ancestor_record_offset":8804,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":11896,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":11868,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":11948,"parent_record_offset":11924,"ancestor_record_offset":8804,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4841,"record_offset":9900,"parent_record_offset":9872,"ancestor_record_offset":8804,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":4841,"record_offset":9424,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":10816,"parent_record_offset":10784,"ancestor_record_offset":8804,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":8804,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":10024,"parent_record_offset":9992,"ancestor_record_offset":8804,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":15764,"parent_record_offset":15740,"ancestor_record_offset":8804,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":9348,"parent_record_offset":9308,"ancestor_record_offset":8804,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":10272,"parent_record_offset":10240,"ancestor_record_offset":8804,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":10364,"parent_record_offset":10332,"ancestor_record_offset":8804,"inlinee":14581,"source_line":985,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4841,"record_offset":9616,"parent_record_offset":9576,"ancestor_record_offset":8804,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4841,"record_offset":13176,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4841,"record_offset":13112,"parent_record_offset":13084,"ancestor_record_offset":8804,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4841,"record_offset":9524,"parent_record_offset":9480,"ancestor_record_offset":8804,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4841,"record_offset":9480,"parent_record_offset":9424,"ancestor_record_offset":8804,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4841,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":8804,"inlinee":4202,"source_line":2842,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":4841,"record_offset":12520,"parent_record_offset":12496,"ancestor_record_offset":8804,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":4841,"record_offset":14108,"parent_record_offset":14020,"ancestor_record_offset":8804,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":4841,"record_offset":12572,"parent_record_offset":12256,"ancestor_record_offset":8804,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":4841,"record_offset":12840,"parent_record_offset":12572,"ancestor_record_offset":8804,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":14020,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":32190,"source_line":969,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":11612,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":9712,"parent_record_offset":9684,"ancestor_record_offset":8804,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":10984,"parent_record_offset":10956,"ancestor_record_offset":8804,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":10956,"parent_record_offset":10916,"ancestor_record_offset":8804,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":9684,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":10608,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":12008,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":10916,"parent_record_offset":10880,"ancestor_record_offset":8804,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":10880,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":12312,"parent_record_offset":12256,"ancestor_record_offset":8804,"inlinee":16662,"source_line":639,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::div_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":10332,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":14579,"source_line":1396,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":10152,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":10784,"parent_record_offset":10608,"ancestor_record_offset":8804,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":9308,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":9576,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":10240,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":26700,"source_line":1764,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4841,"record_offset":9992,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4841,"record_offset":12664,"parent_record_offset":12640,"ancestor_record_offset":8804,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4841,"record_offset":12640,"parent_record_offset":12572,"ancestor_record_offset":8804,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":4841,"record_offset":12404,"parent_record_offset":12372,"ancestor_record_offset":8804,"inlinee":21516,"source_line":26,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4841,"record_offset":11924,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4841,"record_offset":9872,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\once.rs
//     evidence={"module":4841,"record_offset":12436,"parent_record_offset":12404,"ancestor_record_offset":8804,"inlinee":21517,"source_line":149,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\once\futex.rs
//     evidence={"module":4841,"record_offset":12472,"parent_record_offset":12436,"ancestor_record_offset":8804,"inlinee":4531,"source_line":79,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::Boid::unobstructed_dir
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":4841,"record_offset":12256,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":145585,"source_line":156,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":4841,"record_offset":12340,"parent_record_offset":12256,"ancestor_record_offset":8804,"inlinee":145576,"source_line":135,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":4841,"record_offset":12372,"parent_record_offset":12340,"ancestor_record_offset":8804,"inlinee":145574,"source_line":140,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4841,"record_offset":10088,"parent_record_offset":9028,"ancestor_record_offset":8804,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/04841.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::random_vel_change
// 记录形式: compiled_procedure; instances=1
// 源码行: 384
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4840,"record_offset":272,"rva":45255552,"va":5413964672}
// 编译实例 RVAs: 0x2b28b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-30581582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413964672,"raw_hex":"e8b25c2dfe","size":5,"soft_fail":false,"target_va":5383383152,"va":5413964729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-29354819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413964672,"raw_hex":"e8bd1440fe","size":5,"soft_fail":false,"target_va":5384610144,"va":5413964958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30683258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413964672,"raw_hex":"e886cf2bfe","size":5,"soft_fail":false,"target_va":5383281568,"va":5413964821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_vec3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30684750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413964672,"raw_hex":"e8b2c92bfe","size":5,"soft_fail":false,"target_va":5383280144,"va":5413964889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::uniform_sample_cone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30687610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5413964672,"raw_hex":"e886be2bfe","size":5,"soft_fail":false,"target_va":5383277168,"va":5413964773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":4840,"record_offset":404,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":4840,"record_offset":428,"parent_record_offset":404,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4840,"record_offset":932,"parent_record_offset":512,"ancestor_record_offset":272,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4840,"record_offset":840,"parent_record_offset":784,"ancestor_record_offset":272,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4840,"record_offset":608,"parent_record_offset":584,"ancestor_record_offset":272,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4840,"record_offset":640,"parent_record_offset":556,"ancestor_record_offset":272,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4840,"record_offset":704,"parent_record_offset":512,"ancestor_record_offset":272,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":4840,"record_offset":512,"parent_record_offset":404,"ancestor_record_offset":272,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":4840,"record_offset":960,"parent_record_offset":512,"ancestor_record_offset":272,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":4840,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":272,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":4840,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":272,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4840,"record_offset":584,"parent_record_offset":556,"ancestor_record_offset":272,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/04840.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4840,"record_offset":556,"parent_record_offset":512,"ancestor_record_offset":272,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/04840.jsonl.gz"}
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
// 完整结构及原始类型记录: boids.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::boids::Boid
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_critters::boids::Boid", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>", "constructor": "system_critters::boids::Boid::frame_update", "arguments": ["system_critters::fish::fish::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>", "constructor": "system_critters::boids::Boid::frame_update", "arguments": ["system_critters::wren_boids::sim_wren_boids::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::boids::BoidAux
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_critters::boids::BoidAux,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_critters::boids::BoidAux", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["system_critters::boids::BoidAux", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>,system_critters::boids::impl$0::drain::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>", "system_critters::boids::impl$0::drain::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>,system_critters::boids::impl$0::drain::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>,system_critters::boids::impl$0::drain::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::boids::BoidFlock
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::BoidFlock::remove<system_critters::fish::fish::closure_env$1>", "constructor": "system_critters::boids::BoidFlock::remove", "arguments": ["system_critters::fish::fish::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::BoidFlock::remove<system_critters::wren_boids::sim_wren_boids::closure_env$6>", "constructor": "system_critters::boids::BoidFlock::remove", "arguments": ["system_critters::wren_boids::sim_wren_boids::closure_env$6"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: boids.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: boids.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] array$<glam::f32::sse2::vec3a::Vec3A,32> (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::apply_rules
// 信息 ID: dd13a033bea3ea3568013f44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>; module=4841, inline_record=1168; 不是独立调用边。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>; module=4841, inline_record=9028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>
// 信息 ID: 73b0414767f8c4fe0aa13405
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::Boid::frame_update -> ["system_critters::fish::fish::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::fish::fish @ 0x2938f3f -> 0x2b290c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>
// 信息 ID: 0257c0465260b69b4209c4b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::Boid::frame_update -> ["system_critters::wren_boids::sim_wren_boids::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x2946837 -> 0x2b29ee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::new
// 信息 ID: 514238dda68248d97f4e52c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::random_vel_change
// 信息 ID: e3bef1819039fd8449c06dff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0> @ 0x2b29163 -> 0x2b28b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x2b29f98 -> 0x2b28b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::unobstructed_dir
// 信息 ID: 69e8f149eaff321ea38d269a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>; module=4841, inline_record=4620; 不是独立调用边。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>; module=4841, inline_record=12256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::clone_state
// 信息 ID: 9b138af13bafaf0d3544afb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=4165, inline_record=4444; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=4178, inline_record=14436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::drain
// 信息 ID: 7199fd010fdea9bf56fe35f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=4178, inline_record=3656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::iter_mut
// 信息 ID: 1776a6a01fba9bc95843c4d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=4165, inline_record=5292; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=4165, inline_record=14784; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=4165, inline_record=15212; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=4178, inline_record=15444; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=4178, inline_record=28972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::len
// 信息 ID: d7386a7f443fe78a2f743ab7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=4165, inline_record=13744; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=4178, inline_record=14408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::push
// 信息 ID: 0f0c1b4966518a7e4877f25a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=4165, inline_record=3976; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=4178, inline_record=5340; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=4178, inline_record=12000; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=4178, inline_record=27272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::remove<system_critters::fish::fish::closure_env$1>
// 信息 ID: 41fed77e273409d2d551812a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::BoidFlock::remove -> ["system_critters::fish::fish::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::fish::fish @ 0x293996d -> 0x2b2ad00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::remove<system_critters::wren_boids::sim_wren_boids::closure_env$6>
// 信息 ID: f3a5fc50344c6ea4422aa509
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::BoidFlock::remove -> ["system_critters::wren_boids::sim_wren_boids::closure_env$6"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x2947a0c -> 0x2b2aeb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::impl$5::clone
// 信息 ID: f2a27d3c842c97bad2a95a97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=4165, inline_record=5156; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=4178, inline_record=15336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::impl$6::default
// 信息 ID: 597872224332249af0d5dd75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::impl$0::build; module=344, inline_record=524; 不是独立调用边。
// [PDB 内联宿主] system_critters::impl$0::build; module=344, inline_record=924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
