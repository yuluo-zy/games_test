// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/managed/swapchain.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 79a36b168d1d386bb47a06321ce9f83db6471dcaf96d68e9a53f0815532be62c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: swapchain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 707, 710, 1935, 1936, 1987, 3273
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use append_only_vec; // namespace candidate
//   package identities: [{"name":"append-only-vec","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8","version":"0.1.2"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use leekspin; // namespace candidate
//   package identities: [{"name":"leekspin","source":"local_or_path","version":"0.1.0"}]
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
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1987,"record_offset":4724,"rva":25993264,"va":5394702384}
// 编译实例 RVAs: 0x18ca030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-12058314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e8360148ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5394702837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21075933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e82368befe","size":5,"soft_fail":false,"target_va":5373629488,"va":5394705416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19621871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e81198d4fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5394703354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19622382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e81296d4fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5394703881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=6
//     target_aliases=69 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19186625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e83f3cdbfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5394702652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be15f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13539758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e8526631ff","size":5,"soft_fail":false,"target_va":5381166576,"va":5394706329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b5f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-84941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e833b4feff","size":5,"soft_fail":false,"target_va":5394620272,"va":5394705208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b68f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-82823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e879bcfeff","size":5,"soft_fail":false,"target_va":5394622704,"va":5394705522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b6bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-82113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e83fbffeff","size":5,"soft_fail":false,"target_va":5394623472,"va":5394705580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16834898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e852e10001","size":5,"soft_fail":false,"target_va":5411540992,"va":5394706089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::prelude::read_into_uninitialized_vector
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-105654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e84a63feff","size":5,"soft_fail":false,"target_va":5394598032,"va":5394703681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16836716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394702384,"raw_hex":"e86ce80001","size":5,"soft_fail":false,"target_va":5411542880,"va":5394706159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*32(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5394702384,"raw_hex":"ff5020","size":3,"soft_fail":false,"va":5394702474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":10844,"parent_record_offset":4724,"ancestor_record_offset":4724,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":10880,"parent_record_offset":4724,"ancestor_record_offset":4724,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":4872,"parent_record_offset":4724,"ancestor_record_offset":4724,"inlinee":87460,"source_line":125,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":4724,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":4724,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":4724,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5780,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5804,"parent_record_offset":5780,"ancestor_record_offset":4724,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":6964,"parent_record_offset":6880,"ancestor_record_offset":4724,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":6908,"parent_record_offset":6880,"ancestor_record_offset":4724,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":4724,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":4724,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1987,"record_offset":6160,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":4724,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":8804,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1987,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":4724,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1987,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":4724,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1987,"record_offset":6312,"parent_record_offset":6284,"ancestor_record_offset":4724,"inlinee":11832,"source_line":324,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_capabilities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1987,"record_offset":5220,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":87462,"source_line":48,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_present_modes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1987,"record_offset":6188,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":87467,"source_line":31,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::create_swapchain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1987,"record_offset":8248,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":87475,"source_line":14,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::get_swapchain_images
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1987,"record_offset":8736,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":87476,"source_line":41,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::prelude::impl$0::assume_init_on_success::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5356,"parent_record_offset":5288,"ancestor_record_offset":4724,"inlinee":87466,"source_line":25,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::CompositeAlphaFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1987,"record_offset":7952,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":87472,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::SurfaceTransformFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1987,"record_offset":7632,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":87470,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::SwapchainCreateInfoKHR::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1987,"record_offset":8048,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":86852,"source_line":9099,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$2563::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1987,"record_offset":7992,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":87474,"source_line":28602,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5256,"parent_record_offset":5220,"ancestor_record_offset":4724,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5288,"parent_record_offset":5256,"ancestor_record_offset":4724,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5328,"parent_record_offset":5288,"ancestor_record_offset":4724,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$617::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1987,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":4724,"inlinee":86783,"source_line":1141,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1987,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":4724,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":6256,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":5572,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":9060,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":1987,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":4724,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1987,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":4724,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1987,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":4724,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1987,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":4724,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":8860,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":6284,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":6420,"parent_record_offset":6312,"ancestor_record_offset":4724,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":4724,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":4724,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1987,"record_offset":5384,"parent_record_offset":5356,"ancestor_record_offset":4724,"inlinee":11332,"source_line":650,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1987,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":4724,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":8020,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":5660,"parent_record_offset":5628,"ancestor_record_offset":4724,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":5688,"parent_record_offset":5628,"ancestor_record_offset":4724,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":6736,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1987,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":4724,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":6356,"parent_record_offset":6312,"ancestor_record_offset":4724,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":6608,"parent_record_offset":6568,"ancestor_record_offset":4724,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":4724,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":4724,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":5432,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":8764,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":5460,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1987,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":4724,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1987,"record_offset":6644,"parent_record_offset":6532,"ancestor_record_offset":4724,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1987,"record_offset":6476,"parent_record_offset":6448,"ancestor_record_offset":4724,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":4724,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":6568,"parent_record_offset":6532,"ancestor_record_offset":4724,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":4724,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":4724,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":5716,"parent_record_offset":5628,"ancestor_record_offset":4724,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":5604,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":4724,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":4724,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":5488,"parent_record_offset":4908,"ancestor_record_offset":4724,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::SwapchainInner::new_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":4908,"parent_record_offset":4872,"ancestor_record_offset":4724,"inlinee":87461,"source_line":136,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::impl$2::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":10240,"parent_record_offset":10188,"ancestor_record_offset":4724,"inlinee":87464,"source_line":130,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":6448,"parent_record_offset":6420,"ancestor_record_offset":4724,"inlinee":87468,"source_line":203,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::on_focus
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C74
// 首条位置: {"module":710,"inlinee":40052}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":1432,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":710,"record_offset":1188,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::on_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":710,"record_offset":1404,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":40054,"source_line":391,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":1604,"parent_record_offset":1484,"ancestor_record_offset":1160,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":1256,"parent_record_offset":1216,"ancestor_record_offset":1160,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":1160,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":1160,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":710,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":1160,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":1312,"parent_record_offset":1216,"ancestor_record_offset":1160,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":1520,"parent_record_offset":1484,"ancestor_record_offset":1160,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":1160,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":1484,"parent_record_offset":1456,"ancestor_record_offset":1160,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":710,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":1160,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":710,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":1160,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::desc
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1987,"record_offset":10912,"rva":25997568,"va":5394706688}
// 编译实例 RVAs: 0x18cb100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16782205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706688,"raw_hex":"e87d130001","size":5,"soft_fail":false,"target_va":5411488944,"va":5394706734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16784809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706688,"raw_hex":"e8a91d0001","size":5,"soft_fail":false,"target_va":5411491664,"va":5394706850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12883160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706688,"raw_hex":"e8286b3bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394706755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12882790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706688,"raw_hex":"e89a6c3bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394706801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":11376,"parent_record_offset":10912,"ancestor_record_offset":10912,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":11060,"parent_record_offset":10912,"ancestor_record_offset":10912,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1987,"record_offset":11196,"parent_record_offset":11112,"ancestor_record_offset":10912,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":11224,"parent_record_offset":11112,"ancestor_record_offset":10912,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":11544,"parent_record_offset":11428,"ancestor_record_offset":10912,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":11144,"parent_record_offset":11112,"ancestor_record_offset":10912,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":11248,"parent_record_offset":11224,"ancestor_record_offset":10912,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":11164,"parent_record_offset":11144,"ancestor_record_offset":10912,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":11568,"parent_record_offset":11544,"ancestor_record_offset":10912,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":11400,"parent_record_offset":11376,"ancestor_record_offset":10912,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":11284,"parent_record_offset":11080,"ancestor_record_offset":10912,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":11460,"parent_record_offset":11428,"ancestor_record_offset":10912,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":11112,"parent_record_offset":11080,"ancestor_record_offset":10912,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":11080,"parent_record_offset":11060,"ancestor_record_offset":10912,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":11428,"parent_record_offset":11400,"ancestor_record_offset":10912,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":11308,"parent_record_offset":11284,"ancestor_record_offset":10912,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":11484,"parent_record_offset":11460,"ancestor_record_offset":10912,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::resize
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1987,"record_offset":11616,"rva":25997744,"va":5394706864}
// 编译实例 RVAs: 0x18cb1b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12062600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706864,"raw_hex":"e878f047ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5394707123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25731213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706864,"raw_hex":"e8735f77fe","size":5,"soft_fail":false,"target_va":5368976080,"va":5394707288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19190965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706864,"raw_hex":"e84b2bdbfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5394706992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16777936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706864,"raw_hex":"e8d0020001","size":5,"soft_fail":false,"target_va":5411485168,"va":5394707227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16780337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706864,"raw_hex":"e8310c0001","size":5,"soft_fail":false,"target_va":5411487584,"va":5394707242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12883323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706864,"raw_hex":"e8856a3bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394706918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12883168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394706864,"raw_hex":"e8206b3bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394707179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":12372,"parent_record_offset":11616,"ancestor_record_offset":11616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":11768,"parent_record_offset":11616,"ancestor_record_offset":11616,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::resize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":11980,"parent_record_offset":11616,"ancestor_record_offset":11616,"inlinee":87477,"source_line":436,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":12096,"parent_record_offset":11980,"ancestor_record_offset":11616,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":12304,"parent_record_offset":11980,"ancestor_record_offset":11616,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":12332,"parent_record_offset":12304,"ancestor_record_offset":11616,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":12184,"parent_record_offset":12152,"ancestor_record_offset":11616,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":12212,"parent_record_offset":12152,"ancestor_record_offset":11616,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":12552,"parent_record_offset":12428,"ancestor_record_offset":11616,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":11832,"parent_record_offset":11796,"ancestor_record_offset":11616,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":12036,"parent_record_offset":12012,"ancestor_record_offset":11616,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":12576,"parent_record_offset":12552,"ancestor_record_offset":11616,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":11616,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":12060,"parent_record_offset":12036,"ancestor_record_offset":11616,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":12400,"parent_record_offset":12372,"ancestor_record_offset":11616,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":12240,"parent_record_offset":12152,"ancestor_record_offset":11616,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":12128,"parent_record_offset":11980,"ancestor_record_offset":11616,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":12152,"parent_record_offset":12128,"ancestor_record_offset":11616,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":11616,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":12012,"parent_record_offset":11980,"ancestor_record_offset":11616,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":11888,"parent_record_offset":11796,"ancestor_record_offset":11616,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":12464,"parent_record_offset":12428,"ancestor_record_offset":11616,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":11796,"parent_record_offset":11768,"ancestor_record_offset":11616,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":12428,"parent_record_offset":12400,"ancestor_record_offset":11616,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":11912,"parent_record_offset":11888,"ancestor_record_offset":11616,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":12492,"parent_record_offset":12464,"ancestor_record_offset":11616,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::set_vsync
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1987,"record_offset":12624,"rva":25998192,"va":5394707312}
// 编译实例 RVAs: 0x18cb370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12063046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707312,"raw_hex":"e8baee47ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5394707569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25731645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707312,"raw_hex":"e8c35d77fe","size":5,"soft_fail":false,"target_va":5368976080,"va":5394707720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19191411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707312,"raw_hex":"e88d29dbfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5394707438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16777491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707312,"raw_hex":"e813010001","size":5,"soft_fail":false,"target_va":5411485168,"va":5394707672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16779892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707312,"raw_hex":"e8740a0001","size":5,"soft_fail":false,"target_va":5411487584,"va":5394707687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12883770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707312,"raw_hex":"e8c6683bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394707365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12883613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707312,"raw_hex":"e863693bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394707624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":13384,"parent_record_offset":12624,"ancestor_record_offset":12624,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":12780,"parent_record_offset":12624,"ancestor_record_offset":12624,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::set_vsync
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":12992,"parent_record_offset":12624,"ancestor_record_offset":12624,"inlinee":87478,"source_line":441,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":13108,"parent_record_offset":12992,"ancestor_record_offset":12624,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":13316,"parent_record_offset":12992,"ancestor_record_offset":12624,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":13344,"parent_record_offset":13316,"ancestor_record_offset":12624,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":13196,"parent_record_offset":13164,"ancestor_record_offset":12624,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":13224,"parent_record_offset":13164,"ancestor_record_offset":12624,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":13564,"parent_record_offset":13440,"ancestor_record_offset":12624,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":12844,"parent_record_offset":12808,"ancestor_record_offset":12624,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":13048,"parent_record_offset":13024,"ancestor_record_offset":12624,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":13588,"parent_record_offset":13564,"ancestor_record_offset":12624,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":12868,"parent_record_offset":12844,"ancestor_record_offset":12624,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":13072,"parent_record_offset":13048,"ancestor_record_offset":12624,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":13412,"parent_record_offset":13384,"ancestor_record_offset":12624,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":13252,"parent_record_offset":13164,"ancestor_record_offset":12624,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":13140,"parent_record_offset":12992,"ancestor_record_offset":12624,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":13164,"parent_record_offset":13140,"ancestor_record_offset":12624,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":13276,"parent_record_offset":13252,"ancestor_record_offset":12624,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":13024,"parent_record_offset":12992,"ancestor_record_offset":12624,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":12900,"parent_record_offset":12808,"ancestor_record_offset":12624,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":13476,"parent_record_offset":13440,"ancestor_record_offset":12624,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":12808,"parent_record_offset":12780,"ancestor_record_offset":12624,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":13440,"parent_record_offset":13412,"ancestor_record_offset":12624,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":12624,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":13504,"parent_record_offset":13476,"ancestor_record_offset":12624,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::mark_stale
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C77
// 首条位置: {"module":710,"inlinee":40055}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":3016,"parent_record_offset":2680,"ancestor_record_offset":2680,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":710,"record_offset":2708,"parent_record_offset":2680,"ancestor_record_offset":2680,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":710,"record_offset":2924,"parent_record_offset":2680,"ancestor_record_offset":2680,"inlinee":40056,"source_line":446,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2680,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":2680,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":3196,"parent_record_offset":3072,"ancestor_record_offset":2680,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":2776,"parent_record_offset":2736,"ancestor_record_offset":2680,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":2680,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":2680,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":710,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":2680,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":2832,"parent_record_offset":2736,"ancestor_record_offset":2680,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":3108,"parent_record_offset":3072,"ancestor_record_offset":2680,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":2736,"parent_record_offset":2708,"ancestor_record_offset":2680,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":3072,"parent_record_offset":3044,"ancestor_record_offset":2680,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":710,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":2680,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":710,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":2680,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next
// 记录形式: compiled_procedure; instances=1
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1987,"record_offset":13636,"rva":25998624,"va":5394707744}
// 编译实例 RVAs: 0x18cb520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ash..vk..enums..Result$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12831064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e8a8363cff","size":5,"soft_fail":false,"target_va":5381879056,"va":5394710115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12064904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e878e747ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5394709427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25734856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e8385177fe","size":5,"soft_fail":false,"target_va":5368976080,"va":5394710931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21080506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e84656befe","size":5,"soft_fail":false,"target_va":5373629488,"va":5394709989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19627524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e8fc81d4fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5394709007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19629364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e8cc7ad4fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5394710863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19193269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e84b22dbfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5394709296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11869331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e86de34aff","size":5,"soft_fail":false,"target_va":5382840208,"va":5394709534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16830510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e82ed00001","size":5,"soft_fail":false,"target_va":5411540963,"va":5394710448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16831257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e819d30001","size":5,"soft_fail":false,"target_va":5411541584,"va":5394710322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16832644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e884d80001","size":5,"soft_fail":false,"target_va":5411542296,"va":5394709647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16833117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e85dda0001","size":5,"soft_fail":false,"target_va":5411543584,"va":5394710462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16832357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e865d70001","size":5,"soft_fail":false,"target_va":5411542880,"va":5394710518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e80fe4ffff","size":5,"soft_fail":false,"target_va":5394700736,"va":5394707884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16835401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e849e30001","size":5,"soft_fail":false,"target_va":5411543360,"va":5394707954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-42125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e8735bffff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394708488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16774873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e8d9f6ff00","size":5,"soft_fail":false,"target_va":5411485168,"va":5394710290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16777274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e83a000001","size":5,"soft_fail":false,"target_va":5411487584,"va":5394710305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12884217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e807673bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394707812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12885722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e826613bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394709733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13322905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e867b534ff","size":5,"soft_fail":false,"target_va":5381385536,"va":5394708436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13317691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e8c5c934ff","size":5,"soft_fail":false,"target_va":5381390640,"va":5394708326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13322719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e821b634ff","size":5,"soft_fail":false,"target_va":5381386480,"va":5394709194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13323137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e87fb434ff","size":5,"soft_fail":false,"target_va":5381384704,"va":5394707836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t17029444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e844d90301","size":5,"soft_fail":false,"target_va":5411737808,"va":5394708359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-37509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e87b6dffff","size":5,"soft_fail":false,"target_va":5394673152,"va":5394710656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16826726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394707744,"raw_hex":"e866c10001","size":5,"soft_fail":false,"target_va":5411537104,"va":5394710373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5394707744,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5394708645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":21352,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":13796,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::acquire_next_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":14008,"parent_record_offset":13636,"ancestor_record_offset":13636,"inlinee":87480,"source_line":472,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":18504,"parent_record_offset":18480,"ancestor_record_offset":13636,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":13636,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":21144,"parent_record_offset":21120,"ancestor_record_offset":13636,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":18480,"parent_record_offset":18452,"ancestor_record_offset":13636,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":21120,"parent_record_offset":21064,"ancestor_record_offset":13636,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1987,"record_offset":19992,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1987,"record_offset":20040,"parent_record_offset":20016,"ancestor_record_offset":13636,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":18392,"parent_record_offset":18364,"ancestor_record_offset":13636,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":21092,"parent_record_offset":21064,"ancestor_record_offset":13636,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":21064,"parent_record_offset":21040,"ancestor_record_offset":13636,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":18452,"parent_record_offset":18420,"ancestor_record_offset":13636,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":18420,"parent_record_offset":18392,"ancestor_record_offset":13636,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":21040,"parent_record_offset":21016,"ancestor_record_offset":13636,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1987,"record_offset":18328,"parent_record_offset":18300,"ancestor_record_offset":13636,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1987,"record_offset":18300,"parent_record_offset":18272,"ancestor_record_offset":13636,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1987,"record_offset":18244,"parent_record_offset":18216,"ancestor_record_offset":13636,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1987,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":13636,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1987,"record_offset":18216,"parent_record_offset":18188,"ancestor_record_offset":13636,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1987,"record_offset":18188,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1987,"record_offset":20876,"parent_record_offset":20824,"ancestor_record_offset":13636,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1987,"record_offset":20788,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1987,"record_offset":20824,"parent_record_offset":20788,"ancestor_record_offset":13636,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":18364,"parent_record_offset":18328,"ancestor_record_offset":13636,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":16720,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::acquire_next_image
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1987,"record_offset":16912,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":87481,"source_line":54,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16068,"parent_record_offset":16040,"ancestor_record_offset":13636,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16120,"parent_record_offset":16096,"ancestor_record_offset":13636,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16096,"parent_record_offset":16068,"ancestor_record_offset":13636,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16008,"parent_record_offset":15976,"ancestor_record_offset":13636,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16040,"parent_record_offset":16008,"ancestor_record_offset":13636,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16212,"parent_record_offset":16040,"ancestor_record_offset":13636,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16324,"parent_record_offset":16300,"ancestor_record_offset":13636,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":19668,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":20644,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":20904,"parent_record_offset":20824,"ancestor_record_offset":13636,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1987,"record_offset":20076,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1987,"record_offset":18604,"parent_record_offset":18580,"ancestor_record_offset":13636,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1987,"record_offset":16144,"parent_record_offset":16120,"ancestor_record_offset":13636,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1987,"record_offset":14404,"parent_record_offset":14372,"ancestor_record_offset":13636,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1987,"record_offset":15844,"parent_record_offset":15804,"ancestor_record_offset":13636,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":19952,"parent_record_offset":19924,"ancestor_record_offset":13636,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":19900,"parent_record_offset":19876,"ancestor_record_offset":13636,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":19924,"parent_record_offset":19900,"ancestor_record_offset":13636,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":20016,"parent_record_offset":19992,"ancestor_record_offset":13636,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":19756,"parent_record_offset":19724,"ancestor_record_offset":13636,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":19784,"parent_record_offset":19724,"ancestor_record_offset":13636,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1987,"record_offset":14652,"parent_record_offset":14624,"ancestor_record_offset":13636,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1987,"record_offset":15944,"parent_record_offset":15804,"ancestor_record_offset":13636,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1987,"record_offset":18580,"parent_record_offset":18328,"ancestor_record_offset":13636,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":16276,"parent_record_offset":15976,"ancestor_record_offset":13636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":13636,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":15092,"parent_record_offset":15068,"ancestor_record_offset":13636,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":15300,"parent_record_offset":15272,"ancestor_record_offset":13636,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":16168,"parent_record_offset":16144,"ancestor_record_offset":13636,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":20932,"parent_record_offset":20824,"ancestor_record_offset":13636,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1987,"record_offset":16748,"parent_record_offset":16720,"ancestor_record_offset":13636,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1987,"record_offset":16776,"parent_record_offset":16748,"ancestor_record_offset":13636,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":15044,"parent_record_offset":15020,"ancestor_record_offset":13636,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":13636,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":15372,"parent_record_offset":15344,"ancestor_record_offset":13636,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":15020,"parent_record_offset":14904,"ancestor_record_offset":13636,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1987,"record_offset":14372,"parent_record_offset":14344,"ancestor_record_offset":13636,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1987,"record_offset":14732,"parent_record_offset":14704,"ancestor_record_offset":13636,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1987,"record_offset":14872,"parent_record_offset":14840,"ancestor_record_offset":13636,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1987,"record_offset":14840,"parent_record_offset":14804,"ancestor_record_offset":13636,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":14780,"parent_record_offset":14756,"ancestor_record_offset":13636,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":14804,"parent_record_offset":14780,"ancestor_record_offset":13636,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":14756,"parent_record_offset":14732,"ancestor_record_offset":13636,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":15344,"parent_record_offset":14804,"ancestor_record_offset":13636,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14344,"parent_record_offset":14312,"ancestor_record_offset":13636,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14312,"parent_record_offset":14284,"ancestor_record_offset":13636,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14504,"parent_record_offset":14472,"ancestor_record_offset":13636,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14600,"parent_record_offset":14504,"ancestor_record_offset":13636,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":13636,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14284,"parent_record_offset":14248,"ancestor_record_offset":13636,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14624,"parent_record_offset":14600,"ancestor_record_offset":13636,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":14904,"parent_record_offset":14872,"ancestor_record_offset":13636,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":15188,"parent_record_offset":14904,"ancestor_record_offset":13636,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":15160,"parent_record_offset":14904,"ancestor_record_offset":13636,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":14964,"parent_record_offset":14904,"ancestor_record_offset":13636,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":17140,"parent_record_offset":17104,"ancestor_record_offset":13636,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":21532,"parent_record_offset":21408,"ancestor_record_offset":13636,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":13860,"parent_record_offset":13824,"ancestor_record_offset":13636,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":19608,"parent_record_offset":19584,"ancestor_record_offset":13636,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":17164,"parent_record_offset":17140,"ancestor_record_offset":13636,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":21556,"parent_record_offset":21532,"ancestor_record_offset":13636,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":13636,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":19632,"parent_record_offset":19608,"ancestor_record_offset":13636,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":21380,"parent_record_offset":21352,"ancestor_record_offset":13636,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":19812,"parent_record_offset":19724,"ancestor_record_offset":13636,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":19700,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":19724,"parent_record_offset":19700,"ancestor_record_offset":13636,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":19836,"parent_record_offset":19812,"ancestor_record_offset":13636,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":19584,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":13916,"parent_record_offset":13824,"ancestor_record_offset":13636,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":21444,"parent_record_offset":21408,"ancestor_record_offset":13636,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":13824,"parent_record_offset":13796,"ancestor_record_offset":13636,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":21408,"parent_record_offset":21380,"ancestor_record_offset":13636,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":13940,"parent_record_offset":13916,"ancestor_record_offset":13636,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":21472,"parent_record_offset":21444,"ancestor_record_offset":13636,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":15680,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":15720,"parent_record_offset":15680,"ancestor_record_offset":13636,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":14248,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":17316,"parent_record_offset":17276,"ancestor_record_offset":13636,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":17904,"parent_record_offset":17632,"ancestor_record_offset":13636,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1987,"record_offset":15872,"parent_record_offset":15844,"ancestor_record_offset":13636,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":15976,"parent_record_offset":15804,"ancestor_record_offset":13636,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":16248,"parent_record_offset":15976,"ancestor_record_offset":13636,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":14704,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::acquire_next_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":16688,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":39358,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::acquire_next_image_timeout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":19552,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":39372,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::present_err
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":21204,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":39364,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::present_out_of_date
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":20496,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":39361,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::swapchain_pending_recreation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":18156,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":39367,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1987,"record_offset":17080,"parent_record_offset":17052,"ancestor_record_offset":13636,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1987,"record_offset":17104,"parent_record_offset":17080,"ancestor_record_offset":13636,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":19876,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":40056,"source_line":446,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1987,"record_offset":17052,"parent_record_offset":14008,"ancestor_record_offset":13636,"inlinee":10030,"source_line":23,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1987,"record_offset":17204,"parent_record_offset":17052,"ancestor_record_offset":13636,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1987,"record_offset":15900,"parent_record_offset":15872,"ancestor_record_offset":13636,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1987,"record_offset":15804,"parent_record_offset":15760,"ancestor_record_offset":13636,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1987,"record_offset":15760,"parent_record_offset":15720,"ancestor_record_offset":13636,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::present
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1987,"record_offset":21604,"rva":26001840,"va":5394710960}
// 编译实例 RVAs: 0x18cc1b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12067919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8b1db47ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5394712442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25737318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e89a4777fe","size":5,"soft_fail":false,"target_va":5368976080,"va":5394713393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24555142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e87a5189fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5394713265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19196281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e88716dbfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5394712308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140befc70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13487389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8e33232ff","size":5,"soft_fail":false,"target_va":5381225584,"va":5394712968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140befd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13487161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8c73332ff","size":5,"soft_fail":false,"target_va":5381225824,"va":5394712980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c03d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13404263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8997733ff","size":5,"soft_fail":false,"target_va":5381307760,"va":5394712018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16828829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e89dc90001","size":5,"soft_fail":false,"target_va":5411541584,"va":5394712750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16829376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8c0cb0001","size":5,"soft_fail":false,"target_va":5411542296,"va":5394712915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16829001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e849ca0001","size":5,"soft_fail":false,"target_va":5411541904,"va":5394712898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e88ad7ffff","size":5,"soft_fail":false,"target_va":5394700736,"va":5394711089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16832196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8c4d60001","size":5,"soft_fail":false,"target_va":5411543360,"va":5394711159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-45330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8ee4effff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394711693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16779573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e835090001","size":5,"soft_fail":false,"target_va":5411492368,"va":5394712790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16782650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e83a150001","size":5,"soft_fail":false,"target_va":5411494800,"va":5394712145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16772442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e85aedff00","size":5,"soft_fail":false,"target_va":5411485168,"va":5394712721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16774843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8bbf6ff00","size":5,"soft_fail":false,"target_va":5411487584,"va":5394712736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12887434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8765a3bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394711029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12888099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8dd573bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394712110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13326107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8e5a834ff","size":5,"soft_fail":false,"target_va":5381385536,"va":5394711638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13320893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e843bd34ff","size":5,"soft_fail":false,"target_va":5381390640,"va":5394711528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13326162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8aea834ff","size":5,"soft_fail":false,"target_va":5381386480,"va":5394712637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13326361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8e7a734ff","size":5,"soft_fail":false,"target_va":5381384704,"va":5394711060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17026242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8c2cc0301","size":5,"soft_fail":false,"target_va":5411737808,"va":5394711561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-40054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e88a63ffff","size":5,"soft_fail":false,"target_va":5394673152,"va":5394713201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16824273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394710960,"raw_hex":"e8d1b70001","size":5,"soft_fail":false,"target_va":5411537104,"va":5394712826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":27448,"parent_record_offset":21604,"ancestor_record_offset":21604,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":21756,"parent_record_offset":21604,"ancestor_record_offset":21604,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::present_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":21968,"parent_record_offset":21604,"ancestor_record_offset":21604,"inlinee":87482,"source_line":557,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":26036,"parent_record_offset":26000,"ancestor_record_offset":21604,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::PresentInfoKHR::image_indices
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1987,"record_offset":25816,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":87484,"source_line":9161,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":23964,"parent_record_offset":23936,"ancestor_record_offset":21604,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":21604,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":23992,"parent_record_offset":23964,"ancestor_record_offset":21604,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":23904,"parent_record_offset":23872,"ancestor_record_offset":21604,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":23936,"parent_record_offset":23904,"ancestor_record_offset":21604,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":24108,"parent_record_offset":23936,"ancestor_record_offset":21604,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":24220,"parent_record_offset":24196,"ancestor_record_offset":21604,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":26672,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":27244,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":27304,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1987,"record_offset":24040,"parent_record_offset":24016,"ancestor_record_offset":21604,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1987,"record_offset":22300,"parent_record_offset":22268,"ancestor_record_offset":21604,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1987,"record_offset":23740,"parent_record_offset":23700,"ancestor_record_offset":21604,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":24672,"parent_record_offset":24648,"ancestor_record_offset":21604,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":24700,"parent_record_offset":24672,"ancestor_record_offset":21604,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":26760,"parent_record_offset":26728,"ancestor_record_offset":21604,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":26788,"parent_record_offset":26728,"ancestor_record_offset":21604,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1987,"record_offset":22548,"parent_record_offset":22520,"ancestor_record_offset":21604,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1987,"record_offset":23840,"parent_record_offset":23700,"ancestor_record_offset":21604,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":24172,"parent_record_offset":23872,"ancestor_record_offset":21604,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":21604,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":22988,"parent_record_offset":22964,"ancestor_record_offset":21604,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":23196,"parent_record_offset":23168,"ancestor_record_offset":21604,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":24064,"parent_record_offset":24040,"ancestor_record_offset":21604,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1987,"record_offset":26064,"parent_record_offset":26036,"ancestor_record_offset":21604,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1987,"record_offset":26092,"parent_record_offset":26064,"ancestor_record_offset":21604,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":22940,"parent_record_offset":22916,"ancestor_record_offset":21604,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":22964,"parent_record_offset":22940,"ancestor_record_offset":21604,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":23268,"parent_record_offset":23240,"ancestor_record_offset":21604,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":22916,"parent_record_offset":22800,"ancestor_record_offset":21604,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1987,"record_offset":22268,"parent_record_offset":22240,"ancestor_record_offset":21604,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1987,"record_offset":22628,"parent_record_offset":22600,"ancestor_record_offset":21604,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1987,"record_offset":22768,"parent_record_offset":22736,"ancestor_record_offset":21604,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1987,"record_offset":22736,"parent_record_offset":22700,"ancestor_record_offset":21604,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":22676,"parent_record_offset":22652,"ancestor_record_offset":21604,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":22700,"parent_record_offset":22676,"ancestor_record_offset":21604,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":22652,"parent_record_offset":22628,"ancestor_record_offset":21604,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":23240,"parent_record_offset":22700,"ancestor_record_offset":21604,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22240,"parent_record_offset":22208,"ancestor_record_offset":21604,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22208,"parent_record_offset":22180,"ancestor_record_offset":21604,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22400,"parent_record_offset":22368,"ancestor_record_offset":21604,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22496,"parent_record_offset":22400,"ancestor_record_offset":21604,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22368,"parent_record_offset":22344,"ancestor_record_offset":21604,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22180,"parent_record_offset":22144,"ancestor_record_offset":21604,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22520,"parent_record_offset":22496,"ancestor_record_offset":21604,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":22800,"parent_record_offset":22768,"ancestor_record_offset":21604,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":23084,"parent_record_offset":22800,"ancestor_record_offset":21604,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":23056,"parent_record_offset":22800,"ancestor_record_offset":21604,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":22860,"parent_record_offset":22800,"ancestor_record_offset":21604,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":26276,"parent_record_offset":26216,"ancestor_record_offset":21604,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":25908,"parent_record_offset":25872,"ancestor_record_offset":21604,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":27628,"parent_record_offset":27504,"ancestor_record_offset":21604,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":21820,"parent_record_offset":21784,"ancestor_record_offset":21604,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":26612,"parent_record_offset":26588,"ancestor_record_offset":21604,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":26300,"parent_record_offset":26276,"ancestor_record_offset":21604,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":21844,"parent_record_offset":21820,"ancestor_record_offset":21604,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":26636,"parent_record_offset":26612,"ancestor_record_offset":21604,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1987,"record_offset":25844,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1987,"record_offset":26192,"parent_record_offset":26168,"ancestor_record_offset":21604,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":27476,"parent_record_offset":27448,"ancestor_record_offset":21604,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":26816,"parent_record_offset":26728,"ancestor_record_offset":21604,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":26704,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":26728,"parent_record_offset":26704,"ancestor_record_offset":21604,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":26840,"parent_record_offset":26816,"ancestor_record_offset":21604,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":26588,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1987,"record_offset":25872,"parent_record_offset":25844,"ancestor_record_offset":21604,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1987,"record_offset":26216,"parent_record_offset":26192,"ancestor_record_offset":21604,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":21876,"parent_record_offset":21784,"ancestor_record_offset":21604,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":27540,"parent_record_offset":27504,"ancestor_record_offset":21604,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":21784,"parent_record_offset":21756,"ancestor_record_offset":21604,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":27504,"parent_record_offset":27476,"ancestor_record_offset":21604,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":21900,"parent_record_offset":21876,"ancestor_record_offset":21604,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":26248,"parent_record_offset":26216,"ancestor_record_offset":21604,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":23576,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":23616,"parent_record_offset":23576,"ancestor_record_offset":21604,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":22144,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":24816,"parent_record_offset":24780,"ancestor_record_offset":21604,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":25368,"parent_record_offset":25096,"ancestor_record_offset":21604,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1987,"record_offset":23768,"parent_record_offset":23740,"ancestor_record_offset":21604,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":23872,"parent_record_offset":23700,"ancestor_record_offset":21604,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":24144,"parent_record_offset":23872,"ancestor_record_offset":21604,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":22600,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::fullscreen_lost
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":26556,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":39363,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::present_err
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":27272,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":39364,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::present_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":24584,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":39359,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::present_out_of_date
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":26136,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":39361,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::present_retired
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":24616,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":39365,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::present_suboptimal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":26344,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":39362,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":24648,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":40056,"source_line":446,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::SubmissionEngine::queue
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":1987,"record_offset":26000,"parent_record_offset":21968,"ancestor_record_offset":21604,"inlinee":39754,"source_line":144,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1987,"record_offset":23796,"parent_record_offset":23768,"ancestor_record_offset":21604,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1987,"record_offset":23700,"parent_record_offset":23656,"ancestor_record_offset":21604,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1987,"record_offset":23656,"parent_record_offset":23616,"ancestor_record_offset":21604,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::retire
// 记录形式: compiled_procedure; instances=1
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1987,"record_offset":27700,"rva":26004304,"va":5394713424}
// 编译实例 RVAs: 0x18ccb50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12069317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394713424,"raw_hex":"e83bd647ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5394713840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25737969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394713424,"raw_hex":"e80f4577fe","size":5,"soft_fail":false,"target_va":5368976080,"va":5394714044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19197682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394713424,"raw_hex":"e80e11dbfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5394713709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16827900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394713424,"raw_hex":"e8fcc50001","size":5,"soft_fail":false,"target_va":5411541904,"va":5394713999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-47164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394713424,"raw_hex":"e8c447ffff","size":5,"soft_fail":false,"target_va":5394666368,"va":5394713527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16771231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394713424,"raw_hex":"e89fe8ff00","size":5,"soft_fail":false,"target_va":5411485168,"va":5394713932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16773632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394713424,"raw_hex":"e800f2ff00","size":5,"soft_fail":false,"target_va":5411487584,"va":5394713947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12889887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394713424,"raw_hex":"e8e1503bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394713482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12889863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394713424,"raw_hex":"e8f9503bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394713874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*2136(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5394713424,"raw_hex":"ff9358080000","size":6,"soft_fail":false,"va":5394713644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":28592,"parent_record_offset":27700,"ancestor_record_offset":27700,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":27852,"parent_record_offset":27700,"ancestor_record_offset":27700,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::retire
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":28064,"parent_record_offset":27700,"ancestor_record_offset":27700,"inlinee":87485,"source_line":638,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::ext::full_screen_exclusive::Device::release_full_screen_exclusive_mode
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/ext/full_screen_exclusive.rs
//     evidence={"module":1987,"record_offset":28184,"parent_record_offset":28064,"ancestor_record_offset":27700,"inlinee":87486,"source_line":20,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":28208,"parent_record_offset":28184,"ancestor_record_offset":27700,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":28232,"parent_record_offset":28208,"ancestor_record_offset":27700,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":28352,"parent_record_offset":28064,"ancestor_record_offset":27700,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":28560,"parent_record_offset":28064,"ancestor_record_offset":27700,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":28156,"parent_record_offset":28064,"ancestor_record_offset":27700,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":28440,"parent_record_offset":28408,"ancestor_record_offset":27700,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":28468,"parent_record_offset":28408,"ancestor_record_offset":27700,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":28772,"parent_record_offset":28648,"ancestor_record_offset":27700,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":27916,"parent_record_offset":27880,"ancestor_record_offset":27700,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":28292,"parent_record_offset":28268,"ancestor_record_offset":27700,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":28796,"parent_record_offset":28772,"ancestor_record_offset":27700,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":27940,"parent_record_offset":27916,"ancestor_record_offset":27700,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":28316,"parent_record_offset":28292,"ancestor_record_offset":27700,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":28620,"parent_record_offset":28592,"ancestor_record_offset":27700,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":28496,"parent_record_offset":28408,"ancestor_record_offset":27700,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":28384,"parent_record_offset":28064,"ancestor_record_offset":27700,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":28408,"parent_record_offset":28384,"ancestor_record_offset":27700,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":28520,"parent_record_offset":28496,"ancestor_record_offset":27700,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":28268,"parent_record_offset":28064,"ancestor_record_offset":27700,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":27972,"parent_record_offset":27880,"ancestor_record_offset":27700,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":28684,"parent_record_offset":28648,"ancestor_record_offset":27700,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":27880,"parent_record_offset":27852,"ancestor_record_offset":27700,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":28648,"parent_record_offset":28620,"ancestor_record_offset":27700,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":27996,"parent_record_offset":27972,"ancestor_record_offset":27700,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":28712,"parent_record_offset":28684,"ancestor_record_offset":27700,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::leek::swapchain_retire
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":28124,"parent_record_offset":28064,"ancestor_record_offset":27700,"inlinee":39366,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$1::dispose
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1987,"record_offset":28844,"rva":26004944,"va":5394714064}
// 编译实例 RVAs: 0x18ccdd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25738626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394714064,"raw_hex":"e87e4277fe","size":5,"soft_fail":false,"target_va":5368976080,"va":5394714701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140879c00
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17118362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394714064,"raw_hex":"e866cbfafe","size":5,"soft_fail":false,"target_va":5377596416,"va":5394714773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-20358622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394714064,"raw_hex":"e8225ac9fe","size":5,"soft_fail":false,"target_va":5374355568,"va":5394714185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16827239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394714064,"raw_hex":"e867c30001","size":5,"soft_fail":false,"target_va":5411541904,"va":5394714660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16770584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394714064,"raw_hex":"e818e6ff00","size":5,"soft_fail":false,"target_va":5411485168,"va":5394714579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12890525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394714064,"raw_hex":"e8634e3bff","size":5,"soft_fail":false,"target_va":5381823600,"va":5394714120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12890515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394714064,"raw_hex":"e86d4e3bff","size":5,"soft_fail":false,"target_va":5381824016,"va":5394714526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t16772969","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5394714064,"raw_hex":"e969efff00","size":5,"soft_fail":false,"target_va":5411487584,"va":5394714610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":31068,"parent_record_offset":28844,"ancestor_record_offset":28844,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":28988,"parent_record_offset":28844,"ancestor_record_offset":28844,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::dispose
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":29200,"parent_record_offset":28844,"ancestor_record_offset":28844,"inlinee":87487,"source_line":670,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":29400,"parent_record_offset":29376,"ancestor_record_offset":28844,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":29448,"parent_record_offset":29424,"ancestor_record_offset":28844,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":29424,"parent_record_offset":29400,"ancestor_record_offset":28844,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29376,"parent_record_offset":29340,"ancestor_record_offset":28844,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29568,"parent_record_offset":29528,"ancestor_record_offset":28844,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29528,"parent_record_offset":29200,"ancestor_record_offset":28844,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29680,"parent_record_offset":29528,"ancestor_record_offset":28844,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29340,"parent_record_offset":29316,"ancestor_record_offset":28844,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29708,"parent_record_offset":29528,"ancestor_record_offset":28844,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1987,"record_offset":29824,"parent_record_offset":29200,"ancestor_record_offset":28844,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1987,"record_offset":29960,"parent_record_offset":29936,"ancestor_record_offset":28844,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":31036,"parent_record_offset":29200,"ancestor_record_offset":28844,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":29936,"parent_record_offset":29824,"ancestor_record_offset":28844,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1987,"record_offset":29784,"parent_record_offset":29760,"ancestor_record_offset":28844,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":29900,"parent_record_offset":29856,"ancestor_record_offset":28844,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":29984,"parent_record_offset":29960,"ancestor_record_offset":28844,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":29488,"parent_record_offset":29340,"ancestor_record_offset":28844,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1987,"record_offset":29736,"parent_record_offset":29528,"ancestor_record_offset":28844,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1987,"record_offset":29760,"parent_record_offset":29736,"ancestor_record_offset":28844,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":29856,"parent_record_offset":29824,"ancestor_record_offset":28844,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":31248,"parent_record_offset":31124,"ancestor_record_offset":28844,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":29052,"parent_record_offset":29016,"ancestor_record_offset":28844,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":31272,"parent_record_offset":31248,"ancestor_record_offset":28844,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":29076,"parent_record_offset":29052,"ancestor_record_offset":28844,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1987,"record_offset":31096,"parent_record_offset":31068,"ancestor_record_offset":28844,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":29108,"parent_record_offset":29016,"ancestor_record_offset":28844,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":31160,"parent_record_offset":31124,"ancestor_record_offset":28844,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":29016,"parent_record_offset":28988,"ancestor_record_offset":28844,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1987,"record_offset":31124,"parent_record_offset":31096,"ancestor_record_offset":28844,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":29132,"parent_record_offset":29108,"ancestor_record_offset":28844,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":31188,"parent_record_offset":31160,"ancestor_record_offset":28844,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1987,"record_offset":29292,"parent_record_offset":29200,"ancestor_record_offset":28844,"inlinee":39247,"source_line":143,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$7::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1987,"record_offset":29316,"parent_record_offset":29292,"ancestor_record_offset":28844,"inlinee":86797,"source_line":310,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$8::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1987,"record_offset":30056,"parent_record_offset":30024,"ancestor_record_offset":28844,"inlinee":86799,"source_line":316,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155A4
// 首条位置: {"module":1987,"inlinee":87460}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":10188,"parent_record_offset":4872,"ancestor_record_offset":4872,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::new_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":4908,"parent_record_offset":4872,"ancestor_record_offset":4872,"inlinee":87461,"source_line":136,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":4872,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":4872,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":4872,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5780,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5804,"parent_record_offset":5780,"ancestor_record_offset":4872,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":6964,"parent_record_offset":6880,"ancestor_record_offset":4872,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":6908,"parent_record_offset":6880,"ancestor_record_offset":4872,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":4872,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":4872,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1987,"record_offset":6160,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":4872,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":8804,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1987,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":4872,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1987,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":4872,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1987,"record_offset":6312,"parent_record_offset":6284,"ancestor_record_offset":4872,"inlinee":11832,"source_line":324,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_capabilities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1987,"record_offset":5220,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":87462,"source_line":48,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_present_modes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1987,"record_offset":6188,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":87467,"source_line":31,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::create_swapchain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1987,"record_offset":8248,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":87475,"source_line":14,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::get_swapchain_images
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1987,"record_offset":8736,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":87476,"source_line":41,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::prelude::impl$0::assume_init_on_success::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5356,"parent_record_offset":5288,"ancestor_record_offset":4872,"inlinee":87466,"source_line":25,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::CompositeAlphaFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1987,"record_offset":7952,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":87472,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::SurfaceTransformFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1987,"record_offset":7632,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":87470,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::SwapchainCreateInfoKHR::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1987,"record_offset":8048,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":86852,"source_line":9099,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$2563::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1987,"record_offset":7992,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":87474,"source_line":28602,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5256,"parent_record_offset":5220,"ancestor_record_offset":4872,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5288,"parent_record_offset":5256,"ancestor_record_offset":4872,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5328,"parent_record_offset":5288,"ancestor_record_offset":4872,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$617::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1987,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":4872,"inlinee":86783,"source_line":1141,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1987,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":4872,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":6256,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":5572,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":9060,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":1987,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":4872,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1987,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":4872,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1987,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":4872,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1987,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":4872,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":8860,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":6284,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":6420,"parent_record_offset":6312,"ancestor_record_offset":4872,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":4872,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":4872,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1987,"record_offset":5384,"parent_record_offset":5356,"ancestor_record_offset":4872,"inlinee":11332,"source_line":650,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1987,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":4872,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":8020,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":5660,"parent_record_offset":5628,"ancestor_record_offset":4872,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":5688,"parent_record_offset":5628,"ancestor_record_offset":4872,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":6736,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1987,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":4872,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":6356,"parent_record_offset":6312,"ancestor_record_offset":4872,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":6608,"parent_record_offset":6568,"ancestor_record_offset":4872,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":4872,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":4872,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":5432,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":8764,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":5460,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1987,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":4872,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1987,"record_offset":6644,"parent_record_offset":6532,"ancestor_record_offset":4872,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1987,"record_offset":6476,"parent_record_offset":6448,"ancestor_record_offset":4872,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":4872,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":6568,"parent_record_offset":6532,"ancestor_record_offset":4872,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":4872,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":4872,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":5716,"parent_record_offset":5628,"ancestor_record_offset":4872,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":5604,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":4872,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":4872,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":5488,"parent_record_offset":4908,"ancestor_record_offset":4872,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::impl$2::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":10240,"parent_record_offset":10188,"ancestor_record_offset":4872,"inlinee":87464,"source_line":130,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":6448,"parent_record_offset":6420,"ancestor_record_offset":4872,"inlinee":87468,"source_line":203,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155A8
// 首条位置: {"module":1987,"inlinee":87464}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":10368,"parent_record_offset":10240,"ancestor_record_offset":10240,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":10576,"parent_record_offset":10240,"ancestor_record_offset":10240,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":10400,"parent_record_offset":10240,"ancestor_record_offset":10240,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":10284,"parent_record_offset":10240,"ancestor_record_offset":10240,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":10240,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":10752,"parent_record_offset":10696,"ancestor_record_offset":10240,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":10724,"parent_record_offset":10696,"ancestor_record_offset":10240,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":10696,"parent_record_offset":10672,"ancestor_record_offset":10240,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":10672,"parent_record_offset":10648,"ancestor_record_offset":10240,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":10456,"parent_record_offset":10424,"ancestor_record_offset":10240,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":10484,"parent_record_offset":10424,"ancestor_record_offset":10240,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":10600,"parent_record_offset":10576,"ancestor_record_offset":10240,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":10240,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":10332,"parent_record_offset":10308,"ancestor_record_offset":10240,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":10512,"parent_record_offset":10424,"ancestor_record_offset":10240,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":10424,"parent_record_offset":10400,"ancestor_record_offset":10240,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":10240,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::new_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155A5
// 首条位置: {"module":1987,"inlinee":87461}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5780,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1987,"record_offset":6160,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":8804,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_capabilities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1987,"record_offset":5220,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":87462,"source_line":48,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_present_modes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1987,"record_offset":6188,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":87467,"source_line":31,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::swapchain::Device::create_swapchain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1987,"record_offset":8248,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":87475,"source_line":14,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::swapchain::Device::get_swapchain_images
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1987,"record_offset":8736,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":87476,"source_line":41,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::CompositeAlphaFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1987,"record_offset":7952,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":87472,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::SurfaceTransformFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1987,"record_offset":7632,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":87470,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::SwapchainCreateInfoKHR::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1987,"record_offset":8048,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":86852,"source_line":9099,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2563::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1987,"record_offset":7992,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":87474,"source_line":28602,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":6256,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":5572,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":9060,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":8860,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":6284,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":8020,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":6736,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":5432,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":8764,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":5460,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":5604,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":5488,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":4908,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":4908,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":4908,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":5804,"parent_record_offset":5780,"ancestor_record_offset":4908,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":6964,"parent_record_offset":6880,"ancestor_record_offset":4908,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":6908,"parent_record_offset":6880,"ancestor_record_offset":4908,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":4908,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":4908,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":4908,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1987,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":4908,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1987,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":4908,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1987,"record_offset":6312,"parent_record_offset":6284,"ancestor_record_offset":4908,"inlinee":11832,"source_line":324,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::prelude::impl$0::assume_init_on_success::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5356,"parent_record_offset":5288,"ancestor_record_offset":4908,"inlinee":87466,"source_line":25,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5256,"parent_record_offset":5220,"ancestor_record_offset":4908,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5288,"parent_record_offset":5256,"ancestor_record_offset":4908,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":5328,"parent_record_offset":5288,"ancestor_record_offset":4908,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$617::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1987,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":4908,"inlinee":86783,"source_line":1141,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1987,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":4908,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":1987,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":4908,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1987,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":4908,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1987,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":4908,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1987,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":4908,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":6420,"parent_record_offset":6312,"ancestor_record_offset":4908,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":4908,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1987,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":4908,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1987,"record_offset":5384,"parent_record_offset":5356,"ancestor_record_offset":4908,"inlinee":11332,"source_line":650,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1987,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":4908,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":5660,"parent_record_offset":5628,"ancestor_record_offset":4908,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":5688,"parent_record_offset":5628,"ancestor_record_offset":4908,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":4908,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1987,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":4908,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":6356,"parent_record_offset":6312,"ancestor_record_offset":4908,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":6608,"parent_record_offset":6568,"ancestor_record_offset":4908,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":4908,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":4908,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1987,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":4908,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1987,"record_offset":6644,"parent_record_offset":6532,"ancestor_record_offset":4908,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1987,"record_offset":6476,"parent_record_offset":6448,"ancestor_record_offset":4908,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":4908,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":6568,"parent_record_offset":6532,"ancestor_record_offset":4908,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":4908,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":4908,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":5716,"parent_record_offset":5628,"ancestor_record_offset":4908,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":4908,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":4908,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":6448,"parent_record_offset":6420,"ancestor_record_offset":4908,"inlinee":87468,"source_line":203,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155AC
// 首条位置: {"module":1987,"inlinee":87468}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1987,"record_offset":6476,"parent_record_offset":6448,"ancestor_record_offset":6448,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$617::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1987,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":6448,"inlinee":86783,"source_line":1141,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":6608,"parent_record_offset":6568,"ancestor_record_offset":6448,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1987,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":6448,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1987,"record_offset":6644,"parent_record_offset":6532,"ancestor_record_offset":6448,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":6448,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":6568,"parent_record_offset":6532,"ancestor_record_offset":6448,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1BBBE
// 首条位置: {"module":3273,"inlinee":113598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3273,"record_offset":1660,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":3273,"record_offset":1132,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":15367,"source_line":298,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-05a13811c738e3a6/4031d67/src/lib.rs
//     evidence={"module":3273,"record_offset":1532,"parent_record_offset":1488,"ancestor_record_offset":1068,"inlinee":16141,"source_line":104,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-05a13811c738e3a6/4031d67/src/lib.rs
//     evidence={"module":3273,"record_offset":1488,"parent_record_offset":1220,"ancestor_record_offset":1068,"inlinee":39243,"source_line":225,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3273,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":1068,"inlinee":27245,"source_line":39,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3273,"record_offset":1392,"parent_record_offset":1364,"ancestor_record_offset":1068,"inlinee":27247,"source_line":3874,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3273,"record_offset":1616,"parent_record_offset":1488,"ancestor_record_offset":1068,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3273,"record_offset":1340,"parent_record_offset":1220,"ancestor_record_offset":1068,"inlinee":32600,"source_line":38,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":1432,"parent_record_offset":1220,"ancestor_record_offset":1068,"inlinee":39241,"source_line":1779,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":1284,"parent_record_offset":1220,"ancestor_record_offset":1068,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":1068,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":1068,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3273,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":1068,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":3273,"record_offset":1220,"parent_record_offset":1180,"ancestor_record_offset":1068,"inlinee":39240,"source_line":71,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$6::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":3273,"record_offset":1180,"parent_record_offset":1132,"ancestor_record_offset":1068,"inlinee":39274,"source_line":369,"archive":"_pdb/B/modules/03273.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 348
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152CF
// 首条位置: {"module":1935,"inlinee":86735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::create_semaphore
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1936,"record_offset":24684,"parent_record_offset":24620,"ancestor_record_offset":24620,"inlinee":39751,"source_line":2129,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$349::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1936,"record_offset":24656,"parent_record_offset":24620,"ancestor_record_offset":24620,"inlinee":86732,"source_line":7615,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1936,"record_offset":24800,"parent_record_offset":24620,"ancestor_record_offset":24620,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1936,"record_offset":24712,"parent_record_offset":24684,"ancestor_record_offset":24620,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1936,"record_offset":24736,"parent_record_offset":24712,"ancestor_record_offset":24620,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1936,"record_offset":24760,"parent_record_offset":24736,"ancestor_record_offset":24620,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 359
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152CA
// 首条位置: {"module":1935,"inlinee":86730}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::create_semaphore
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1936,"record_offset":27364,"parent_record_offset":27300,"ancestor_record_offset":27300,"inlinee":39751,"source_line":2129,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$349::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1936,"record_offset":27336,"parent_record_offset":27300,"ancestor_record_offset":27300,"inlinee":86732,"source_line":7615,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1936,"record_offset":27480,"parent_record_offset":27300,"ancestor_record_offset":27300,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1936,"record_offset":27392,"parent_record_offset":27364,"ancestor_record_offset":27300,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1936,"record_offset":27416,"parent_record_offset":27392,"ancestor_record_offset":27300,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1936,"record_offset":27440,"parent_record_offset":27416,"ancestor_record_offset":27300,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01936.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::on_focus
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 391
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C76
// 首条位置: {"module":710,"inlinee":40054}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::resize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 436
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155B5
// 首条位置: {"module":1987,"inlinee":87477}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":12096,"parent_record_offset":11980,"ancestor_record_offset":11980,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":12304,"parent_record_offset":11980,"ancestor_record_offset":11980,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":12128,"parent_record_offset":11980,"ancestor_record_offset":11980,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":12012,"parent_record_offset":11980,"ancestor_record_offset":11980,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":12332,"parent_record_offset":12304,"ancestor_record_offset":11980,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":12184,"parent_record_offset":12152,"ancestor_record_offset":11980,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":12212,"parent_record_offset":12152,"ancestor_record_offset":11980,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":12036,"parent_record_offset":12012,"ancestor_record_offset":11980,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":12060,"parent_record_offset":12036,"ancestor_record_offset":11980,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":12240,"parent_record_offset":12152,"ancestor_record_offset":11980,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":12152,"parent_record_offset":12128,"ancestor_record_offset":11980,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":11980,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::set_vsync
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 441
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155B6
// 首条位置: {"module":1987,"inlinee":87478}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":13108,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":13316,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":13140,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":13024,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":13344,"parent_record_offset":13316,"ancestor_record_offset":12992,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":13196,"parent_record_offset":13164,"ancestor_record_offset":12992,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":13224,"parent_record_offset":13164,"ancestor_record_offset":12992,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":13048,"parent_record_offset":13024,"ancestor_record_offset":12992,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":13072,"parent_record_offset":13048,"ancestor_record_offset":12992,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":13252,"parent_record_offset":13164,"ancestor_record_offset":12992,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":13164,"parent_record_offset":13140,"ancestor_record_offset":12992,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":13276,"parent_record_offset":13252,"ancestor_record_offset":12992,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 446
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C78
// 首条位置: {"module":710,"inlinee":40056}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2924,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":19952,"parent_record_offset":19924,"ancestor_record_offset":19876,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":2924,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::set_fullscreen_exclusive
// 记录形式: inlinee_declaration; instances=1
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155B7
// 首条位置: {"module":1987,"inlinee":87479}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::acquire_next_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 472
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155B8
// 首条位置: {"module":1987,"inlinee":87480}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1987,"record_offset":19992,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1987,"record_offset":18188,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1987,"record_offset":20788,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":16720,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::swapchain::Device::acquire_next_image
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1987,"record_offset":16912,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":87481,"source_line":54,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":19668,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":20644,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1987,"record_offset":20076,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":17236,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":19700,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":19584,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":15680,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":14248,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":14704,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::acquire_next_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":16688,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":39358,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::acquire_next_image_timeout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":19552,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":39372,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::present_err
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":21204,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":39364,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::present_out_of_date
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":20496,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":39361,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::swapchain_pending_recreation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":18156,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":39367,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":19876,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":40056,"source_line":446,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1987,"record_offset":17052,"parent_record_offset":14008,"ancestor_record_offset":14008,"inlinee":10030,"source_line":23,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":18504,"parent_record_offset":18480,"ancestor_record_offset":14008,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":14008,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":21144,"parent_record_offset":21120,"ancestor_record_offset":14008,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":18480,"parent_record_offset":18452,"ancestor_record_offset":14008,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1987,"record_offset":21120,"parent_record_offset":21064,"ancestor_record_offset":14008,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1987,"record_offset":20040,"parent_record_offset":20016,"ancestor_record_offset":14008,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":18392,"parent_record_offset":18364,"ancestor_record_offset":14008,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":21092,"parent_record_offset":21064,"ancestor_record_offset":14008,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":21064,"parent_record_offset":21040,"ancestor_record_offset":14008,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":18452,"parent_record_offset":18420,"ancestor_record_offset":14008,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":18420,"parent_record_offset":18392,"ancestor_record_offset":14008,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":21040,"parent_record_offset":21016,"ancestor_record_offset":14008,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1987,"record_offset":18328,"parent_record_offset":18300,"ancestor_record_offset":14008,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1987,"record_offset":18300,"parent_record_offset":18272,"ancestor_record_offset":14008,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1987,"record_offset":18244,"parent_record_offset":18216,"ancestor_record_offset":14008,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1987,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":14008,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1987,"record_offset":18216,"parent_record_offset":18188,"ancestor_record_offset":14008,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1987,"record_offset":20876,"parent_record_offset":20824,"ancestor_record_offset":14008,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1987,"record_offset":20824,"parent_record_offset":20788,"ancestor_record_offset":14008,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":18364,"parent_record_offset":18328,"ancestor_record_offset":14008,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16068,"parent_record_offset":16040,"ancestor_record_offset":14008,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16120,"parent_record_offset":16096,"ancestor_record_offset":14008,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16096,"parent_record_offset":16068,"ancestor_record_offset":14008,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16008,"parent_record_offset":15976,"ancestor_record_offset":14008,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16040,"parent_record_offset":16008,"ancestor_record_offset":14008,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16212,"parent_record_offset":16040,"ancestor_record_offset":14008,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":16324,"parent_record_offset":16300,"ancestor_record_offset":14008,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":20904,"parent_record_offset":20824,"ancestor_record_offset":14008,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1987,"record_offset":18604,"parent_record_offset":18580,"ancestor_record_offset":14008,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1987,"record_offset":16144,"parent_record_offset":16120,"ancestor_record_offset":14008,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1987,"record_offset":14404,"parent_record_offset":14372,"ancestor_record_offset":14008,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1987,"record_offset":15844,"parent_record_offset":15804,"ancestor_record_offset":14008,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":19952,"parent_record_offset":19924,"ancestor_record_offset":14008,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":19900,"parent_record_offset":19876,"ancestor_record_offset":14008,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":19924,"parent_record_offset":19900,"ancestor_record_offset":14008,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":20016,"parent_record_offset":19992,"ancestor_record_offset":14008,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":19756,"parent_record_offset":19724,"ancestor_record_offset":14008,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":19784,"parent_record_offset":19724,"ancestor_record_offset":14008,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1987,"record_offset":14652,"parent_record_offset":14624,"ancestor_record_offset":14008,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1987,"record_offset":15944,"parent_record_offset":15804,"ancestor_record_offset":14008,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1987,"record_offset":18580,"parent_record_offset":18328,"ancestor_record_offset":14008,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":16276,"parent_record_offset":15976,"ancestor_record_offset":14008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":14008,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":15092,"parent_record_offset":15068,"ancestor_record_offset":14008,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":15300,"parent_record_offset":15272,"ancestor_record_offset":14008,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":16168,"parent_record_offset":16144,"ancestor_record_offset":14008,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1987,"record_offset":20932,"parent_record_offset":20824,"ancestor_record_offset":14008,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1987,"record_offset":16748,"parent_record_offset":16720,"ancestor_record_offset":14008,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1987,"record_offset":16776,"parent_record_offset":16748,"ancestor_record_offset":14008,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":15044,"parent_record_offset":15020,"ancestor_record_offset":14008,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":14008,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":15372,"parent_record_offset":15344,"ancestor_record_offset":14008,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":15020,"parent_record_offset":14904,"ancestor_record_offset":14008,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1987,"record_offset":14372,"parent_record_offset":14344,"ancestor_record_offset":14008,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1987,"record_offset":14732,"parent_record_offset":14704,"ancestor_record_offset":14008,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1987,"record_offset":14872,"parent_record_offset":14840,"ancestor_record_offset":14008,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1987,"record_offset":14840,"parent_record_offset":14804,"ancestor_record_offset":14008,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":14780,"parent_record_offset":14756,"ancestor_record_offset":14008,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":14804,"parent_record_offset":14780,"ancestor_record_offset":14008,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":14756,"parent_record_offset":14732,"ancestor_record_offset":14008,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":15344,"parent_record_offset":14804,"ancestor_record_offset":14008,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14344,"parent_record_offset":14312,"ancestor_record_offset":14008,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14312,"parent_record_offset":14284,"ancestor_record_offset":14008,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14504,"parent_record_offset":14472,"ancestor_record_offset":14008,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14600,"parent_record_offset":14504,"ancestor_record_offset":14008,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":14008,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14284,"parent_record_offset":14248,"ancestor_record_offset":14008,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":14624,"parent_record_offset":14600,"ancestor_record_offset":14008,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":14904,"parent_record_offset":14872,"ancestor_record_offset":14008,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":15188,"parent_record_offset":14904,"ancestor_record_offset":14008,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":15160,"parent_record_offset":14904,"ancestor_record_offset":14008,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":14964,"parent_record_offset":14904,"ancestor_record_offset":14008,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":17140,"parent_record_offset":17104,"ancestor_record_offset":14008,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":19608,"parent_record_offset":19584,"ancestor_record_offset":14008,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":17164,"parent_record_offset":17140,"ancestor_record_offset":14008,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":19632,"parent_record_offset":19608,"ancestor_record_offset":14008,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":19812,"parent_record_offset":19724,"ancestor_record_offset":14008,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":19724,"parent_record_offset":19700,"ancestor_record_offset":14008,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":19836,"parent_record_offset":19812,"ancestor_record_offset":14008,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":15720,"parent_record_offset":15680,"ancestor_record_offset":14008,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":17316,"parent_record_offset":17276,"ancestor_record_offset":14008,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":17904,"parent_record_offset":17632,"ancestor_record_offset":14008,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1987,"record_offset":15872,"parent_record_offset":15844,"ancestor_record_offset":14008,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":15976,"parent_record_offset":15804,"ancestor_record_offset":14008,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":16248,"parent_record_offset":15976,"ancestor_record_offset":14008,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1987,"record_offset":17080,"parent_record_offset":17052,"ancestor_record_offset":14008,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1987,"record_offset":17104,"parent_record_offset":17080,"ancestor_record_offset":14008,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1987,"record_offset":17204,"parent_record_offset":17052,"ancestor_record_offset":14008,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1987,"record_offset":15900,"parent_record_offset":15872,"ancestor_record_offset":14008,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1987,"record_offset":15804,"parent_record_offset":15760,"ancestor_record_offset":14008,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1987,"record_offset":15760,"parent_record_offset":15720,"ancestor_record_offset":14008,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::present_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 557
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155BA
// 首条位置: {"module":1987,"inlinee":87482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::definitions::PresentInfoKHR::image_indices
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1987,"record_offset":25816,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":87484,"source_line":9161,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":26672,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":27244,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":27304,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":24740,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1987,"record_offset":25844,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":26704,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":26588,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":23576,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":22144,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":22600,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::fullscreen_lost
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":26556,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":39363,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::present_err
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":27272,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":39364,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::present_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":24584,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":39359,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::present_out_of_date
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":26136,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":39361,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::present_retired
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":24616,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":39365,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::present_suboptimal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":26344,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":39362,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1987,"record_offset":24648,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":40056,"source_line":446,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::queue
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":1987,"record_offset":26000,"parent_record_offset":21968,"ancestor_record_offset":21968,"inlinee":39754,"source_line":144,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":26036,"parent_record_offset":26000,"ancestor_record_offset":21968,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":23964,"parent_record_offset":23936,"ancestor_record_offset":21968,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":21968,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":23992,"parent_record_offset":23964,"ancestor_record_offset":21968,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":23904,"parent_record_offset":23872,"ancestor_record_offset":21968,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":23936,"parent_record_offset":23904,"ancestor_record_offset":21968,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":24108,"parent_record_offset":23936,"ancestor_record_offset":21968,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1987,"record_offset":24220,"parent_record_offset":24196,"ancestor_record_offset":21968,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1987,"record_offset":24040,"parent_record_offset":24016,"ancestor_record_offset":21968,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1987,"record_offset":22300,"parent_record_offset":22268,"ancestor_record_offset":21968,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1987,"record_offset":23740,"parent_record_offset":23700,"ancestor_record_offset":21968,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":24672,"parent_record_offset":24648,"ancestor_record_offset":21968,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":24700,"parent_record_offset":24672,"ancestor_record_offset":21968,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":26760,"parent_record_offset":26728,"ancestor_record_offset":21968,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":26788,"parent_record_offset":26728,"ancestor_record_offset":21968,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1987,"record_offset":22548,"parent_record_offset":22520,"ancestor_record_offset":21968,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1987,"record_offset":23840,"parent_record_offset":23700,"ancestor_record_offset":21968,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":24172,"parent_record_offset":23872,"ancestor_record_offset":21968,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":21968,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":22988,"parent_record_offset":22964,"ancestor_record_offset":21968,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":23196,"parent_record_offset":23168,"ancestor_record_offset":21968,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":24064,"parent_record_offset":24040,"ancestor_record_offset":21968,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1987,"record_offset":26064,"parent_record_offset":26036,"ancestor_record_offset":21968,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1987,"record_offset":26092,"parent_record_offset":26064,"ancestor_record_offset":21968,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":22940,"parent_record_offset":22916,"ancestor_record_offset":21968,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":22964,"parent_record_offset":22940,"ancestor_record_offset":21968,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":23268,"parent_record_offset":23240,"ancestor_record_offset":21968,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":22916,"parent_record_offset":22800,"ancestor_record_offset":21968,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1987,"record_offset":22268,"parent_record_offset":22240,"ancestor_record_offset":21968,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1987,"record_offset":22628,"parent_record_offset":22600,"ancestor_record_offset":21968,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1987,"record_offset":22768,"parent_record_offset":22736,"ancestor_record_offset":21968,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1987,"record_offset":22736,"parent_record_offset":22700,"ancestor_record_offset":21968,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":22676,"parent_record_offset":22652,"ancestor_record_offset":21968,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":22700,"parent_record_offset":22676,"ancestor_record_offset":21968,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":22652,"parent_record_offset":22628,"ancestor_record_offset":21968,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1987,"record_offset":23240,"parent_record_offset":22700,"ancestor_record_offset":21968,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22240,"parent_record_offset":22208,"ancestor_record_offset":21968,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22208,"parent_record_offset":22180,"ancestor_record_offset":21968,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22400,"parent_record_offset":22368,"ancestor_record_offset":21968,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22496,"parent_record_offset":22400,"ancestor_record_offset":21968,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22368,"parent_record_offset":22344,"ancestor_record_offset":21968,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22180,"parent_record_offset":22144,"ancestor_record_offset":21968,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1987,"record_offset":22520,"parent_record_offset":22496,"ancestor_record_offset":21968,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":22800,"parent_record_offset":22768,"ancestor_record_offset":21968,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":23084,"parent_record_offset":22800,"ancestor_record_offset":21968,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":23056,"parent_record_offset":22800,"ancestor_record_offset":21968,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1987,"record_offset":22860,"parent_record_offset":22800,"ancestor_record_offset":21968,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":26276,"parent_record_offset":26216,"ancestor_record_offset":21968,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":25908,"parent_record_offset":25872,"ancestor_record_offset":21968,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":26612,"parent_record_offset":26588,"ancestor_record_offset":21968,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":26300,"parent_record_offset":26276,"ancestor_record_offset":21968,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":25932,"parent_record_offset":25908,"ancestor_record_offset":21968,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":26636,"parent_record_offset":26612,"ancestor_record_offset":21968,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1987,"record_offset":26192,"parent_record_offset":26168,"ancestor_record_offset":21968,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":26816,"parent_record_offset":26728,"ancestor_record_offset":21968,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":26728,"parent_record_offset":26704,"ancestor_record_offset":21968,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":26840,"parent_record_offset":26816,"ancestor_record_offset":21968,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1987,"record_offset":25872,"parent_record_offset":25844,"ancestor_record_offset":21968,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1987,"record_offset":26216,"parent_record_offset":26192,"ancestor_record_offset":21968,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":25964,"parent_record_offset":25872,"ancestor_record_offset":21968,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1987,"record_offset":26248,"parent_record_offset":26216,"ancestor_record_offset":21968,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":23616,"parent_record_offset":23576,"ancestor_record_offset":21968,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":24816,"parent_record_offset":24780,"ancestor_record_offset":21968,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":25368,"parent_record_offset":25096,"ancestor_record_offset":21968,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1987,"record_offset":23768,"parent_record_offset":23740,"ancestor_record_offset":21968,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":23872,"parent_record_offset":23700,"ancestor_record_offset":21968,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1987,"record_offset":24144,"parent_record_offset":23872,"ancestor_record_offset":21968,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1987,"record_offset":23796,"parent_record_offset":23768,"ancestor_record_offset":21968,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1987,"record_offset":23700,"parent_record_offset":23656,"ancestor_record_offset":21968,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1987,"record_offset":23656,"parent_record_offset":23616,"ancestor_record_offset":21968,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::present_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 581
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C08
// 首条位置: {"module":707,"inlinee":39944}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::extensions_generated::khr::swapchain::Device::queue_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":707,"record_offset":10436,"parent_record_offset":10372,"ancestor_record_offset":10372,"inlinee":39945,"source_line":81,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::queue_present
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":707,"record_offset":10404,"parent_record_offset":10372,"ancestor_record_offset":10372,"inlinee":39360,"source_line":32,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::retire
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 638
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155BD
// 首条位置: {"module":1987,"inlinee":87485}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::extensions_generated::ext::full_screen_exclusive::Device::release_full_screen_exclusive_mode
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/ext/full_screen_exclusive.rs
//     evidence={"module":1987,"record_offset":28184,"parent_record_offset":28064,"ancestor_record_offset":28064,"inlinee":87486,"source_line":20,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1987,"record_offset":28352,"parent_record_offset":28064,"ancestor_record_offset":28064,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":28560,"parent_record_offset":28064,"ancestor_record_offset":28064,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":28156,"parent_record_offset":28064,"ancestor_record_offset":28064,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":28384,"parent_record_offset":28064,"ancestor_record_offset":28064,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":28268,"parent_record_offset":28064,"ancestor_record_offset":28064,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::swapchain_retire
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":1987,"record_offset":28124,"parent_record_offset":28064,"ancestor_record_offset":28064,"inlinee":39366,"source_line":32,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":28208,"parent_record_offset":28184,"ancestor_record_offset":28064,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1987,"record_offset":28232,"parent_record_offset":28208,"ancestor_record_offset":28064,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":28440,"parent_record_offset":28408,"ancestor_record_offset":28064,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1987,"record_offset":28468,"parent_record_offset":28408,"ancestor_record_offset":28064,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":28292,"parent_record_offset":28268,"ancestor_record_offset":28064,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1987,"record_offset":28316,"parent_record_offset":28292,"ancestor_record_offset":28064,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":28496,"parent_record_offset":28408,"ancestor_record_offset":28064,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1987,"record_offset":28408,"parent_record_offset":28384,"ancestor_record_offset":28064,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1987,"record_offset":28520,"parent_record_offset":28496,"ancestor_record_offset":28064,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::dispose
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 670
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155BF
// 首条位置: {"module":1987,"inlinee":87487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29528,"parent_record_offset":29200,"ancestor_record_offset":29200,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1987,"record_offset":29824,"parent_record_offset":29200,"ancestor_record_offset":29200,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1987,"record_offset":31036,"parent_record_offset":29200,"ancestor_record_offset":29200,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1987,"record_offset":29292,"parent_record_offset":29200,"ancestor_record_offset":29200,"inlinee":39247,"source_line":143,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":29400,"parent_record_offset":29376,"ancestor_record_offset":29200,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":29448,"parent_record_offset":29424,"ancestor_record_offset":29200,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1987,"record_offset":29424,"parent_record_offset":29400,"ancestor_record_offset":29200,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29376,"parent_record_offset":29340,"ancestor_record_offset":29200,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29568,"parent_record_offset":29528,"ancestor_record_offset":29200,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29680,"parent_record_offset":29528,"ancestor_record_offset":29200,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29340,"parent_record_offset":29316,"ancestor_record_offset":29200,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1987,"record_offset":29708,"parent_record_offset":29528,"ancestor_record_offset":29200,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1987,"record_offset":29960,"parent_record_offset":29936,"ancestor_record_offset":29200,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1987,"record_offset":29936,"parent_record_offset":29824,"ancestor_record_offset":29200,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1987,"record_offset":29784,"parent_record_offset":29760,"ancestor_record_offset":29200,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1987,"record_offset":29900,"parent_record_offset":29856,"ancestor_record_offset":29200,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":29984,"parent_record_offset":29960,"ancestor_record_offset":29200,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1987,"record_offset":29488,"parent_record_offset":29340,"ancestor_record_offset":29200,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1987,"record_offset":29736,"parent_record_offset":29528,"ancestor_record_offset":29200,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1987,"record_offset":29760,"parent_record_offset":29736,"ancestor_record_offset":29200,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1987,"record_offset":29856,"parent_record_offset":29824,"ancestor_record_offset":29200,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$7::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1987,"record_offset":29316,"parent_record_offset":29292,"ancestor_record_offset":29200,"inlinee":86797,"source_line":310,"archive":"_pdb/B/modules/01987.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$8::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1987,"record_offset":30056,"parent_record_offset":30024,"ancestor_record_offset":29200,"inlinee":86799,"source_line":316,"archive":"_pdb/B/modules/01987.jsonl.gz"}
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
// 完整结构及原始类型记录: swapchain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::swapchain::ManagedSwapchain
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::swapchain::ManagedSwapchain>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::swapchain::ManagedSwapchain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::swapchain::ManagedSwapchain>", "constructor": "rhapsody::raw::managed::resources::impl$5::dispose", "arguments": ["rhapsody::raw::managed::swapchain::ManagedSwapchain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain>", "constructor": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0", "arguments": ["rhapsody::raw::managed::swapchain::ManagedSwapchain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::swapchain::SwapchainInner
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 12; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,rhapsody::raw::managed::swapchain::SwapchainInner>", "constructor": "lock_api::rwlock::RwLockWriteGuard", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "rhapsody::raw::managed::swapchain::SwapchainInner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,rhapsody::raw::managed::swapchain::SwapchainInner> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,rhapsody::raw::managed::swapchain::SwapchainInner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: swapchain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: swapchain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next
// 信息 ID: 5cc8a8184c93b7d8ed7a9a72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::Swapchain::acquire_next_image @ 0xc0ed21 -> 0x18cb520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::desc
// 信息 ID: c67e6058d0bbddccb98d5a3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::Swapchain::extent @ 0xc0eac4 -> 0x18cb100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::mark_stale
// 信息 ID: 0ce2b7c47cc48bb3c0299ab4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::mark_stale; module=710, inline_record=2680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::new
// 信息 ID: 4fdba87c57bae0d96ac2913b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_swapchain @ 0xbfd7b2 -> 0x18ca030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::on_focus
// 信息 ID: 5ae3159189786645fb9c4a35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::on_focus; module=710, inline_record=1160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::present
// 信息 ID: d3fcd5c1924bd1ea0425ed1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::Swapchain::present @ 0xc0efba -> 0x18cc1b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::resize
// 信息 ID: caa6fe968ad44cd3487db9fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::Swapchain::resize @ 0xc0e82a -> 0x18cb1b0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::retire
// 信息 ID: 531c463f670fe9ac20cb2836
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_swapchain @ 0xbfd778 -> 0x18ccb50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::set_vsync
// 信息 ID: 8b2426dee9e154ad69070dde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::Swapchain::set_vsync @ 0xc0e8da -> 0x18cb370 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::acquire_next_image
// 信息 ID: d73a738f1591314caeeaa981
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next; module=1987, inline_record=14008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::dispose
// 信息 ID: d1a95ec939ddbd89ebc0c237
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::impl$1::dispose; module=1987, inline_record=29200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
// 信息 ID: 6561a83e0a78b6462573e462
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::mark_stale; module=710, inline_record=2924; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next; module=1987, inline_record=19876; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next; module=1987, inline_record=20672; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::present; module=1987, inline_record=24648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::new
// 信息 ID: 316b7067fd1add5ed90f0daf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::new; module=1987, inline_record=4872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::new_impl
// 信息 ID: fa3843f2ae44ad7e3ba63428
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::new; module=1987, inline_record=4908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::on_focus
// 信息 ID: 3b23a71e10f0b316898c6f7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::on_focus; module=710, inline_record=1404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::present_image
// 信息 ID: dd45c014cdc31e88ce65d2f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::present; module=1987, inline_record=21968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::resize
// 信息 ID: f12de893f90d9f16ade2d0ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::resize; module=1987, inline_record=11980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::retire
// 信息 ID: b061f59f1e0f04f950db29de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::retire; module=1987, inline_record=28064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::set_fullscreen_exclusive
// 信息 ID: 4aa2705d42d53106419efb77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::set_vsync
// 信息 ID: 9e9fd252657e1a150c99fc6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::set_vsync; module=1987, inline_record=12992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$1::dispose
// 信息 ID: 9f90519b918423087a98b5c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::swapchain::ManagedSwapchain> @ 0x18af569 -> 0x18ccdd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::new::closure$0
// 信息 ID: 5861b7711ad73a91025505ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::new; module=1987, inline_record=10240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$2
// 信息 ID: 6d70f7162d1e720cd7c4e12f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::new_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::new; module=1987, inline_record=6448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$5
// 信息 ID: b8965bc141f27858f24384a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::new_impl", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5 <- alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::texture::Texture,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5 <- alloc::vec::into_iter::impl$5::fold<ash::vk::definitions::Image,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ash::vk::definitions::Image,rhapsody::raw::texture::Texture,tuple$<>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::texture::Texture,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::texture::Texture,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5 <- alloc::vec::into_iter::impl$5::fold<ash::vk::definitions::Image,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ash::vk::definitions::Image,rhapsody::raw::texture::Texture,tuple$<>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::texture::Texture,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::texture::Texture,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::fold<ash::vk::definitions::Image,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ash::vk::definitions::Image,rhapsody::raw::texture::Texture,tuple$<>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::texture::Texture,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::texture::Texture,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> > > > >; module=3273, inline_record=1068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$6
// 信息 ID: 46dae41b78544834682565fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::new_impl", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$6 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$6> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$6> >; module=1936, inline_record=24620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$7
// 信息 ID: ac0d292992cb7520dea4d539
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::new_impl", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$7 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$7> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$7> >; module=1936, inline_record=27300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::present_image::closure$0
// 信息 ID: fe960a7881e006a60e3e19de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::present_image", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0 <- rhapsody::raw::device::Queue::with_raw<rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::raw::device::Queue::with_raw<rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >; module=707, inline_record=10372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
