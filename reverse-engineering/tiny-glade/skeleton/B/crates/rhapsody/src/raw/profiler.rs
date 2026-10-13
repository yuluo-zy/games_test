// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/profiler.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6afbda6a5261ae4f4345cbd1be605b3eac503664222009f75331e0b035f5ddd4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: profiler.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 675, 703, 1972
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use gpu_allocator; // namespace candidate
//   package identities: [{"name":"gpu-allocator","source":"local_or_path","version":"0.26.0"}]
// use gpu_profiler; // namespace candidate
//   package identities: [{"name":"gpu-profiler","source":"local_or_path","version":"0.1.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_gpu_profiler_output_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":146612,"rva":12580000,"va":5381289120}
// 编译实例 RVAs: 0xbff4a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1355214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289120,"raw_hex":"e8cead1400","size":5,"soft_fail":false,"target_va":5382644528,"va":5381289309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5773135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289120,"raw_hex":"e8b1e8a7ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381289162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bff5a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289120,"raw_hex":"e826000000","size":5,"soft_fail":false,"target_va":5381289376,"va":5381289333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":146860,"parent_record_offset":146612,"ancestor_record_offset":146612,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] gpu_profiler::backend::ash::ProfilerOutputBuffer::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/gpu-profiler/src/backend/ash.rs
//     evidence={"module":703,"record_offset":147068,"parent_record_offset":146612,"ancestor_record_offset":146612,"inlinee":39785,"source_line":38,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":146892,"parent_record_offset":146612,"ancestor_record_offset":146612,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":146780,"parent_record_offset":146612,"ancestor_record_offset":146612,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":146948,"parent_record_offset":146916,"ancestor_record_offset":146612,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":146976,"parent_record_offset":146916,"ancestor_record_offset":146612,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":146804,"parent_record_offset":146780,"ancestor_record_offset":146612,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":146824,"parent_record_offset":146804,"ancestor_record_offset":146612,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":147004,"parent_record_offset":146916,"ancestor_record_offset":146612,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":146916,"parent_record_offset":146892,"ancestor_record_offset":146612,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":147028,"parent_record_offset":147004,"ancestor_record_offset":146612,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::profiler::impl$1::timestamp_period
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/profiler.rs
//     evidence={"module":703,"record_offset":147100,"parent_record_offset":147068,"ancestor_record_offset":146612,"inlinee":39787,"source_line":69,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::profiler::impl$1::create_query_result_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":147136,"rva":12580256,"va":5381289376}
// 编译实例 RVAs: 0xbff5a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11132068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e85c2356ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381290191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6208638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e88243a1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381290137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406868b0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5738089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e89771a8ff","size":5,"soft_fail":false,"target_va":5375551664,"va":5381289748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-131844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e8fcfcfdff","size":5,"soft_fail":false,"target_va":5381158416,"va":5381290255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30252942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e88e9fcd01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381289933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocation::memory
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5433946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e8a615adff","size":5,"soft_fail":false,"target_va":5375855792,"va":5381289733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30202493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e87ddacc01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381289870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30204910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e8eee3cc01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381289885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t534037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e815260800","size":5,"soft_fail":false,"target_va":5381823600,"va":5381289558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t534304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e820270800","size":5,"soft_fail":false,"target_va":5381824016,"va":5381289707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t58272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381289376,"raw_hex":"e8a0e30000","size":5,"soft_fail":false,"target_va":5381347904,"va":5381289627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*384(%r14)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381289376,"raw_hex":"41ff9680010000","size":7,"soft_fail":false,"va":5381289482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":147456,"parent_record_offset":147136,"ancestor_record_offset":147136,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::bind_buffer_memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":148180,"parent_record_offset":147136,"ancestor_record_offset":147136,"inlinee":39597,"source_line":2702,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":147312,"parent_record_offset":147136,"ancestor_record_offset":147136,"inlinee":39592,"source_line":2202,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::get_buffer_memory_requirements
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":147428,"parent_record_offset":147136,"ancestor_record_offset":147136,"inlinee":39593,"source_line":2636,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":147992,"parent_record_offset":147136,"ancestor_record_offset":147136,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":148268,"parent_record_offset":147136,"ancestor_record_offset":147136,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":147696,"parent_record_offset":147136,"ancestor_record_offset":147136,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":147484,"parent_record_offset":147136,"ancestor_record_offset":147136,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":147640,"parent_record_offset":147136,"ancestor_record_offset":147136,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":147932,"parent_record_offset":147908,"ancestor_record_offset":147136,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":147908,"parent_record_offset":147852,"ancestor_record_offset":147136,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":147880,"parent_record_offset":147852,"ancestor_record_offset":147136,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":147852,"parent_record_offset":147828,"ancestor_record_offset":147136,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":147828,"parent_record_offset":147804,"ancestor_record_offset":147136,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":147340,"parent_record_offset":147312,"ancestor_record_offset":147136,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":147364,"parent_record_offset":147340,"ancestor_record_offset":147136,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":147388,"parent_record_offset":147364,"ancestor_record_offset":147136,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":147664,"parent_record_offset":147640,"ancestor_record_offset":147136,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":147732,"parent_record_offset":147696,"ancestor_record_offset":147136,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":148112,"parent_record_offset":148048,"ancestor_record_offset":147136,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":147548,"parent_record_offset":147512,"ancestor_record_offset":147136,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":148136,"parent_record_offset":148112,"ancestor_record_offset":147136,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":147572,"parent_record_offset":147548,"ancestor_record_offset":147136,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":148020,"parent_record_offset":147992,"ancestor_record_offset":147136,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":147512,"parent_record_offset":147484,"ancestor_record_offset":147136,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":148048,"parent_record_offset":148020,"ancestor_record_offset":147136,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":147604,"parent_record_offset":147512,"ancestor_record_offset":147136,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":148084,"parent_record_offset":148048,"ancestor_record_offset":147136,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::profiler::impl$1::timestamp_period
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B6B
// 首条位置: {"module":703,"inlinee":39787}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::profiler::impl$2::mapped_slice
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":675,"record_offset":3988,"rva":12401792,"va":5381110912}
// 编译实例 RVAs: 0xbd3c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30430829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381110912,"raw_hex":"e86d56d001","size":5,"soft_fail":false,"target_va":5411541776,"va":5381110942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocation::mapped_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t251699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381110912,"raw_hex":"e833d70300","size":5,"soft_fail":false,"target_va":5381362624,"va":5381110920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":675,"record_offset":4120,"parent_record_offset":3988,"ancestor_record_offset":3988,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::profiler::impl$2::raw
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x981C
// 首条位置: {"module":1972,"inlinee":38940}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::profiler::TimedFrame::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":675,"record_offset":4152,"rva":12401840,"va":5381110960}
// 编译实例 RVAs: 0xbd3cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6029572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381110960,"raw_hex":"e8fcfea3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381111055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6029667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381110960,"raw_hex":"e89dfea3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381111166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6754517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381110960,"raw_hex":"e82bef98ff","size":5,"soft_fail":false,"target_va":5374356912,"va":5381111424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30429868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381110960,"raw_hex":"e8ac52d001","size":5,"soft_fail":false,"target_va":5411540963,"va":5381111090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_profiler::shared::FrameMarkers::marker_name
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t250011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381110960,"raw_hex":"e89bd00300","size":5,"soft_fail":false,"target_va":5381361376,"va":5381111360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_profiler::shared::GpuTick::duration_since
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t249849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381110960,"raw_hex":"e8f9cf0300","size":5,"soft_fail":false,"target_va":5381361232,"va":5381111378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_profiler::shared::GpuTickDuration::nanoseconds
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t249848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381110960,"raw_hex":"e8f8cf0300","size":5,"soft_fail":false,"target_va":5381361248,"va":5381111395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":5764,"parent_record_offset":4152,"ancestor_record_offset":4152,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":4288,"parent_record_offset":4152,"ancestor_record_offset":4152,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":675,"record_offset":5272,"parent_record_offset":4152,"ancestor_record_offset":4152,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":675,"record_offset":4744,"parent_record_offset":4152,"ancestor_record_offset":4152,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":675,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":4152,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":675,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":4152,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":675,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":4152,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":675,"record_offset":4640,"parent_record_offset":4416,"ancestor_record_offset":4152,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":675,"record_offset":4908,"parent_record_offset":4816,"ancestor_record_offset":4152,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":5868,"parent_record_offset":5820,"ancestor_record_offset":4152,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":4352,"parent_record_offset":4320,"ancestor_record_offset":4152,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":4844,"parent_record_offset":4816,"ancestor_record_offset":4152,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":4152,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":5924,"parent_record_offset":5896,"ancestor_record_offset":4152,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":5896,"parent_record_offset":5868,"ancestor_record_offset":4152,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":4416,"parent_record_offset":4380,"ancestor_record_offset":4152,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":4380,"parent_record_offset":4352,"ancestor_record_offset":4152,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":4792,"parent_record_offset":4768,"ancestor_record_offset":4152,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":4460,"parent_record_offset":4416,"ancestor_record_offset":4152,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":5820,"parent_record_offset":5764,"ancestor_record_offset":4152,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":4320,"parent_record_offset":4288,"ancestor_record_offset":4152,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":675,"record_offset":5468,"parent_record_offset":5444,"ancestor_record_offset":4152,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":675,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":4152,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":675,"record_offset":5360,"parent_record_offset":5272,"ancestor_record_offset":4152,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":675,"record_offset":4488,"parent_record_offset":4460,"ancestor_record_offset":4152,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":675,"record_offset":4516,"parent_record_offset":4488,"ancestor_record_offset":4152,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":675,"record_offset":4596,"parent_record_offset":4540,"ancestor_record_offset":4152,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":675,"record_offset":5304,"parent_record_offset":5272,"ancestor_record_offset":4152,"inlinee":9207,"source_line":347,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":675,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":4152,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":675,"record_offset":4568,"parent_record_offset":4540,"ancestor_record_offset":4152,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":675,"record_offset":4876,"parent_record_offset":4844,"ancestor_record_offset":4152,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":675,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":4152,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":675,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":4152,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":675,"record_offset":5384,"parent_record_offset":5360,"ancestor_record_offset":4152,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":675,"record_offset":5968,"parent_record_offset":5764,"ancestor_record_offset":4152,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::profiler::TimedFrame::send_to_puffin
// 记录形式: compiled_procedure; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":675,"record_offset":7088,"rva":12402736,"va":5381111856}
// 编译实例 RVAs: 0xbd4030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6030855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381111856,"raw_hex":"e8f9f9a3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381112338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6030974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381111856,"raw_hex":"e882f9a3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381112473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: profiler.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5595977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381111856,"raw_hex":"e8b79caaff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381112004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1725514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381111856,"raw_hex":"e84a541a00","size":5,"soft_fail":false,"target_va":5382837616,"va":5381112097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30369717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381111856,"raw_hex":"e8b567cf01","size":5,"soft_fail":false,"target_va":5411481920,"va":5381112198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30428347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381111856,"raw_hex":"e8bb4cd001","size":5,"soft_fail":false,"target_va":5411540963,"va":5381112611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30430583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381111856,"raw_hex":"e87755d001","size":5,"soft_fail":false,"target_va":5411543104,"va":5381112516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::Stream::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381111856,"raw_hex":"e8b5f9ffff","size":5,"soft_fail":false,"target_va":5381110336,"va":5381111942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::global_reporter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t272266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381111856,"raw_hex":"e88a270400","size":5,"soft_fail":false,"target_va":5381384720,"va":5381112449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":675,"record_offset":10076,"parent_record_offset":7088,"ancestor_record_offset":7088,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":7272,"parent_record_offset":7088,"ancestor_record_offset":7088,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":675,"record_offset":10576,"parent_record_offset":7088,"ancestor_record_offset":7088,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":675,"record_offset":7532,"parent_record_offset":7088,"ancestor_record_offset":7088,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_direct] puffin::Stream::end_scope
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/data.rs
//     evidence={"module":675,"record_offset":7684,"parent_record_offset":7088,"ancestor_record_offset":7088,"inlinee":38933,"source_line":79,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_direct] puffin::StreamInfo::as_stream_into_ref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":675,"record_offset":10548,"parent_record_offset":7088,"ancestor_record_offset":7088,"inlinee":38936,"source_line":307,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_direct] puffin::impl$12::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":675,"record_offset":7248,"parent_record_offset":7088,"ancestor_record_offset":7088,"inlinee":38932,"source_line":141,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":675,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":7088,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":675,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":7088,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":675,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":7088,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":675,"record_offset":10368,"parent_record_offset":10340,"ancestor_record_offset":7088,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":675,"record_offset":10752,"parent_record_offset":10696,"ancestor_record_offset":7088,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":7088,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":8568,"parent_record_offset":8540,"ancestor_record_offset":7088,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":10280,"parent_record_offset":10252,"ancestor_record_offset":7088,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":8652,"parent_record_offset":8628,"ancestor_record_offset":7088,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":10724,"parent_record_offset":10696,"ancestor_record_offset":7088,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":10696,"parent_record_offset":10672,"ancestor_record_offset":7088,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":8628,"parent_record_offset":8596,"ancestor_record_offset":7088,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":7088,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":7400,"parent_record_offset":7376,"ancestor_record_offset":7088,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":8596,"parent_record_offset":8568,"ancestor_record_offset":7088,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":10340,"parent_record_offset":10308,"ancestor_record_offset":7088,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":10308,"parent_record_offset":10280,"ancestor_record_offset":7088,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":675,"record_offset":10672,"parent_record_offset":10648,"ancestor_record_offset":7088,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":675,"record_offset":10216,"parent_record_offset":10188,"ancestor_record_offset":7088,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":675,"record_offset":10188,"parent_record_offset":10160,"ancestor_record_offset":7088,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":675,"record_offset":10132,"parent_record_offset":10104,"ancestor_record_offset":7088,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":675,"record_offset":10160,"parent_record_offset":10132,"ancestor_record_offset":7088,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":675,"record_offset":10104,"parent_record_offset":10076,"ancestor_record_offset":7088,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":8500,"parent_record_offset":8468,"ancestor_record_offset":7088,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":7088,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":7872,"parent_record_offset":7792,"ancestor_record_offset":7088,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":7088,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":7328,"parent_record_offset":7296,"ancestor_record_offset":7088,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":8436,"parent_record_offset":8404,"ancestor_record_offset":7088,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":7736,"parent_record_offset":7684,"ancestor_record_offset":7088,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":8160,"parent_record_offset":7684,"ancestor_record_offset":7088,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":8540,"parent_record_offset":8500,"ancestor_record_offset":7088,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":10252,"parent_record_offset":10216,"ancestor_record_offset":7088,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":7764,"parent_record_offset":7684,"ancestor_record_offset":7088,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":675,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":7088,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":675,"record_offset":8468,"parent_record_offset":8436,"ancestor_record_offset":7088,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::WriteBytesExt::write_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":675,"record_offset":8372,"parent_record_offset":8340,"ancestor_record_offset":7088,"inlinee":38926,"source_line":1394,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":675,"record_offset":8020,"parent_record_offset":7684,"ancestor_record_offset":7088,"inlinee":9455,"source_line":1367,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":675,"record_offset":8116,"parent_record_offset":8092,"ancestor_record_offset":7088,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":675,"record_offset":10468,"parent_record_offset":10216,"ancestor_record_offset":7088,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":675,"record_offset":10600,"parent_record_offset":10576,"ancestor_record_offset":7088,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":675,"record_offset":7492,"parent_record_offset":7468,"ancestor_record_offset":7088,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":675,"record_offset":7644,"parent_record_offset":7620,"ancestor_record_offset":7088,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":675,"record_offset":7568,"parent_record_offset":7532,"ancestor_record_offset":7088,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":675,"record_offset":8308,"parent_record_offset":8160,"ancestor_record_offset":7088,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":675,"record_offset":8092,"parent_record_offset":8068,"ancestor_record_offset":7088,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":675,"record_offset":7792,"parent_record_offset":7764,"ancestor_record_offset":7088,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":675,"record_offset":7836,"parent_record_offset":7792,"ancestor_record_offset":7088,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":675,"record_offset":7468,"parent_record_offset":7296,"ancestor_record_offset":7088,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":675,"record_offset":7620,"parent_record_offset":7596,"ancestor_record_offset":7088,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":675,"record_offset":7596,"parent_record_offset":7532,"ancestor_record_offset":7088,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] puffin::Stream::write_nanos
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/data.rs
//     evidence={"module":675,"record_offset":8340,"parent_record_offset":7684,"ancestor_record_offset":7088,"inlinee":38934,"source_line":95,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$10::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":675,"record_offset":8068,"parent_record_offset":8044,"ancestor_record_offset":7088,"inlinee":5245,"source_line":430,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$10::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":675,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":7088,"inlinee":5314,"source_line":457,"archive":"_pdb/B/modules/00675.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":675,"record_offset":8404,"parent_record_offset":8372,"ancestor_record_offset":7088,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00675.jsonl.gz"}
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
// 完整结构及原始类型记录: profiler.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::profiler::ProfilerBuffer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 20
// [PDB 泛型关系，非字段关系] {"type": "gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer>", "constructor": "gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0", "arguments": ["rhapsody::raw::profiler::ProfilerBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >", "constructor": "core::slice::impl$0::sort_unstable_by_key::closure_env$0", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "u64", "gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::quicksort::quicksort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::unstable::quicksort::quicksort", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "gpu_profiler::backend::ash::ProfilerSubmittedFrame::retrieve_results<rhapsody::raw::profiler::ProfilerBuffer>", "constructor": "gpu_profiler::backend::ash::ProfilerSubmittedFrame::retrieve_results", "arguments": ["rhapsody::raw::profiler::ProfilerBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::heapsort::heapsort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::unstable::heapsort::heapsort", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>", "constructor": "gpu_profiler::backend::ash::ProfilerOutputBuffer", "arguments": ["rhapsody::raw::profiler::ProfilerBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::choose_pivot<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::pivot::choose_pivot", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::bidirectional_merge<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::smallsort::bidirectional_merge", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::unstable::ipnsort<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "core::slice::sort::unstable::ipnsort", "arguments": ["tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>", "core::slice::impl$0::sort_unstable_by_key::closure_env$0<tuple$<gpu_profiler::shared::MarkerId,gpu_profiler::shared::GpuTick>,u64,gpu_profiler::backend::ash::impl$1::retrieve_results::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer> >", "constructor": "core::ptr::drop_in_place", "arguments": ["gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "gpu_profiler::backend::ash::ProfilerSubmittedFrame<rhapsody::raw::profiler::ProfilerBuffer>", "constructor": "gpu_profiler::backend::ash::ProfilerSubmittedFrame", "arguments": ["rhapsody::raw::profiler::ProfilerBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<gpu_profiler::backend::ash::ProfilerSubmittedFrame<rhapsody::raw::profiler::ProfilerBuffer> >", "constructor": "core::ptr::drop_in_place", "arguments": ["gpu_profiler::backend::ash::ProfilerSubmittedFrame<rhapsody::raw::profiler::ProfilerBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer> >", "constructor": "core::option::Option", "arguments": ["gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "enum2$", "arguments": ["core::option::Option<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer>", "constructor": "gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0", "arguments": ["rhapsody::raw::profiler::ProfilerBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::take::Take<core::slice::iter::Iter<core::sync::atomic::AtomicU64> >,gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::take::Take<core::slice::iter::Iter<core::sync::atomic::AtomicU64> >", "gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<gpu_profiler::shared::MarkerId,core::iter::adapters::map::Map<core::iter::adapters::take::Take<core::slice::iter::Iter<core::sync::atomic::AtomicU64> >,gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["gpu_profiler::shared::MarkerId", "core::iter::adapters::map::Map<core::iter::adapters::take::Take<core::slice::iter::Iter<core::sync::atomic::AtomicU64> >,gpu_profiler::backend::ash::impl$3::end_frame::closure_env$0<rhapsody::raw::profiler::ProfilerBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "gpu_profiler::backend::ash::ActiveProfiler::end_frame<rhapsody::raw::profiler::ProfilerBuffer>", "constructor": "gpu_profiler::backend::ash::ActiveProfiler::end_frame", "arguments": ["rhapsody::raw::profiler::ProfilerBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::profiler::TimedFrame
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: profiler.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: profiler.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_gpu_profiler_output_buffer
// 信息 ID: 79982e86e4047ac51d1a86c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e95e3 -> 0xbff4a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::profiler::TimedFrame::new
// 信息 ID: c4176c731ef55d7ae82b7630
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e99c0 -> 0xbd3cb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::profiler::TimedFrame::send_to_puffin
// 信息 ID: ae12ac82113216529912646e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e99cf -> 0xbd4030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::profiler::impl$1::create_query_result_buffer
// 信息 ID: 94b7f2b5af4e91377d650ca1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_gpu_profiler_output_buffer @ 0xbff575 -> 0xbff5a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::profiler::impl$1::timestamp_period
// 信息 ID: dff3798fc9f26ab2e1006ea6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_gpu_profiler_output_buffer; module=703, inline_record=147100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::profiler::impl$2::mapped_slice
// 信息 ID: b208da0836e8197e9dc2931e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] gpu_profiler::backend::ash::ProfilerSubmittedFrame::retrieve_results<rhapsody::raw::profiler::ProfilerBuffer> @ 0x294d25 -> 0xbd3c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::profiler::impl$2::raw
// 信息 ID: d4f1e6f1601b0d1ba71b041a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] gpu_profiler::backend::ash::ActiveProfiler::end_frame<rhapsody::raw::profiler::ProfilerBuffer>; module=1972, inline_record=516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
