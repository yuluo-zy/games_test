// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/managed/swapchain.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 051d682495e0fe7ad3628f13c3b342a8750f17d2aaa90f777987a68b379bd562
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: swapchain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 569, 1593, 1622, 1624, 3007
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use append_only_vec; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1622,"record_offset":5032,"rva":18387152,"va":5387096272}
// 编译实例 RVAs: 0x11890d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14347197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387096272,"raw_hex":"e8431425ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5387096888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14347780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387096272,"raw_hex":"e8fc1125ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5387097487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10863272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387096272,"raw_hex":"e8583d5aff","size":5,"soft_fail":false,"target_va":5376236208,"va":5387099475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117a3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-62942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387096272,"raw_hex":"e8220affff","size":5,"soft_fail":false,"target_va":5387035616,"va":5387098553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117b480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-58955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387096272,"raw_hex":"e8b519ffff","size":5,"soft_fail":false,"target_va":5387039872,"va":5387098822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15878872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387096272,"raw_hex":"e8d84af200","size":5,"soft_fail":false,"target_va":5402978144,"va":5387099267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::prelude::read_into_uninitialized_vector
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-30600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387096272,"raw_hex":"e87888ffff","size":5,"soft_fail":false,"target_va":5387066656,"va":5387097251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15880786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387096272,"raw_hex":"e85252f200","size":5,"soft_fail":false,"target_va":5402980128,"va":5387099337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-9742820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387096272,"raw_hex":"e81c566bff","size":5,"soft_fail":false,"target_va":5377353776,"va":5387096591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*32(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387096272,"raw_hex":"ff5020","size":3,"soft_fail":false,"va":5387096354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":9936,"parent_record_offset":5032,"ancestor_record_offset":5032,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":9972,"parent_record_offset":5032,"ancestor_record_offset":5032,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":5180,"parent_record_offset":5032,"ancestor_record_offset":5032,"inlinee":70277,"source_line":115,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":5032,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":5032,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":5032,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5624,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":5032,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":7004,"parent_record_offset":6916,"ancestor_record_offset":5032,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":5032,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":5032,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1622,"record_offset":6172,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":8496,"parent_record_offset":8472,"ancestor_record_offset":5032,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":8416,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1622,"record_offset":6820,"parent_record_offset":6796,"ancestor_record_offset":5032,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1622,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":5032,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1622,"record_offset":6344,"parent_record_offset":6304,"ancestor_record_offset":5032,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_capabilities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1622,"record_offset":5488,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":70279,"source_line":48,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_present_modes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1622,"record_offset":6200,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":70282,"source_line":31,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::create_swapchain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1622,"record_offset":7912,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":70290,"source_line":14,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::get_swapchain_images
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1622,"record_offset":8348,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":70291,"source_line":41,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::CompositeAlphaFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1622,"record_offset":7616,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":70287,"source_line":32,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::SurfaceTransformFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1622,"record_offset":7420,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":70285,"source_line":32,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::SwapchainCreateInfoKHR::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1622,"record_offset":7708,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":70251,"source_line":9099,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$2563::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1622,"record_offset":7652,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":70289,"source_line":28602,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":5520,"parent_record_offset":5488,"ancestor_record_offset":5032,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":5548,"parent_record_offset":5520,"ancestor_record_offset":5032,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":5584,"parent_record_offset":5548,"ancestor_record_offset":5032,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$617::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1622,"record_offset":6704,"parent_record_offset":6676,"ancestor_record_offset":5032,"inlinee":70205,"source_line":1141,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1622,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":5032,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":6088,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":6116,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":1622,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":5032,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1622,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":5032,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1622,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":5032,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1622,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":5032,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":8472,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":6272,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":6448,"parent_record_offset":6304,"ancestor_record_offset":5032,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":5032,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":6304,"parent_record_offset":6272,"ancestor_record_offset":5032,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1622,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":5032,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":7680,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":6768,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":6796,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1622,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":5032,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":6380,"parent_record_offset":6344,"ancestor_record_offset":5032,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":6640,"parent_record_offset":6596,"ancestor_record_offset":5032,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":6408,"parent_record_offset":6380,"ancestor_record_offset":5032,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":5032,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":6228,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":8376,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1622,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":5032,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1622,"record_offset":6676,"parent_record_offset":6560,"ancestor_record_offset":5032,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1622,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":5032,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":6560,"parent_record_offset":6532,"ancestor_record_offset":5032,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":6596,"parent_record_offset":6560,"ancestor_record_offset":5032,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":6028,"parent_record_offset":6004,"ancestor_record_offset":5032,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":5032,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":6144,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":6004,"parent_record_offset":5216,"ancestor_record_offset":5032,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::SwapchainInner::new_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":5216,"parent_record_offset":5180,"ancestor_record_offset":5032,"inlinee":70278,"source_line":126,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::impl$2::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":9480,"parent_record_offset":9432,"ancestor_record_offset":5032,"inlinee":70281,"source_line":120,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":6476,"parent_record_offset":6448,"ancestor_record_offset":5032,"inlinee":70283,"source_line":197,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::on_focus
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x789D
// 首条位置: {"module":569,"inlinee":30877}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":569,"record_offset":1548,"parent_record_offset":1312,"ancestor_record_offset":1312,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":1340,"parent_record_offset":1312,"ancestor_record_offset":1312,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::on_focus
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":569,"record_offset":1520,"parent_record_offset":1312,"ancestor_record_offset":1312,"inlinee":30879,"source_line":372,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":569,"record_offset":1456,"parent_record_offset":1368,"ancestor_record_offset":1312,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":569,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":1312,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":1632,"parent_record_offset":1600,"ancestor_record_offset":1312,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":1400,"parent_record_offset":1368,"ancestor_record_offset":1312,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":1312,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":1312,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":1312,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":1368,"parent_record_offset":1340,"ancestor_record_offset":1312,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":1600,"parent_record_offset":1572,"ancestor_record_offset":1312,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::desc
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78A4
// 首条位置: {"module":569,"inlinee":30884}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":569,"record_offset":3824,"parent_record_offset":3508,"ancestor_record_offset":3508,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":3544,"parent_record_offset":3508,"ancestor_record_offset":3508,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":569,"record_offset":3688,"parent_record_offset":3600,"ancestor_record_offset":3508,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":569,"record_offset":3720,"parent_record_offset":3688,"ancestor_record_offset":3508,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3756,"parent_record_offset":3600,"ancestor_record_offset":3508,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3908,"parent_record_offset":3876,"ancestor_record_offset":3508,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3632,"parent_record_offset":3600,"ancestor_record_offset":3508,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":3508,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":3508,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":3508,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":3508,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":3600,"parent_record_offset":3572,"ancestor_record_offset":3508,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":3572,"parent_record_offset":3544,"ancestor_record_offset":3508,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3508,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::resize
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1622,"record_offset":10004,"rva":18390592,"va":5387099712}
// 编译实例 RVAs: 0x1189e40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9746105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387099712,"raw_hex":"e847496bff","size":5,"soft_fail":false,"target_va":5377353776,"va":5387099876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15863918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387099712,"raw_hex":"e86e10f200","size":5,"soft_fail":false,"target_va":5402963904,"va":5387099981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15865340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387099712,"raw_hex":"e8fc15f200","size":5,"soft_fail":false,"target_va":5402965344,"va":5387099999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":10572,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":10156,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::resize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":10336,"parent_record_offset":10004,"ancestor_record_offset":10004,"inlinee":70292,"source_line":417,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":10452,"parent_record_offset":10336,"ancestor_record_offset":10004,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":10480,"parent_record_offset":10336,"ancestor_record_offset":10004,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":10536,"parent_record_offset":10336,"ancestor_record_offset":10004,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":10272,"parent_record_offset":10184,"ancestor_record_offset":10004,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":10296,"parent_record_offset":10272,"ancestor_record_offset":10004,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":10660,"parent_record_offset":10628,"ancestor_record_offset":10004,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":10216,"parent_record_offset":10184,"ancestor_record_offset":10004,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":10004,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":10684,"parent_record_offset":10660,"ancestor_record_offset":10004,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":10240,"parent_record_offset":10216,"ancestor_record_offset":10004,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":10004,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":10600,"parent_record_offset":10572,"ancestor_record_offset":10004,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":10508,"parent_record_offset":10336,"ancestor_record_offset":10004,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":10368,"parent_record_offset":10336,"ancestor_record_offset":10004,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":10184,"parent_record_offset":10156,"ancestor_record_offset":10004,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":10628,"parent_record_offset":10600,"ancestor_record_offset":10004,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::set_vsync
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1622,"record_offset":10944,"rva":18390960,"va":5387100080}
// 编译实例 RVAs: 0x1189fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9746472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100080,"raw_hex":"e8d8476bff","size":5,"soft_fail":false,"target_va":5377353776,"va":5387100243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15863568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100080,"raw_hex":"e8100ff200","size":5,"soft_fail":false,"target_va":5402963904,"va":5387100331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15864991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100080,"raw_hex":"e89f14f200","size":5,"soft_fail":false,"target_va":5402965344,"va":5387100348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":11516,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":11100,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::set_vsync
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":11280,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":70293,"source_line":422,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":11396,"parent_record_offset":11280,"ancestor_record_offset":10944,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":11424,"parent_record_offset":11280,"ancestor_record_offset":10944,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":11480,"parent_record_offset":11280,"ancestor_record_offset":10944,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":11216,"parent_record_offset":11128,"ancestor_record_offset":10944,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":10944,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":11604,"parent_record_offset":11572,"ancestor_record_offset":10944,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":11160,"parent_record_offset":11128,"ancestor_record_offset":10944,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":11336,"parent_record_offset":11312,"ancestor_record_offset":10944,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":11628,"parent_record_offset":11604,"ancestor_record_offset":10944,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":10944,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":11360,"parent_record_offset":11336,"ancestor_record_offset":10944,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":11544,"parent_record_offset":11516,"ancestor_record_offset":10944,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":11452,"parent_record_offset":11280,"ancestor_record_offset":10944,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":11312,"parent_record_offset":11280,"ancestor_record_offset":10944,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":11128,"parent_record_offset":11100,"ancestor_record_offset":10944,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":11572,"parent_record_offset":11544,"ancestor_record_offset":10944,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::mark_stale
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78A0
// 首条位置: {"module":569,"inlinee":30880}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":569,"record_offset":3012,"parent_record_offset":2736,"ancestor_record_offset":2736,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":2764,"parent_record_offset":2736,"ancestor_record_offset":2736,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":569,"record_offset":2944,"parent_record_offset":2736,"ancestor_record_offset":2736,"inlinee":30881,"source_line":427,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":569,"record_offset":2972,"parent_record_offset":2944,"ancestor_record_offset":2736,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":569,"record_offset":2880,"parent_record_offset":2792,"ancestor_record_offset":2736,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":569,"record_offset":2904,"parent_record_offset":2880,"ancestor_record_offset":2736,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3100,"parent_record_offset":3064,"ancestor_record_offset":2736,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":2824,"parent_record_offset":2792,"ancestor_record_offset":2736,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3128,"parent_record_offset":3100,"ancestor_record_offset":2736,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":2736,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":2736,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":2792,"parent_record_offset":2764,"ancestor_record_offset":2736,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":3064,"parent_record_offset":3036,"ancestor_record_offset":2736,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::set_fullscreen_exclusive
// 记录形式: inlinee_declaration; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78A2
// 首条位置: {"module":569,"inlinee":30882}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next
// 记录形式: compiled_procedure; instances=1
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1622,"record_offset":11888,"rva":18391312,"va":5387100432}
// 编译实例 RVAs: 0x118a110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ash..vk..enums..Result$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10166899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e88ddd64ff","size":5,"soft_fail":false,"target_va":5376934384,"va":5387101278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14351251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e86d0425ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5387100942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14352626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e80eff24ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5387102333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15875917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e84d3ff200","size":5,"soft_fail":false,"target_va":5402978144,"va":5387102222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15876486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e88641f200","size":5,"soft_fail":false,"target_va":5402978688,"va":5387102197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15876639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e81f42f200","size":5,"soft_fail":false,"target_va":5402978880,"va":5387102236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15876964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e86443f200","size":5,"soft_fail":false,"target_va":5402979152,"va":5387102183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15877831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e8c746f200","size":5,"soft_fail":false,"target_va":5402979344,"va":5387101508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e83de3ffff","size":5,"soft_fail":false,"target_va":5387094736,"va":5387102094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15878294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e89648f200","size":5,"soft_fail":false,"target_va":5402980128,"va":5387101829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e821eaffff","size":5,"soft_fail":false,"target_va":5387094928,"va":5387100522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15880047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e86f4ff200","size":5,"soft_fail":false,"target_va":5402980656,"va":5387100604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9747376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e850446bff","size":5,"soft_fail":false,"target_va":5377353776,"va":5387101147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15862369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e8610af200","size":5,"soft_fail":false,"target_va":5402963904,"va":5387101530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15863199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e89f0df200","size":5,"soft_fail":false,"target_va":5402965344,"va":5387102140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10745096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e8f80a5cff","size":5,"soft_fail":false,"target_va":5376356912,"va":5387102003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10741206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e82a1a5cff","size":5,"soft_fail":false,"target_va":5376360560,"va":5387101761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10744354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e8de0d5cff","size":5,"soft_fail":false,"target_va":5376356144,"va":5387100493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hf5f0f90a77fe959e (.llvm.16944548595553555407)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-52346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387100432,"raw_hex":"e88633ffff","size":5,"soft_fail":false,"target_va":5387049440,"va":5387101781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5387100432,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5387100740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":16552,"parent_record_offset":11888,"ancestor_record_offset":11888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":12048,"parent_record_offset":11888,"ancestor_record_offset":11888,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::acquire_next_image
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":12228,"parent_record_offset":11888,"ancestor_record_offset":11888,"inlinee":70294,"source_line":453,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":11888,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":11888,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":14416,"parent_record_offset":14392,"ancestor_record_offset":11888,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":13624,"parent_record_offset":13596,"ancestor_record_offset":11888,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":14392,"parent_record_offset":14336,"ancestor_record_offset":11888,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":14364,"parent_record_offset":14336,"ancestor_record_offset":11888,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":13596,"parent_record_offset":13568,"ancestor_record_offset":11888,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":13568,"parent_record_offset":13540,"ancestor_record_offset":11888,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":13744,"parent_record_offset":13568,"ancestor_record_offset":11888,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":14336,"parent_record_offset":14308,"ancestor_record_offset":11888,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1622,"record_offset":13504,"parent_record_offset":13476,"ancestor_record_offset":11888,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1622,"record_offset":13476,"parent_record_offset":13448,"ancestor_record_offset":11888,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1622,"record_offset":13420,"parent_record_offset":13392,"ancestor_record_offset":11888,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1622,"record_offset":13448,"parent_record_offset":13420,"ancestor_record_offset":11888,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1622,"record_offset":13392,"parent_record_offset":13364,"ancestor_record_offset":11888,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1622,"record_offset":13364,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1622,"record_offset":14168,"parent_record_offset":14120,"ancestor_record_offset":11888,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1622,"record_offset":14120,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":13540,"parent_record_offset":13504,"ancestor_record_offset":11888,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":12976,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::acquire_next_image
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1622,"record_offset":13072,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":70295,"source_line":54,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":15900,"parent_record_offset":15872,"ancestor_record_offset":11888,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":15952,"parent_record_offset":15928,"ancestor_record_offset":11888,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":15928,"parent_record_offset":15900,"ancestor_record_offset":11888,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":15840,"parent_record_offset":15808,"ancestor_record_offset":11888,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":15872,"parent_record_offset":15840,"ancestor_record_offset":11888,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":16044,"parent_record_offset":15872,"ancestor_record_offset":11888,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":16156,"parent_record_offset":16132,"ancestor_record_offset":11888,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":13944,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":13972,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":14196,"parent_record_offset":14120,"ancestor_record_offset":11888,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1622,"record_offset":13804,"parent_record_offset":13780,"ancestor_record_offset":11888,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1622,"record_offset":15976,"parent_record_offset":15952,"ancestor_record_offset":11888,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1622,"record_offset":12604,"parent_record_offset":12508,"ancestor_record_offset":11888,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":15732,"parent_record_offset":15704,"ancestor_record_offset":11888,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":14052,"parent_record_offset":14028,"ancestor_record_offset":11888,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":15064,"parent_record_offset":14604,"ancestor_record_offset":11888,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":15780,"parent_record_offset":15600,"ancestor_record_offset":11888,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":14084,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1622,"record_offset":12924,"parent_record_offset":12896,"ancestor_record_offset":11888,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1622,"record_offset":13780,"parent_record_offset":13504,"ancestor_record_offset":11888,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":14260,"parent_record_offset":14120,"ancestor_record_offset":11888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":14840,"parent_record_offset":14816,"ancestor_record_offset":11888,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":14816,"parent_record_offset":14792,"ancestor_record_offset":11888,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":15024,"parent_record_offset":14992,"ancestor_record_offset":11888,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":13700,"parent_record_offset":13672,"ancestor_record_offset":11888,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":11888,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":14224,"parent_record_offset":14120,"ancestor_record_offset":11888,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":12164,"parent_record_offset":12076,"ancestor_record_offset":11888,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":11888,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1622,"record_offset":12540,"parent_record_offset":12508,"ancestor_record_offset":11888,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":13004,"parent_record_offset":12976,"ancestor_record_offset":11888,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":12568,"parent_record_offset":12540,"ancestor_record_offset":11888,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":13032,"parent_record_offset":13004,"ancestor_record_offset":11888,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":14792,"parent_record_offset":14768,"ancestor_record_offset":11888,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":11888,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":14768,"parent_record_offset":14660,"ancestor_record_offset":11888,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1622,"record_offset":12508,"parent_record_offset":12480,"ancestor_record_offset":11888,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1622,"record_offset":14500,"parent_record_offset":14468,"ancestor_record_offset":11888,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1622,"record_offset":14632,"parent_record_offset":14604,"ancestor_record_offset":11888,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1622,"record_offset":14604,"parent_record_offset":14572,"ancestor_record_offset":11888,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":14548,"parent_record_offset":14524,"ancestor_record_offset":11888,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":14572,"parent_record_offset":14548,"ancestor_record_offset":11888,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":14524,"parent_record_offset":14500,"ancestor_record_offset":11888,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":15096,"parent_record_offset":14572,"ancestor_record_offset":11888,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12712,"parent_record_offset":12676,"ancestor_record_offset":11888,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12872,"parent_record_offset":12712,"ancestor_record_offset":11888,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12676,"parent_record_offset":12648,"ancestor_record_offset":11888,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12416,"parent_record_offset":12376,"ancestor_record_offset":11888,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12896,"parent_record_offset":12872,"ancestor_record_offset":11888,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12480,"parent_record_offset":12444,"ancestor_record_offset":11888,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12444,"parent_record_offset":12416,"ancestor_record_offset":11888,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":14660,"parent_record_offset":14632,"ancestor_record_offset":11888,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":14908,"parent_record_offset":14660,"ancestor_record_offset":11888,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":14880,"parent_record_offset":14660,"ancestor_record_offset":11888,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":14712,"parent_record_offset":14660,"ancestor_record_offset":11888,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":13296,"parent_record_offset":13260,"ancestor_record_offset":11888,"inlinee":30606,"source_line":2665,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":16644,"parent_record_offset":16608,"ancestor_record_offset":11888,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":12108,"parent_record_offset":12076,"ancestor_record_offset":11888,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":11888,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":13320,"parent_record_offset":13296,"ancestor_record_offset":11888,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":16668,"parent_record_offset":16644,"ancestor_record_offset":11888,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":12132,"parent_record_offset":12108,"ancestor_record_offset":11888,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":13908,"parent_record_offset":13884,"ancestor_record_offset":11888,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":16580,"parent_record_offset":16552,"ancestor_record_offset":11888,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":14000,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":13860,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":12076,"parent_record_offset":12048,"ancestor_record_offset":11888,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":16608,"parent_record_offset":16580,"ancestor_record_offset":11888,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":15480,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":15520,"parent_record_offset":15480,"ancestor_record_offset":11888,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":12376,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1622,"record_offset":15644,"parent_record_offset":15600,"ancestor_record_offset":11888,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":15808,"parent_record_offset":15600,"ancestor_record_offset":11888,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":16080,"parent_record_offset":15808,"ancestor_record_offset":11888,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":14468,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1622,"record_offset":13236,"parent_record_offset":13212,"ancestor_record_offset":11888,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1622,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":11888,"inlinee":30605,"source_line":107,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":14028,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":30881,"source_line":427,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1622,"record_offset":13212,"parent_record_offset":12228,"ancestor_record_offset":11888,"inlinee":12247,"source_line":23,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1622,"record_offset":15672,"parent_record_offset":15644,"ancestor_record_offset":11888,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1622,"record_offset":15704,"parent_record_offset":15672,"ancestor_record_offset":11888,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1622,"record_offset":15600,"parent_record_offset":15560,"ancestor_record_offset":11888,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1622,"record_offset":15560,"parent_record_offset":15520,"ancestor_record_offset":11888,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::present
// 记录形式: compiled_procedure; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1622,"record_offset":16928,"rva":18393440,"va":5387102560}
// 编译实例 RVAs: 0x118a960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10864103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8193a5aff","size":5,"soft_fail":false,"target_va":5376239920,"va":5387104018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073aa70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10814795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8b5fa5aff","size":5,"soft_fail":false,"target_va":5376289392,"va":5387104182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118c280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8cb140000","size":5,"soft_fail":false,"target_va":5387108992,"va":5387103664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15874519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8d739f200","size":5,"soft_fail":false,"target_va":5402978688,"va":5387104164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15875254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8b63cf200","size":5,"soft_fail":false,"target_va":5402979344,"va":5387104085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15874789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8e53af200","size":5,"soft_fail":false,"target_va":5402978944,"va":5387104150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8c2dbffff","size":5,"soft_fail":false,"target_va":5387094736,"va":5387104009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15876965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e86543f200","size":5,"soft_fail":false,"target_va":5402980128,"va":5387103158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e882e1ffff","size":5,"soft_fail":false,"target_va":5387094928,"va":5387102729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15877852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8dc46f200","size":5,"soft_fail":false,"target_va":5402980656,"va":5387102799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9750126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e892396bff","size":5,"soft_fail":false,"target_va":5377353776,"va":5387103897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15865391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e82f16f200","size":5,"soft_fail":false,"target_va":5402968992,"va":5387103596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t15867055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8af1cf200","size":5,"soft_fail":false,"target_va":5402970800,"va":5387103740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15861257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e80906f200","size":5,"soft_fail":false,"target_va":5402963904,"va":5387102642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15861286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e82606f200","size":5,"soft_fail":false,"target_va":5402965344,"va":5387104053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10746428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8c4055cff","size":5,"soft_fail":false,"target_va":5376356912,"va":5387103335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10742534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8fa145cff","size":5,"soft_fail":false,"target_va":5376360560,"va":5387103089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10746561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e83f055cff","size":5,"soft_fail":false,"target_va":5376356144,"va":5387102700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hf5f0f90a77fe959e (.llvm.16944548595553555407)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-53674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387102560,"raw_hex":"e8562effff","size":5,"soft_fail":false,"target_va":5387049440,"va":5387103109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":21772,"parent_record_offset":16928,"ancestor_record_offset":16928,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":17080,"parent_record_offset":16928,"ancestor_record_offset":16928,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::present_image
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":17252,"parent_record_offset":16928,"ancestor_record_offset":16928,"inlinee":70296,"source_line":509,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":20340,"parent_record_offset":20304,"ancestor_record_offset":16928,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::PresentInfoKHR::image_indices
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1622,"record_offset":20104,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":70298,"source_line":9161,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19384,"parent_record_offset":19356,"ancestor_record_offset":16928,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19436,"parent_record_offset":19412,"ancestor_record_offset":16928,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19412,"parent_record_offset":19384,"ancestor_record_offset":16928,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19324,"parent_record_offset":19292,"ancestor_record_offset":16928,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19356,"parent_record_offset":19324,"ancestor_record_offset":16928,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19528,"parent_record_offset":19356,"ancestor_record_offset":16928,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19640,"parent_record_offset":19616,"ancestor_record_offset":16928,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":20876,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":20904,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":21452,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1622,"record_offset":19460,"parent_record_offset":19436,"ancestor_record_offset":16928,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1622,"record_offset":17588,"parent_record_offset":17492,"ancestor_record_offset":16928,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":19216,"parent_record_offset":19188,"ancestor_record_offset":16928,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":20060,"parent_record_offset":20032,"ancestor_record_offset":16928,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":18548,"parent_record_offset":18088,"ancestor_record_offset":16928,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":19264,"parent_record_offset":19084,"ancestor_record_offset":16928,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1622,"record_offset":17900,"parent_record_offset":17872,"ancestor_record_offset":16928,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":19592,"parent_record_offset":19292,"ancestor_record_offset":16928,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":18324,"parent_record_offset":18300,"ancestor_record_offset":16928,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":18300,"parent_record_offset":18276,"ancestor_record_offset":16928,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":18508,"parent_record_offset":18476,"ancestor_record_offset":16928,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":19484,"parent_record_offset":19460,"ancestor_record_offset":16928,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":19992,"parent_record_offset":19044,"ancestor_record_offset":16928,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":17188,"parent_record_offset":17104,"ancestor_record_offset":16928,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":17212,"parent_record_offset":17188,"ancestor_record_offset":16928,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1622,"record_offset":17524,"parent_record_offset":17492,"ancestor_record_offset":16928,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":20368,"parent_record_offset":20340,"ancestor_record_offset":16928,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":17552,"parent_record_offset":17524,"ancestor_record_offset":16928,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":20396,"parent_record_offset":20368,"ancestor_record_offset":16928,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":16928,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":18604,"parent_record_offset":18580,"ancestor_record_offset":16928,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":18252,"parent_record_offset":18144,"ancestor_record_offset":16928,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1622,"record_offset":17492,"parent_record_offset":17464,"ancestor_record_offset":16928,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1622,"record_offset":17984,"parent_record_offset":17952,"ancestor_record_offset":16928,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1622,"record_offset":18116,"parent_record_offset":18088,"ancestor_record_offset":16928,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1622,"record_offset":18088,"parent_record_offset":18056,"ancestor_record_offset":16928,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":18032,"parent_record_offset":18008,"ancestor_record_offset":16928,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":16928,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":18008,"parent_record_offset":17984,"ancestor_record_offset":16928,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":18580,"parent_record_offset":18056,"ancestor_record_offset":16928,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17688,"parent_record_offset":17656,"ancestor_record_offset":16928,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17848,"parent_record_offset":17688,"ancestor_record_offset":16928,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17656,"parent_record_offset":17632,"ancestor_record_offset":16928,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17408,"parent_record_offset":17376,"ancestor_record_offset":16928,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17872,"parent_record_offset":17848,"ancestor_record_offset":16928,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17464,"parent_record_offset":17432,"ancestor_record_offset":16928,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17432,"parent_record_offset":17408,"ancestor_record_offset":16928,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":18144,"parent_record_offset":18116,"ancestor_record_offset":16928,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":18392,"parent_record_offset":18144,"ancestor_record_offset":16928,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":18364,"parent_record_offset":18144,"ancestor_record_offset":16928,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":18196,"parent_record_offset":18144,"ancestor_record_offset":16928,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20520,"parent_record_offset":20488,"ancestor_record_offset":16928,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20184,"parent_record_offset":20156,"ancestor_record_offset":16928,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":21852,"parent_record_offset":21820,"ancestor_record_offset":16928,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":17132,"parent_record_offset":17104,"ancestor_record_offset":16928,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20816,"parent_record_offset":20792,"ancestor_record_offset":16928,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20544,"parent_record_offset":20520,"ancestor_record_offset":16928,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":17156,"parent_record_offset":17132,"ancestor_record_offset":16928,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20840,"parent_record_offset":20816,"ancestor_record_offset":16928,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1622,"record_offset":20132,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1622,"record_offset":20464,"parent_record_offset":20440,"ancestor_record_offset":16928,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":21796,"parent_record_offset":21772,"ancestor_record_offset":16928,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":20932,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":20792,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1622,"record_offset":20156,"parent_record_offset":20132,"ancestor_record_offset":16928,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1622,"record_offset":20488,"parent_record_offset":20464,"ancestor_record_offset":16928,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":17104,"parent_record_offset":17080,"ancestor_record_offset":16928,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":21820,"parent_record_offset":21796,"ancestor_record_offset":16928,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":18964,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":19004,"parent_record_offset":18964,"ancestor_record_offset":16928,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":17376,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1622,"record_offset":19128,"parent_record_offset":19084,"ancestor_record_offset":16928,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":19292,"parent_record_offset":19084,"ancestor_record_offset":16928,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":19564,"parent_record_offset":19292,"ancestor_record_offset":16928,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":17952,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":20032,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":30881,"source_line":427,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::SubmissionEngine::queue
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":1622,"record_offset":20304,"parent_record_offset":17252,"ancestor_record_offset":16928,"inlinee":30506,"source_line":132,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1622,"record_offset":19156,"parent_record_offset":19128,"ancestor_record_offset":16928,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1622,"record_offset":19188,"parent_record_offset":19156,"ancestor_record_offset":16928,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1622,"record_offset":19084,"parent_record_offset":19044,"ancestor_record_offset":16928,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1622,"record_offset":19044,"parent_record_offset":19004,"ancestor_record_offset":16928,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::ManagedSwapchain::retire
// 记录形式: compiled_procedure; instances=1
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1622,"record_offset":22240,"rva":18395584,"va":5387104704}
// 编译实例 RVAs: 0x118b1c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15873769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387104704,"raw_hex":"e8e936f200","size":5,"soft_fail":false,"target_va":5402978944,"va":5387105170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9751259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387104704,"raw_hex":"e825356bff","size":5,"soft_fail":false,"target_va":5377353776,"va":5387105030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15858794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387104704,"raw_hex":"e86afcf100","size":5,"soft_fail":false,"target_va":5402963904,"va":5387105105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15860256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387104704,"raw_hex":"e82002f200","size":5,"soft_fail":false,"target_va":5402965344,"va":5387105083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*2088(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387104704,"raw_hex":"ff9028080000","size":6,"soft_fail":false,"va":5387104902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":22916,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":22392,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::retire
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":22572,"parent_record_offset":22240,"ancestor_record_offset":22240,"inlinee":70299,"source_line":580,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::ext::full_screen_exclusive::Device::release_full_screen_exclusive_mode
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/ext/full_screen_exclusive.rs
//     evidence={"module":1622,"record_offset":22632,"parent_record_offset":22572,"ancestor_record_offset":22240,"inlinee":70301,"source_line":20,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":22240,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":22680,"parent_record_offset":22656,"ancestor_record_offset":22240,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":22800,"parent_record_offset":22572,"ancestor_record_offset":22240,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":22884,"parent_record_offset":22572,"ancestor_record_offset":22240,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":22828,"parent_record_offset":22572,"ancestor_record_offset":22240,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":22508,"parent_record_offset":22420,"ancestor_record_offset":22240,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":22532,"parent_record_offset":22508,"ancestor_record_offset":22240,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":23012,"parent_record_offset":22972,"ancestor_record_offset":22240,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":22452,"parent_record_offset":22420,"ancestor_record_offset":22240,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":22740,"parent_record_offset":22716,"ancestor_record_offset":22240,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":23040,"parent_record_offset":23012,"ancestor_record_offset":22240,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":22476,"parent_record_offset":22452,"ancestor_record_offset":22240,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":22764,"parent_record_offset":22740,"ancestor_record_offset":22240,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":22944,"parent_record_offset":22916,"ancestor_record_offset":22240,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":22856,"parent_record_offset":22572,"ancestor_record_offset":22240,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":22716,"parent_record_offset":22572,"ancestor_record_offset":22240,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":22420,"parent_record_offset":22392,"ancestor_record_offset":22240,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":22972,"parent_record_offset":22944,"ancestor_record_offset":22240,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$1::dispose
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1622,"record_offset":23308,"rva":18396128,"va":5387105248}
// 编译实例 RVAs: 0x118b3e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072de40
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10868628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387105248,"raw_hex":"e86c285aff","size":5,"soft_fail":false,"target_va":5376237120,"va":5387105743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-15800283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387105248,"raw_hex":"e825e80eff","size":5,"soft_fail":false,"target_va":5371305056,"va":5387105334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15873330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387105248,"raw_hex":"e83235f200","size":5,"soft_fail":false,"target_va":5402978944,"va":5387105609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15858361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387105248,"raw_hex":"e8b9faf100","size":5,"soft_fail":false,"target_va":5402963904,"va":5387105538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15859637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387105248,"raw_hex":"e8b5fff100","size":5,"soft_fail":false,"target_va":5402965344,"va":5387105702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t15859711","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387105248,"raw_hex":"e9fffff100","size":5,"soft_fail":false,"target_va":5402965344,"va":5387105628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":24548,"parent_record_offset":23308,"ancestor_record_offset":23308,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":23452,"parent_record_offset":23308,"ancestor_record_offset":23308,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::dispose
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":23632,"parent_record_offset":23308,"ancestor_record_offset":23308,"inlinee":70302,"source_line":610,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":23308,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23784,"parent_record_offset":23748,"ancestor_record_offset":23308,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23916,"parent_record_offset":23880,"ancestor_record_offset":23308,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23880,"parent_record_offset":23632,"ancestor_record_offset":23308,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23972,"parent_record_offset":23880,"ancestor_record_offset":23308,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23748,"parent_record_offset":23724,"ancestor_record_offset":23308,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":24000,"parent_record_offset":23880,"ancestor_record_offset":23308,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1622,"record_offset":24116,"parent_record_offset":23632,"ancestor_record_offset":23308,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1622,"record_offset":24248,"parent_record_offset":24224,"ancestor_record_offset":23308,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":24516,"parent_record_offset":23632,"ancestor_record_offset":23308,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":24224,"parent_record_offset":24116,"ancestor_record_offset":23308,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1622,"record_offset":24076,"parent_record_offset":24052,"ancestor_record_offset":23308,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":24188,"parent_record_offset":24148,"ancestor_record_offset":23308,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":24272,"parent_record_offset":24248,"ancestor_record_offset":23308,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":23840,"parent_record_offset":23748,"ancestor_record_offset":23308,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":23568,"parent_record_offset":23480,"ancestor_record_offset":23308,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":23592,"parent_record_offset":23568,"ancestor_record_offset":23308,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1622,"record_offset":24028,"parent_record_offset":23880,"ancestor_record_offset":23308,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1622,"record_offset":24052,"parent_record_offset":24028,"ancestor_record_offset":23308,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":24148,"parent_record_offset":24116,"ancestor_record_offset":23308,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":24640,"parent_record_offset":24604,"ancestor_record_offset":23308,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":23512,"parent_record_offset":23480,"ancestor_record_offset":23308,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":24664,"parent_record_offset":24640,"ancestor_record_offset":23308,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":23536,"parent_record_offset":23512,"ancestor_record_offset":23308,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1622,"record_offset":24576,"parent_record_offset":24548,"ancestor_record_offset":23308,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":23480,"parent_record_offset":23452,"ancestor_record_offset":23308,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1622,"record_offset":24604,"parent_record_offset":24576,"ancestor_record_offset":23308,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1622,"record_offset":23700,"parent_record_offset":23632,"ancestor_record_offset":23308,"inlinee":30801,"source_line":119,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$7::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1622,"record_offset":23724,"parent_record_offset":23700,"ancestor_record_offset":23308,"inlinee":70058,"source_line":227,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$8::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1622,"record_offset":24344,"parent_record_offset":24312,"ancestor_record_offset":23308,"inlinee":70062,"source_line":233,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11285
// 首条位置: {"module":1622,"inlinee":70277}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":9432,"parent_record_offset":5180,"ancestor_record_offset":5180,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::new_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":5216,"parent_record_offset":5180,"ancestor_record_offset":5180,"inlinee":70278,"source_line":126,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":5180,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":5180,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":5180,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5624,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":5180,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":7004,"parent_record_offset":6916,"ancestor_record_offset":5180,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":5180,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":5180,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1622,"record_offset":6172,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":8496,"parent_record_offset":8472,"ancestor_record_offset":5180,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":8416,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1622,"record_offset":6820,"parent_record_offset":6796,"ancestor_record_offset":5180,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1622,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":5180,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1622,"record_offset":6344,"parent_record_offset":6304,"ancestor_record_offset":5180,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_capabilities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1622,"record_offset":5488,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":70279,"source_line":48,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_present_modes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1622,"record_offset":6200,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":70282,"source_line":31,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::create_swapchain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1622,"record_offset":7912,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":70290,"source_line":14,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::get_swapchain_images
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1622,"record_offset":8348,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":70291,"source_line":41,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::CompositeAlphaFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1622,"record_offset":7616,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":70287,"source_line":32,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::SurfaceTransformFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1622,"record_offset":7420,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":70285,"source_line":32,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::SwapchainCreateInfoKHR::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1622,"record_offset":7708,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":70251,"source_line":9099,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$2563::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1622,"record_offset":7652,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":70289,"source_line":28602,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":5520,"parent_record_offset":5488,"ancestor_record_offset":5180,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":5548,"parent_record_offset":5520,"ancestor_record_offset":5180,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":5584,"parent_record_offset":5548,"ancestor_record_offset":5180,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$617::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1622,"record_offset":6704,"parent_record_offset":6676,"ancestor_record_offset":5180,"inlinee":70205,"source_line":1141,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1622,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":5180,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":6088,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":6116,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":1622,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":5180,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1622,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":5180,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1622,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":5180,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1622,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":5180,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":8472,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":6272,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":6448,"parent_record_offset":6304,"ancestor_record_offset":5180,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":5180,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":6304,"parent_record_offset":6272,"ancestor_record_offset":5180,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1622,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":5180,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":7680,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":6768,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":6796,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1622,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":5180,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":6380,"parent_record_offset":6344,"ancestor_record_offset":5180,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":6640,"parent_record_offset":6596,"ancestor_record_offset":5180,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":6408,"parent_record_offset":6380,"ancestor_record_offset":5180,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":5180,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":6228,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":8376,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1622,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":5180,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1622,"record_offset":6676,"parent_record_offset":6560,"ancestor_record_offset":5180,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1622,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":5180,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":6560,"parent_record_offset":6532,"ancestor_record_offset":5180,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":6596,"parent_record_offset":6560,"ancestor_record_offset":5180,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":6028,"parent_record_offset":6004,"ancestor_record_offset":5180,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":5180,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":6144,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":6004,"parent_record_offset":5216,"ancestor_record_offset":5180,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::impl$2::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":9480,"parent_record_offset":9432,"ancestor_record_offset":5180,"inlinee":70281,"source_line":120,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":6476,"parent_record_offset":6448,"ancestor_record_offset":5180,"inlinee":70283,"source_line":197,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11289
// 首条位置: {"module":1622,"inlinee":70281}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":9608,"parent_record_offset":9480,"ancestor_record_offset":9480,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":9636,"parent_record_offset":9480,"ancestor_record_offset":9480,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":9692,"parent_record_offset":9480,"ancestor_record_offset":9480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":9664,"parent_record_offset":9480,"ancestor_record_offset":9480,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":9524,"parent_record_offset":9480,"ancestor_record_offset":9480,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":9872,"parent_record_offset":9848,"ancestor_record_offset":9480,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":9848,"parent_record_offset":9792,"ancestor_record_offset":9480,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":9820,"parent_record_offset":9792,"ancestor_record_offset":9480,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":9792,"parent_record_offset":9764,"ancestor_record_offset":9480,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":9480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":9548,"parent_record_offset":9524,"ancestor_record_offset":9480,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":9572,"parent_record_offset":9548,"ancestor_record_offset":9480,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::new_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11286
// 首条位置: {"module":1622,"inlinee":70278}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5624,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1622,"record_offset":6172,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":8416,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_capabilities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1622,"record_offset":5488,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":70279,"source_line":48,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_present_modes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":1622,"record_offset":6200,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":70282,"source_line":31,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::swapchain::Device::create_swapchain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1622,"record_offset":7912,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":70290,"source_line":14,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::swapchain::Device::get_swapchain_images
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1622,"record_offset":8348,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":70291,"source_line":41,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::CompositeAlphaFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1622,"record_offset":7616,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":70287,"source_line":32,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::SurfaceTransformFlagsKHR::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1622,"record_offset":7420,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":70285,"source_line":32,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::SwapchainCreateInfoKHR::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1622,"record_offset":7708,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":70251,"source_line":9099,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2563::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1622,"record_offset":7652,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":70289,"source_line":28602,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":6088,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":6116,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":8472,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":6272,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":7680,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":6768,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":6796,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":6228,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":8376,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":6144,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":6004,"parent_record_offset":5216,"ancestor_record_offset":5216,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":5216,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":5216,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":5216,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":5216,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":7004,"parent_record_offset":6916,"ancestor_record_offset":5216,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":5216,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":5216,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":8496,"parent_record_offset":8472,"ancestor_record_offset":5216,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1622,"record_offset":6820,"parent_record_offset":6796,"ancestor_record_offset":5216,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1622,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":5216,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1622,"record_offset":6344,"parent_record_offset":6304,"ancestor_record_offset":5216,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":5520,"parent_record_offset":5488,"ancestor_record_offset":5216,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":5548,"parent_record_offset":5520,"ancestor_record_offset":5216,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":5584,"parent_record_offset":5548,"ancestor_record_offset":5216,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$617::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1622,"record_offset":6704,"parent_record_offset":6676,"ancestor_record_offset":5216,"inlinee":70205,"source_line":1141,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1622,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":5216,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":1622,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":5216,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1622,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":5216,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1622,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":5216,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1622,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":5216,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":6448,"parent_record_offset":6304,"ancestor_record_offset":5216,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":5216,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1622,"record_offset":6304,"parent_record_offset":6272,"ancestor_record_offset":5216,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1622,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":5216,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":6844,"parent_record_offset":6820,"ancestor_record_offset":5216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1622,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":5216,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":6380,"parent_record_offset":6344,"ancestor_record_offset":5216,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":6640,"parent_record_offset":6596,"ancestor_record_offset":5216,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":6408,"parent_record_offset":6380,"ancestor_record_offset":5216,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":5216,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1622,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":5216,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1622,"record_offset":6676,"parent_record_offset":6560,"ancestor_record_offset":5216,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1622,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":5216,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":6560,"parent_record_offset":6532,"ancestor_record_offset":5216,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":6596,"parent_record_offset":6560,"ancestor_record_offset":5216,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":6028,"parent_record_offset":6004,"ancestor_record_offset":5216,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":5216,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":6476,"parent_record_offset":6448,"ancestor_record_offset":5216,"inlinee":70283,"source_line":197,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1128B
// 首条位置: {"module":1622,"inlinee":70283}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1622,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":6476,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$617::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1622,"record_offset":6704,"parent_record_offset":6676,"ancestor_record_offset":6476,"inlinee":70205,"source_line":1141,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":6640,"parent_record_offset":6596,"ancestor_record_offset":6476,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1622,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":6476,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1622,"record_offset":6676,"parent_record_offset":6560,"ancestor_record_offset":6476,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":6560,"parent_record_offset":6532,"ancestor_record_offset":6476,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":6596,"parent_record_offset":6560,"ancestor_record_offset":6476,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 315
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18186
// 首条位置: {"module":3007,"inlinee":98694}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3007,"record_offset":14884,"parent_record_offset":14324,"ancestor_record_offset":14324,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":3007,"record_offset":14388,"parent_record_offset":14324,"ancestor_record_offset":14324,"inlinee":11133,"source_line":297,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":3007,"record_offset":14756,"parent_record_offset":14708,"ancestor_record_offset":14324,"inlinee":13428,"source_line":104,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":3007,"record_offset":14708,"parent_record_offset":14476,"ancestor_record_offset":14324,"inlinee":30795,"source_line":225,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3007,"record_offset":14840,"parent_record_offset":14708,"ancestor_record_offset":14324,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3007,"record_offset":14596,"parent_record_offset":14476,"ancestor_record_offset":14324,"inlinee":30790,"source_line":35,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null::runtime_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3007,"record_offset":14620,"parent_record_offset":14596,"ancestor_record_offset":14324,"inlinee":30792,"source_line":37,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":14652,"parent_record_offset":14476,"ancestor_record_offset":14324,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":14540,"parent_record_offset":14476,"ancestor_record_offset":14324,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":14780,"parent_record_offset":14756,"ancestor_record_offset":14324,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":14324,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3007,"record_offset":14564,"parent_record_offset":14540,"ancestor_record_offset":14324,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":3007,"record_offset":14476,"parent_record_offset":14436,"ancestor_record_offset":14324,"inlinee":30789,"source_line":71,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$6::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":3007,"record_offset":14436,"parent_record_offset":14388,"ancestor_record_offset":14324,"inlinee":30825,"source_line":368,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 341
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111BE
// 首条位置: {"module":1593,"inlinee":70078}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::create_semaphore
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1593,"record_offset":19792,"parent_record_offset":19728,"ancestor_record_offset":19728,"inlinee":30837,"source_line":2129,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$349::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1593,"record_offset":19764,"parent_record_offset":19728,"ancestor_record_offset":19728,"inlinee":30836,"source_line":7615,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1593,"record_offset":19908,"parent_record_offset":19728,"ancestor_record_offset":19728,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1593,"record_offset":19820,"parent_record_offset":19792,"ancestor_record_offset":19728,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1593,"record_offset":19844,"parent_record_offset":19820,"ancestor_record_offset":19728,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1593,"record_offset":19868,"parent_record_offset":19844,"ancestor_record_offset":19728,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01593.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::on_focus
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x789F
// 首条位置: {"module":569,"inlinee":30879}
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
// 源码行: 417
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11294
// 首条位置: {"module":1622,"inlinee":70292}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":10452,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":10480,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":10536,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":10508,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":10368,"parent_record_offset":10336,"ancestor_record_offset":10336,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":10336,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":10336,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::set_vsync
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 422
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11295
// 首条位置: {"module":1622,"inlinee":70293}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":11396,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":11424,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":11480,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":11452,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":11312,"parent_record_offset":11280,"ancestor_record_offset":11280,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":11336,"parent_record_offset":11312,"ancestor_record_offset":11280,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":11360,"parent_record_offset":11336,"ancestor_record_offset":11280,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 427
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78A1
// 首条位置: {"module":569,"inlinee":30881}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":569,"record_offset":2972,"parent_record_offset":2944,"ancestor_record_offset":2944,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::set_fullscreen_exclusive
// 记录形式: inlinee_declaration; instances=2
// 源码行: 431
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78A3
// 首条位置: {"module":569,"inlinee":30883}
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
// 源码行: 453
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11296
// 首条位置: {"module":1622,"inlinee":70294}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1622,"record_offset":13364,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1622,"record_offset":14120,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":12976,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::swapchain::Device::acquire_next_image
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1622,"record_offset":13072,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":70295,"source_line":54,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":13944,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":13972,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":14084,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":14000,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":13860,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":15480,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":12376,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":14468,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":14028,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":30881,"source_line":427,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1622,"record_offset":13212,"parent_record_offset":12228,"ancestor_record_offset":12228,"inlinee":12247,"source_line":23,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":12228,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":12228,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":14416,"parent_record_offset":14392,"ancestor_record_offset":12228,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":13624,"parent_record_offset":13596,"ancestor_record_offset":12228,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1622,"record_offset":14392,"parent_record_offset":14336,"ancestor_record_offset":12228,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":14364,"parent_record_offset":14336,"ancestor_record_offset":12228,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":13596,"parent_record_offset":13568,"ancestor_record_offset":12228,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":13568,"parent_record_offset":13540,"ancestor_record_offset":12228,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":13744,"parent_record_offset":13568,"ancestor_record_offset":12228,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":14336,"parent_record_offset":14308,"ancestor_record_offset":12228,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1622,"record_offset":13504,"parent_record_offset":13476,"ancestor_record_offset":12228,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1622,"record_offset":13476,"parent_record_offset":13448,"ancestor_record_offset":12228,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1622,"record_offset":13420,"parent_record_offset":13392,"ancestor_record_offset":12228,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1622,"record_offset":13448,"parent_record_offset":13420,"ancestor_record_offset":12228,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1622,"record_offset":13392,"parent_record_offset":13364,"ancestor_record_offset":12228,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1622,"record_offset":14168,"parent_record_offset":14120,"ancestor_record_offset":12228,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":13540,"parent_record_offset":13504,"ancestor_record_offset":12228,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":15900,"parent_record_offset":15872,"ancestor_record_offset":12228,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":15952,"parent_record_offset":15928,"ancestor_record_offset":12228,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":15928,"parent_record_offset":15900,"ancestor_record_offset":12228,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":15840,"parent_record_offset":15808,"ancestor_record_offset":12228,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":15872,"parent_record_offset":15840,"ancestor_record_offset":12228,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":16044,"parent_record_offset":15872,"ancestor_record_offset":12228,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":16156,"parent_record_offset":16132,"ancestor_record_offset":12228,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":14196,"parent_record_offset":14120,"ancestor_record_offset":12228,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1622,"record_offset":13804,"parent_record_offset":13780,"ancestor_record_offset":12228,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1622,"record_offset":15976,"parent_record_offset":15952,"ancestor_record_offset":12228,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1622,"record_offset":12604,"parent_record_offset":12508,"ancestor_record_offset":12228,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":15732,"parent_record_offset":15704,"ancestor_record_offset":12228,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":14052,"parent_record_offset":14028,"ancestor_record_offset":12228,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":15064,"parent_record_offset":14604,"ancestor_record_offset":12228,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":15780,"parent_record_offset":15600,"ancestor_record_offset":12228,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1622,"record_offset":12924,"parent_record_offset":12896,"ancestor_record_offset":12228,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1622,"record_offset":13780,"parent_record_offset":13504,"ancestor_record_offset":12228,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":14260,"parent_record_offset":14120,"ancestor_record_offset":12228,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":14840,"parent_record_offset":14816,"ancestor_record_offset":12228,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":14816,"parent_record_offset":14792,"ancestor_record_offset":12228,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":15024,"parent_record_offset":14992,"ancestor_record_offset":12228,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":13700,"parent_record_offset":13672,"ancestor_record_offset":12228,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":12228,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":14224,"parent_record_offset":14120,"ancestor_record_offset":12228,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1622,"record_offset":12540,"parent_record_offset":12508,"ancestor_record_offset":12228,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":13004,"parent_record_offset":12976,"ancestor_record_offset":12228,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":12568,"parent_record_offset":12540,"ancestor_record_offset":12228,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":13032,"parent_record_offset":13004,"ancestor_record_offset":12228,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":14792,"parent_record_offset":14768,"ancestor_record_offset":12228,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":12228,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":14768,"parent_record_offset":14660,"ancestor_record_offset":12228,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1622,"record_offset":12508,"parent_record_offset":12480,"ancestor_record_offset":12228,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1622,"record_offset":14500,"parent_record_offset":14468,"ancestor_record_offset":12228,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1622,"record_offset":14632,"parent_record_offset":14604,"ancestor_record_offset":12228,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1622,"record_offset":14604,"parent_record_offset":14572,"ancestor_record_offset":12228,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":14548,"parent_record_offset":14524,"ancestor_record_offset":12228,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":14572,"parent_record_offset":14548,"ancestor_record_offset":12228,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":14524,"parent_record_offset":14500,"ancestor_record_offset":12228,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":15096,"parent_record_offset":14572,"ancestor_record_offset":12228,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12712,"parent_record_offset":12676,"ancestor_record_offset":12228,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12872,"parent_record_offset":12712,"ancestor_record_offset":12228,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12676,"parent_record_offset":12648,"ancestor_record_offset":12228,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12416,"parent_record_offset":12376,"ancestor_record_offset":12228,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12896,"parent_record_offset":12872,"ancestor_record_offset":12228,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12480,"parent_record_offset":12444,"ancestor_record_offset":12228,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":12444,"parent_record_offset":12416,"ancestor_record_offset":12228,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":14660,"parent_record_offset":14632,"ancestor_record_offset":12228,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":14908,"parent_record_offset":14660,"ancestor_record_offset":12228,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":14880,"parent_record_offset":14660,"ancestor_record_offset":12228,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":14712,"parent_record_offset":14660,"ancestor_record_offset":12228,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":13296,"parent_record_offset":13260,"ancestor_record_offset":12228,"inlinee":30606,"source_line":2665,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":12228,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":13320,"parent_record_offset":13296,"ancestor_record_offset":12228,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":13908,"parent_record_offset":13884,"ancestor_record_offset":12228,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":15520,"parent_record_offset":15480,"ancestor_record_offset":12228,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1622,"record_offset":15644,"parent_record_offset":15600,"ancestor_record_offset":12228,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":15808,"parent_record_offset":15600,"ancestor_record_offset":12228,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":16080,"parent_record_offset":15808,"ancestor_record_offset":12228,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1622,"record_offset":13236,"parent_record_offset":13212,"ancestor_record_offset":12228,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1622,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":12228,"inlinee":30605,"source_line":107,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1622,"record_offset":15672,"parent_record_offset":15644,"ancestor_record_offset":12228,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1622,"record_offset":15704,"parent_record_offset":15672,"ancestor_record_offset":12228,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1622,"record_offset":15600,"parent_record_offset":15560,"ancestor_record_offset":12228,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1622,"record_offset":15560,"parent_record_offset":15520,"ancestor_record_offset":12228,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::present_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 509
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11298
// 首条位置: {"module":1622,"inlinee":70296}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::definitions::PresentInfoKHR::image_indices
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1622,"record_offset":20104,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":70298,"source_line":9161,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":20876,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":20904,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":21452,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":20440,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1622,"record_offset":20132,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":20932,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":20792,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":18964,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":17376,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":17952,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1622,"record_offset":20032,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":30881,"source_line":427,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::queue
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":1622,"record_offset":20304,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":30506,"source_line":132,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":20340,"parent_record_offset":20304,"ancestor_record_offset":17252,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19384,"parent_record_offset":19356,"ancestor_record_offset":17252,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19436,"parent_record_offset":19412,"ancestor_record_offset":17252,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19412,"parent_record_offset":19384,"ancestor_record_offset":17252,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19324,"parent_record_offset":19292,"ancestor_record_offset":17252,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19356,"parent_record_offset":19324,"ancestor_record_offset":17252,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19528,"parent_record_offset":19356,"ancestor_record_offset":17252,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1622,"record_offset":19640,"parent_record_offset":19616,"ancestor_record_offset":17252,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1622,"record_offset":19460,"parent_record_offset":19436,"ancestor_record_offset":17252,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1622,"record_offset":17588,"parent_record_offset":17492,"ancestor_record_offset":17252,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":19216,"parent_record_offset":19188,"ancestor_record_offset":17252,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":20060,"parent_record_offset":20032,"ancestor_record_offset":17252,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":18548,"parent_record_offset":18088,"ancestor_record_offset":17252,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":19264,"parent_record_offset":19084,"ancestor_record_offset":17252,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1622,"record_offset":17900,"parent_record_offset":17872,"ancestor_record_offset":17252,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":19592,"parent_record_offset":19292,"ancestor_record_offset":17252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":18324,"parent_record_offset":18300,"ancestor_record_offset":17252,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":18300,"parent_record_offset":18276,"ancestor_record_offset":17252,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":18508,"parent_record_offset":18476,"ancestor_record_offset":17252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":19484,"parent_record_offset":19460,"ancestor_record_offset":17252,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":19992,"parent_record_offset":19044,"ancestor_record_offset":17252,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":20240,"parent_record_offset":20156,"ancestor_record_offset":17252,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1622,"record_offset":20264,"parent_record_offset":20240,"ancestor_record_offset":17252,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1622,"record_offset":17524,"parent_record_offset":17492,"ancestor_record_offset":17252,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":20368,"parent_record_offset":20340,"ancestor_record_offset":17252,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":17552,"parent_record_offset":17524,"ancestor_record_offset":17252,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1622,"record_offset":20396,"parent_record_offset":20368,"ancestor_record_offset":17252,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":17252,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":18604,"parent_record_offset":18580,"ancestor_record_offset":17252,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":18252,"parent_record_offset":18144,"ancestor_record_offset":17252,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1622,"record_offset":17492,"parent_record_offset":17464,"ancestor_record_offset":17252,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1622,"record_offset":17984,"parent_record_offset":17952,"ancestor_record_offset":17252,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1622,"record_offset":18116,"parent_record_offset":18088,"ancestor_record_offset":17252,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1622,"record_offset":18088,"parent_record_offset":18056,"ancestor_record_offset":17252,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":18032,"parent_record_offset":18008,"ancestor_record_offset":17252,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":17252,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":18008,"parent_record_offset":17984,"ancestor_record_offset":17252,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1622,"record_offset":18580,"parent_record_offset":18056,"ancestor_record_offset":17252,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17688,"parent_record_offset":17656,"ancestor_record_offset":17252,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17848,"parent_record_offset":17688,"ancestor_record_offset":17252,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17656,"parent_record_offset":17632,"ancestor_record_offset":17252,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17408,"parent_record_offset":17376,"ancestor_record_offset":17252,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17872,"parent_record_offset":17848,"ancestor_record_offset":17252,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17464,"parent_record_offset":17432,"ancestor_record_offset":17252,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1622,"record_offset":17432,"parent_record_offset":17408,"ancestor_record_offset":17252,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":18144,"parent_record_offset":18116,"ancestor_record_offset":17252,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":18392,"parent_record_offset":18144,"ancestor_record_offset":17252,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":18364,"parent_record_offset":18144,"ancestor_record_offset":17252,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1622,"record_offset":18196,"parent_record_offset":18144,"ancestor_record_offset":17252,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20520,"parent_record_offset":20488,"ancestor_record_offset":17252,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20184,"parent_record_offset":20156,"ancestor_record_offset":17252,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20816,"parent_record_offset":20792,"ancestor_record_offset":17252,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20544,"parent_record_offset":20520,"ancestor_record_offset":17252,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20208,"parent_record_offset":20184,"ancestor_record_offset":17252,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":20840,"parent_record_offset":20816,"ancestor_record_offset":17252,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1622,"record_offset":20464,"parent_record_offset":20440,"ancestor_record_offset":17252,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1622,"record_offset":20156,"parent_record_offset":20132,"ancestor_record_offset":17252,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1622,"record_offset":20488,"parent_record_offset":20464,"ancestor_record_offset":17252,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":19004,"parent_record_offset":18964,"ancestor_record_offset":17252,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1622,"record_offset":19128,"parent_record_offset":19084,"ancestor_record_offset":17252,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":19292,"parent_record_offset":19084,"ancestor_record_offset":17252,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1622,"record_offset":19564,"parent_record_offset":19292,"ancestor_record_offset":17252,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1622,"record_offset":19156,"parent_record_offset":19128,"ancestor_record_offset":17252,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1622,"record_offset":19188,"parent_record_offset":19156,"ancestor_record_offset":17252,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1622,"record_offset":19084,"parent_record_offset":19044,"ancestor_record_offset":17252,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1622,"record_offset":19044,"parent_record_offset":19004,"ancestor_record_offset":17252,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::impl$2::present_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 533
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11352
// 首条位置: {"module":1624,"inlinee":70482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::extensions_generated::khr::swapchain::Device::queue_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1624,"record_offset":3748,"parent_record_offset":3720,"ancestor_record_offset":3720,"inlinee":70483,"source_line":81,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::retire
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 580
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1129B
// 首条位置: {"module":1622,"inlinee":70299}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::extensions_generated::ext::full_screen_exclusive::Device::release_full_screen_exclusive_mode
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/ext/full_screen_exclusive.rs
//     evidence={"module":1622,"record_offset":22632,"parent_record_offset":22572,"ancestor_record_offset":22572,"inlinee":70301,"source_line":20,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1622,"record_offset":22800,"parent_record_offset":22572,"ancestor_record_offset":22572,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":22884,"parent_record_offset":22572,"ancestor_record_offset":22572,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":22828,"parent_record_offset":22572,"ancestor_record_offset":22572,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1622,"record_offset":22856,"parent_record_offset":22572,"ancestor_record_offset":22572,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1622,"record_offset":22716,"parent_record_offset":22572,"ancestor_record_offset":22572,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":22572,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1622,"record_offset":22680,"parent_record_offset":22656,"ancestor_record_offset":22572,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":22740,"parent_record_offset":22716,"ancestor_record_offset":22572,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1622,"record_offset":22764,"parent_record_offset":22740,"ancestor_record_offset":22572,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::swapchain::SwapchainInner::dispose
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 610
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1129E
// 首条位置: {"module":1622,"inlinee":70302}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23880,"parent_record_offset":23632,"ancestor_record_offset":23632,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1622,"record_offset":24116,"parent_record_offset":23632,"ancestor_record_offset":23632,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1622,"record_offset":24516,"parent_record_offset":23632,"ancestor_record_offset":23632,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1622,"record_offset":23700,"parent_record_offset":23632,"ancestor_record_offset":23632,"inlinee":30801,"source_line":119,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":23632,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23784,"parent_record_offset":23748,"ancestor_record_offset":23632,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23916,"parent_record_offset":23880,"ancestor_record_offset":23632,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23972,"parent_record_offset":23880,"ancestor_record_offset":23632,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23748,"parent_record_offset":23724,"ancestor_record_offset":23632,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":24000,"parent_record_offset":23880,"ancestor_record_offset":23632,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1622,"record_offset":24248,"parent_record_offset":24224,"ancestor_record_offset":23632,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1622,"record_offset":24224,"parent_record_offset":24116,"ancestor_record_offset":23632,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1622,"record_offset":24076,"parent_record_offset":24052,"ancestor_record_offset":23632,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1622,"record_offset":24188,"parent_record_offset":24148,"ancestor_record_offset":23632,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":24272,"parent_record_offset":24248,"ancestor_record_offset":23632,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":23840,"parent_record_offset":23748,"ancestor_record_offset":23632,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1622,"record_offset":24028,"parent_record_offset":23880,"ancestor_record_offset":23632,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1622,"record_offset":24052,"parent_record_offset":24028,"ancestor_record_offset":23632,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1622,"record_offset":24148,"parent_record_offset":24116,"ancestor_record_offset":23632,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$7::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1622,"record_offset":23724,"parent_record_offset":23700,"ancestor_record_offset":23632,"inlinee":70058,"source_line":227,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$8::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1622,"record_offset":24344,"parent_record_offset":24312,"ancestor_record_offset":23632,"inlinee":70062,"source_line":233,"archive":"_pdb/A/modules/01622.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::swapchain::ManagedSwapchain>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::swapchain::ManagedSwapchain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::swapchain::ManagedSwapchain> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::swapchain::ManagedSwapchain>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::swapchain::ManagedSwapchain>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::swapchain::ManagedSwapchain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::swapchain::ManagedSwapchain>", "constructor": "rhapsody::raw::managed::resources::impl$5::dispose", "arguments": ["rhapsody::raw::managed::swapchain::ManagedSwapchain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain>", "constructor": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0", "arguments": ["rhapsody::raw::managed::swapchain::ManagedSwapchain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::swapchain::ManagedSwapchain>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::swapchain::ManagedSwapchain"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::swapchain::ManagedSwapchain> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::swapchain::ManagedSwapchain>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::swapchain::SwapchainInner
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 12; 过程引用数: 0
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next
// 信息 ID: 3e1cef2c5da5bd160b15f17b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::Swapchain::acquire_next_texture @ 0x737f6e -> 0x118a110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::desc
// 信息 ID: d47627b76e25cc72b9197fb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::extent; module=569, inline_record=3508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::mark_stale
// 信息 ID: 0b1ec4694d894e2f8d502f69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::mark_stale; module=569, inline_record=2736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::new
// 信息 ID: 685bf45b4f37da04a6220d09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_swapchain @ 0x743e72 -> 0x11890d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::on_focus
// 信息 ID: 8ecaa6e46e51a429f5bb0a33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::on_focus; module=569, inline_record=1312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::present
// 信息 ID: 847cef72dfa92cf10fec81be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::Swapchain::present @ 0x738185 -> 0x118a960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::resize
// 信息 ID: 31c31b5059a4c9901531171a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::Swapchain::resize @ 0x737bba -> 0x1189e40 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::retire
// 信息 ID: 9e54f90cf2fc905c819f42b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_swapchain @ 0x743e38 -> 0x118b1c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::set_fullscreen_exclusive
// 信息 ID: 5d14ed32681080fbd1253cb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::ManagedSwapchain::set_vsync
// 信息 ID: cb73de48329ba18a0d2779c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::swapchain::Swapchain::set_vsync @ 0x737c6a -> 0x1189fb0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::acquire_next_image
// 信息 ID: 9f97a5b9e742af85db89d2f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next; module=1622, inline_record=12228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::dispose
// 信息 ID: b2ef4f5ce22fe7e284be019a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::impl$1::dispose; module=1622, inline_record=23632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
// 信息 ID: 2637887a44d6f143b6caad83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::mark_stale; module=569, inline_record=2944; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next; module=1622, inline_record=14028; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::present; module=1622, inline_record=20032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::new
// 信息 ID: b0649ab7c751696a47dbdd9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::new; module=1622, inline_record=5180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::new_impl
// 信息 ID: 6891cce10a773d2a96be548a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::new; module=1622, inline_record=5216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::on_focus
// 信息 ID: 1563326f784d36f260e20bb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::on_focus; module=569, inline_record=1520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::present_image
// 信息 ID: 8379ebefbcb6c99c0f68f6cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::present; module=1622, inline_record=17252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::resize
// 信息 ID: ba31b5294b7eaa1dc429d439
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::resize; module=1622, inline_record=10336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::retire
// 信息 ID: 81498533f2055c0327174fde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::retire; module=1622, inline_record=22572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::set_fullscreen_exclusive
// 信息 ID: e2bc7f38916bbda0b3d84f11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::SwapchainInner::set_vsync
// 信息 ID: 7f5949533b0af920a3dac616
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::set_vsync; module=1622, inline_record=11280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$1::dispose
// 信息 ID: 17c46a12863ef6da4baa42dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::swapchain::ManagedSwapchain> @ 0x1178329 -> 0x118b3e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::new::closure$0
// 信息 ID: 1b0968f5266da422fce95741
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::new; module=1622, inline_record=9480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$2
// 信息 ID: c517254e983d4d1a8cb38f5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::new_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::ManagedSwapchain::new; module=1622, inline_record=6476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$5
// 信息 ID: da687e335e659f50b74698a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::new_impl", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5 <- alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::texture::Texture,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::texture::Texture,alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::texture::Texture,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::texture::Texture,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::texture::Texture,alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::texture::Texture,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::texture::Texture,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::texture::Texture,alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::texture::Texture,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::texture::Texture,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> > > >; module=3007, inline_record=14324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::new_impl::closure$6
// 信息 ID: ae9f2a23ecb8f428762a1f79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::new_impl", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$6 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$6> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::Semaphore,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$6> >; module=1593, inline_record=19728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::swapchain::impl$2::present_image::closure$0
// 信息 ID: 56b57fcc8e36d1b148d67088
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::managed::swapchain::impl$2::present_image", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0 <- rhapsody::raw::device::Queue::with_raw<rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::raw::device::Queue::with_raw<rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >; module=1624, inline_record=3720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
