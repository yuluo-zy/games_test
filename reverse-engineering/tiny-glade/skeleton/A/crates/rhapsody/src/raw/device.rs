// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/device.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: eb89cc60bc94f1c4e6400fe937b03f0c0d39f6024e637924dca72a51b073b2d7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: device.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 127, 139, 551, 556, 565, 570, 571, 1332, 1592, 1593, 1594, 1624, 1974, 3007
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use bumpalo; // namespace candidate
//   package identities: [{"name":"bumpalo","source":"PDB_registry_path_only","version":"3.12.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"PDB_registry_path_only","version":"0.27.2"}]
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
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":30300,"rva":7585040,"va":5376294160}
// 编译实例 RVAs: 0x73bd10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bumpalo..Bump$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t602686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e83e320900","size":5,"soft_fail":false,"target_va":5376902512,"va":5376299821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$gpu_allocator..AllocatorDebugSettings$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10816509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8fd0ba500","size":5,"soft_fail":false,"target_va":5387118592,"va":5376302078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140065920
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7170188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8749792ff","size":5,"soft_fail":false,"target_va":5369125152,"va":5376295335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6680816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8100f9aff","size":5,"soft_fail":false,"target_va":5369614224,"va":5376295035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceb20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4657340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e844efb8ff","size":5,"soft_fail":false,"target_va":5371652896,"va":5376310231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d06f0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4635511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e88944b9ff","size":5,"soft_fail":false,"target_va":5371660016,"va":5376295522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3924042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8b61fc4ff","size":5,"soft_fail":false,"target_va":5372390432,"va":5376314469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3544867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8dde8c9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376294558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3545567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e821e6c9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376295274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8ca0
//     owner=unresolved_namespace; count=3
//     target_aliases=19 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-418012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8249ff9ff","size":5,"soft_fail":false,"target_va":5375888544,"va":5376306551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b630
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-79756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e874c8feff","size":5,"soft_fail":false,"target_va":5376226864,"va":5376306615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-82170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e806bffeff","size":5,"soft_fail":false,"target_va":5376227344,"va":5376309509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-82625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e83fbdfeff","size":5,"soft_fail":false,"target_va":5376227392,"va":5376310012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072bd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-80390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8fac5feff","size":5,"soft_fail":false,"target_va":5376228624,"va":5376309009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072bfe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-81322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e856c2feff","size":5,"soft_fail":false,"target_va":5376229344,"va":5376310661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-79354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e806cafeff","size":5,"soft_fail":false,"target_va":5376229648,"va":5376308997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c5b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-75769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e807d8feff","size":5,"soft_fail":false,"target_va":5376230832,"va":5376306596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072d0b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-72934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e81ae3feff","size":5,"soft_fail":false,"target_va":5376233648,"va":5376306577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072d290
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-72409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e827e5feff","size":5,"soft_fail":false,"target_va":5376234128,"va":5376306532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-66521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e827fcfeff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376306436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d0550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t606043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e85b3f0900","size":5,"soft_fail":false,"target_va":5376902480,"va":5376296432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141175af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10706898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8d25fa300","size":5,"soft_fail":false,"target_va":5387016944,"va":5376310041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141175b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10707233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e82161a300","size":5,"soft_fail":false,"target_va":5387017008,"va":5376309770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117b620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10736023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e897d1a300","size":5,"soft_fail":false,"target_va":5387040288,"va":5376304260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117d820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10743340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e82ceea300","size":5,"soft_fail":false,"target_va":5387048992,"va":5376305647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26605930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e86af99501","size":5,"soft_fail":false,"target_va":5402900112,"va":5376294177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27298114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e84289a001","size":5,"soft_fail":false,"target_va":5403593344,"va":5376295225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27295985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8f180a001","size":5,"soft_fail":false,"target_va":5403593360,"va":5376297370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27299425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8618ea001","size":5,"soft_fail":false,"target_va":5403594400,"va":5376294970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-26153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8d799ffff","size":5,"soft_fail":false,"target_va":5376280688,"va":5376306836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26678253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8ed139701","size":5,"soft_fail":false,"target_va":5402978144,"va":5376299886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::_$LT$impl$u20$core..ffi..c_str..CStr$GT$::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1234862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8aed71200","size":5,"soft_fail":false,"target_va":5377529856,"va":5376294989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1235445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8f5d91200","size":5,"soft_fail":false,"target_va":5377529888,"va":5376294438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4988467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8cde1b3ff","size":5,"soft_fail":false,"target_va":5371306912,"va":5376295374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc::new<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::allocator::GpuAllocator> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-26647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8e997ffff","size":5,"soft_fail":false,"target_va":5376280496,"va":5376307138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::extensions_generated::ext::debug_utils::Device::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10801341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8bdd0a400","size":5,"soft_fail":false,"target_va":5387107168,"va":5376305822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::extensions_generated::ext::full_screen_exclusive::Device::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10813488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e83000a500","size":5,"soft_fail":false,"target_va":5387119200,"va":5376305707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::extensions_generated::khr::swapchain::Device::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10814181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8e502a500","size":5,"soft_fail":false,"target_va":5387120144,"va":5376305958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::instance::Instance::create_device
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e884b3ffff","size":5,"soft_fail":false,"target_va":5376279120,"va":5376298695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::alloc_layout_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t606069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8753f0900","size":5,"soft_fail":false,"target_va":5376902656,"va":5376296582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26674932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8f4069701","size":5,"soft_fail":false,"target_va":5402971536,"va":5376296599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26670507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8abf59601","size":5,"soft_fail":false,"target_va":5402979152,"va":5376308640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26671375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e80ff99601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376308748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::AllocationSizes::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10816504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8f80ba500","size":5,"soft_fail":false,"target_va":5387118608,"va":5376302099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lock_api::mutex::impl$7::default<parking_lot::raw_mutex::RawMutex,rhapsody::raw::crash::CrashMarkerNames>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8e09affff","size":5,"soft_fail":false,"target_va":5376281360,"va":5376307243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t1057705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8a9231000","size":5,"soft_fail":false,"target_va":5377353776,"va":5376296066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-79647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8e1c8feff","size":5,"soft_fail":false,"target_va":5376223984,"va":5376303626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBufferPool::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8e9660000","size":5,"soft_fail":false,"target_va":5376333616,"va":5376307266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::create_buffer_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8bea0ffff","size":5,"soft_fail":false,"target_va":5376281760,"va":5376306141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::required_extensions::closure$0
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t23220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8b45a0000","size":5,"soft_fail":false,"target_va":5376317968,"va":5376294743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::disposal::impl$14::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t24214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8965e0000","size":5,"soft_fail":false,"target_va":5376331552,"va":5376307333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::resources::ManagedResources::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-90527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e8619efeff","size":5,"soft_fail":false,"target_va":5376215472,"va":5376305994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_bytes
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-85967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e831b0feff","size":5,"soft_fail":false,"target_va":5376213072,"va":5376299034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::semaphore::create_raw_timeline_semaphore
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10815029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376294160,"raw_hex":"e83506a500","size":5,"soft_fail":false,"target_va":5387119008,"va":5376303974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5376294160,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5376297584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":35152,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":45092,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":570,"record_offset":43460,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":12811,"source_line":151,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":35904,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":45128,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":36884,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_pipeline_cache
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":44636,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30994,"source_line":2236,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::get_device_queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":43332,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30980,"source_line":2357,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::impl$5::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":39864,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30969,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_support
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":570,"record_offset":43364,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30981,"source_line":12,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::instance::impl$4::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":570,"record_offset":39356,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30961,"source_line":13,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DeviceQueueCreateInfo::queue_family_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":35332,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30942,"source_line":1274,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DeviceQueueCreateInfo::queue_priorities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":35876,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30944,"source_line":1279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$268::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":44608,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30993,"source_line":5167,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$35::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":35304,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30940,"source_line":1251,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":570,"record_offset":35360,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30943,"source_line":1169,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_copy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":570,"record_offset":35988,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30945,"source_line":1303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":570,"record_offset":44396,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":34624,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":38476,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":38964,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":34652,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":570,"record_offset":34072,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":43544,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::next_multiple_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":38824,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30959,"source_line":2641,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":44508,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":33884,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":42856,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":45632,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":38324,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":30712,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU16::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":44452,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30991,"source_line":2665,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":43132,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":9703,"source_line":154,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":570,"record_offset":34680,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":570,"record_offset":34540,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":570,"record_offset":44540,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30618,"source_line":116,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::create_samplers
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/sampler.rs
//     evidence={"module":570,"record_offset":43628,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30984,"source_line":24,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::validate_physical_device_requirements
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":30592,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30934,"source_line":658,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::create_raw_device
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":37020,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30947,"source_line":699,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::required_extensions
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":30740,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30936,"source_line":864,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$15::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":570,"record_offset":43516,"parent_record_offset":30300,"ancestor_record_offset":30300,"inlinee":30983,"source_line":51,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":30916,"parent_record_offset":30892,"ancestor_record_offset":30300,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":30940,"parent_record_offset":30916,"ancestor_record_offset":30300,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":31548,"parent_record_offset":31520,"ancestor_record_offset":30300,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":30860,"parent_record_offset":30740,"ancestor_record_offset":30300,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":30892,"parent_record_offset":30860,"ancestor_record_offset":30300,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":31520,"parent_record_offset":31456,"ancestor_record_offset":30300,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":46736,"parent_record_offset":46712,"ancestor_record_offset":30300,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":570,"record_offset":43484,"parent_record_offset":43460,"ancestor_record_offset":30300,"inlinee":12812,"source_line":542,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":570,"record_offset":30624,"parent_record_offset":30592,"ancestor_record_offset":30300,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":570,"record_offset":30672,"parent_record_offset":30648,"ancestor_record_offset":30300,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":31488,"parent_record_offset":31456,"ancestor_record_offset":30300,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":31320,"parent_record_offset":31288,"ancestor_record_offset":30300,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":31456,"parent_record_offset":31428,"ancestor_record_offset":30300,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":31036,"parent_record_offset":31012,"ancestor_record_offset":30300,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":31012,"parent_record_offset":30740,"ancestor_record_offset":30300,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":33944,"parent_record_offset":33908,"ancestor_record_offset":30300,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":33908,"parent_record_offset":33884,"ancestor_record_offset":30300,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":31288,"parent_record_offset":31236,"ancestor_record_offset":30300,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":31060,"parent_record_offset":31036,"ancestor_record_offset":30300,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":33124,"parent_record_offset":30740,"ancestor_record_offset":30300,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":31096,"parent_record_offset":30740,"ancestor_record_offset":30300,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":31236,"parent_record_offset":30740,"ancestor_record_offset":30300,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":43568,"parent_record_offset":43544,"ancestor_record_offset":30300,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":44252,"parent_record_offset":43628,"ancestor_record_offset":30300,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":31124,"parent_record_offset":30740,"ancestor_record_offset":30300,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::instance::Instance::get_physical_device_features2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":570,"record_offset":37556,"parent_record_offset":37020,"ancestor_record_offset":30300,"inlinee":30956,"source_line":151,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$32::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":570,"record_offset":39388,"parent_record_offset":39356,"ancestor_record_offset":30300,"inlinee":30963,"source_line":112,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$33::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":570,"record_offset":39944,"parent_record_offset":39864,"ancestor_record_offset":30300,"inlinee":30973,"source_line":395,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$35::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":570,"record_offset":39624,"parent_record_offset":39356,"ancestor_record_offset":30300,"inlinee":30965,"source_line":2713,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$36::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":570,"record_offset":42348,"parent_record_offset":39864,"ancestor_record_offset":30300,"inlinee":30977,"source_line":2966,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$39::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":570,"record_offset":42616,"parent_record_offset":39864,"ancestor_record_offset":30300,"inlinee":30979,"source_line":3307,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$41::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":570,"record_offset":39804,"parent_record_offset":39356,"ancestor_record_offset":30300,"inlinee":30967,"source_line":3574,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::tables::impl$42::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/tables.rs
//     evidence={"module":570,"record_offset":41776,"parent_record_offset":39864,"ancestor_record_offset":30300,"inlinee":30975,"source_line":3610,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::DeviceCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":38076,"parent_record_offset":37020,"ancestor_record_offset":30300,"inlinee":30958,"source_line":1383,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::PhysicalDeviceFeatures2::push_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":37148,"parent_record_offset":37020,"ancestor_record_offset":30300,"inlinee":30955,"source_line":11056,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$2798::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":37092,"parent_record_offset":37020,"ancestor_record_offset":30300,"inlinee":30951,"source_line":30394,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$287::default
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":43884,"parent_record_offset":43860,"ancestor_record_offset":30300,"inlinee":30988,"source_line":5362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$2906::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":37064,"parent_record_offset":37020,"ancestor_record_offset":30300,"inlinee":30949,"source_line":31432,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$6448::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":570,"record_offset":39912,"parent_record_offset":39864,"ancestor_record_offset":30300,"inlinee":30971,"source_line":127,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$646::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":37120,"parent_record_offset":37020,"ancestor_record_offset":30300,"inlinee":30953,"source_line":11031,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":44668,"parent_record_offset":44636,"ancestor_record_offset":30300,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":43400,"parent_record_offset":43364,"ancestor_record_offset":30300,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":43424,"parent_record_offset":43400,"ancestor_record_offset":30300,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":570,"record_offset":37296,"parent_record_offset":37272,"ancestor_record_offset":30300,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":570,"record_offset":35388,"parent_record_offset":35360,"ancestor_record_offset":30300,"inlinee":6155,"source_line":1396,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":570,"record_offset":36020,"parent_record_offset":35988,"ancestor_record_offset":30300,"inlinee":30946,"source_line":1267,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":570,"record_offset":35412,"parent_record_offset":35388,"ancestor_record_offset":30300,"inlinee":6156,"source_line":1411,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":570,"record_offset":35448,"parent_record_offset":35412,"ancestor_record_offset":30300,"inlinee":6157,"source_line":1420,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$8::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":570,"record_offset":35808,"parent_record_offset":35780,"ancestor_record_offset":30300,"inlinee":6179,"source_line":1397,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":570,"record_offset":36076,"parent_record_offset":36020,"ancestor_record_offset":30300,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":570,"record_offset":36100,"parent_record_offset":36076,"ancestor_record_offset":30300,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":35492,"parent_record_offset":35448,"ancestor_record_offset":30300,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":35684,"parent_record_offset":35660,"ancestor_record_offset":30300,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":35660,"parent_record_offset":35448,"ancestor_record_offset":30300,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=106
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":570,"record_offset":39424,"parent_record_offset":39388,"ancestor_record_offset":30300,"inlinee":6771,"source_line":160,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":36676,"parent_record_offset":36636,"ancestor_record_offset":30300,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":32696,"parent_record_offset":32668,"ancestor_record_offset":30300,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":570,"record_offset":33096,"parent_record_offset":30740,"ancestor_record_offset":30300,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":570,"record_offset":32056,"parent_record_offset":32032,"ancestor_record_offset":30300,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":570,"record_offset":32140,"parent_record_offset":32116,"ancestor_record_offset":30300,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":31668,"parent_record_offset":30740,"ancestor_record_offset":30300,"inlinee":9936,"source_line":274,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":31696,"parent_record_offset":31668,"ancestor_record_offset":30300,"inlinee":9938,"source_line":739,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen::strlen_rt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":31720,"parent_record_offset":31696,"ancestor_record_offset":30300,"inlinee":9940,"source_line":752,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":570,"record_offset":32080,"parent_record_offset":32056,"ancestor_record_offset":30300,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":570,"record_offset":34104,"parent_record_offset":34072,"ancestor_record_offset":30300,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":570,"record_offset":44332,"parent_record_offset":44308,"ancestor_record_offset":30300,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":570,"record_offset":37200,"parent_record_offset":37176,"ancestor_record_offset":30300,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":570,"record_offset":37224,"parent_record_offset":37200,"ancestor_record_offset":30300,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":570,"record_offset":37272,"parent_record_offset":37248,"ancestor_record_offset":30300,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":570,"record_offset":36712,"parent_record_offset":36636,"ancestor_record_offset":30300,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":570,"record_offset":36636,"parent_record_offset":36600,"ancestor_record_offset":30300,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":570,"record_offset":36600,"parent_record_offset":36020,"ancestor_record_offset":30300,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":570,"record_offset":43592,"parent_record_offset":43568,"ancestor_record_offset":30300,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":44364,"parent_record_offset":43628,"ancestor_record_offset":30300,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":37176,"parent_record_offset":37148,"ancestor_record_offset":30300,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":44308,"parent_record_offset":43628,"ancestor_record_offset":30300,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":37248,"parent_record_offset":37224,"ancestor_record_offset":30300,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":35708,"parent_record_offset":35684,"ancestor_record_offset":30300,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":36740,"parent_record_offset":36712,"ancestor_record_offset":30300,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":33268,"parent_record_offset":33236,"ancestor_record_offset":30300,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":570,"record_offset":32256,"parent_record_offset":32232,"ancestor_record_offset":30300,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":570,"record_offset":34872,"parent_record_offset":34760,"ancestor_record_offset":30300,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":34252,"parent_record_offset":34072,"ancestor_record_offset":30300,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":30648,"parent_record_offset":30624,"ancestor_record_offset":30300,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":31380,"parent_record_offset":31236,"ancestor_record_offset":30300,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":31196,"parent_record_offset":31172,"ancestor_record_offset":30300,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":32600,"parent_record_offset":32576,"ancestor_record_offset":30300,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":32492,"parent_record_offset":32468,"ancestor_record_offset":30300,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":35572,"parent_record_offset":35548,"ancestor_record_offset":30300,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":35548,"parent_record_offset":35448,"ancestor_record_offset":30300,"inlinee":6158,"source_line":1299,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":33976,"parent_record_offset":33944,"ancestor_record_offset":30300,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":31636,"parent_record_offset":31604,"ancestor_record_offset":30300,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":30968,"parent_record_offset":30940,"ancestor_record_offset":30300,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":31352,"parent_record_offset":31236,"ancestor_record_offset":30300,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":38280,"parent_record_offset":37020,"ancestor_record_offset":30300,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":35780,"parent_record_offset":35388,"ancestor_record_offset":30300,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":570,"record_offset":32780,"parent_record_offset":32752,"ancestor_record_offset":30300,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":570,"record_offset":32808,"parent_record_offset":32780,"ancestor_record_offset":30300,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":31148,"parent_record_offset":31124,"ancestor_record_offset":30300,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":570,"record_offset":31172,"parent_record_offset":31148,"ancestor_record_offset":30300,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":34132,"parent_record_offset":34104,"ancestor_record_offset":30300,"inlinee":8711,"source_line":296,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":31604,"parent_record_offset":30740,"ancestor_record_offset":30300,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":32724,"parent_record_offset":32696,"ancestor_record_offset":30300,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":45152,"parent_record_offset":45128,"ancestor_record_offset":30300,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":34564,"parent_record_offset":34540,"ancestor_record_offset":30300,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":44476,"parent_record_offset":44452,"ancestor_record_offset":30300,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":34588,"parent_record_offset":34564,"ancestor_record_offset":30300,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":34004,"parent_record_offset":33944,"ancestor_record_offset":30300,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":34036,"parent_record_offset":33908,"ancestor_record_offset":30300,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":570,"record_offset":32668,"parent_record_offset":32640,"ancestor_record_offset":30300,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":31812,"parent_record_offset":31784,"ancestor_record_offset":30300,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":31840,"parent_record_offset":31812,"ancestor_record_offset":30300,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":32640,"parent_record_offset":32420,"ancestor_record_offset":30300,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":31876,"parent_record_offset":31840,"ancestor_record_offset":30300,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32552,"parent_record_offset":32528,"ancestor_record_offset":30300,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32528,"parent_record_offset":32420,"ancestor_record_offset":30300,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32468,"parent_record_offset":32444,"ancestor_record_offset":30300,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32388,"parent_record_offset":31952,"ancestor_record_offset":30300,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32444,"parent_record_offset":32420,"ancestor_record_offset":30300,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":31928,"parent_record_offset":31904,"ancestor_record_offset":30300,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":31904,"parent_record_offset":31840,"ancestor_record_offset":30300,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":31952,"parent_record_offset":31928,"ancestor_record_offset":30300,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":32200,"parent_record_offset":32172,"ancestor_record_offset":30300,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":32232,"parent_record_offset":32200,"ancestor_record_offset":30300,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":32172,"parent_record_offset":31952,"ancestor_record_offset":30300,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32004,"parent_record_offset":31952,"ancestor_record_offset":30300,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32420,"parent_record_offset":31952,"ancestor_record_offset":30300,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":32032,"parent_record_offset":31952,"ancestor_record_offset":30300,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":32116,"parent_record_offset":31952,"ancestor_record_offset":30300,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":32296,"parent_record_offset":31952,"ancestor_record_offset":30300,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":570,"record_offset":31784,"parent_record_offset":31756,"ancestor_record_offset":30300,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::impl$4::create::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":34896,"parent_record_offset":34872,"ancestor_record_offset":30300,"inlinee":30938,"source_line":281,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::impl$4::create::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":44420,"parent_record_offset":44396,"ancestor_record_offset":30300,"inlinee":30989,"source_line":447,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::sampler::impl$0::create_samplers::closure$0
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/sampler.rs
//     evidence={"module":570,"record_offset":43860,"parent_record_offset":43628,"ancestor_record_offset":30300,"inlinee":30986,"source_line":25,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":570,"record_offset":31756,"parent_record_offset":30740,"ancestor_record_offset":30300,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::required_extensions::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":54524,"rva":7608848,"va":5376317968}
// 编译实例 RVAs: 0x741a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6703885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317968,"raw_hex":"e8f3b499ff","size":5,"soft_fail":false,"target_va":5369614224,"va":5376318104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27275065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317968,"raw_hex":"e8392fa001","size":5,"soft_fail":false,"target_va":5403593344,"va":5376318274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5013495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317968,"raw_hex":"e80980b3ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5376318546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ffi::c_str::CStr::to_str
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1241512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317968,"raw_hex":"e8a8f11200","size":5,"soft_fail":false,"target_va":5377559552,"va":5376318035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26661415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317968,"raw_hex":"e827d29601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376318708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1035257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317968,"raw_hex":"e8f9cb0f00","size":5,"soft_fail":false,"target_va":5377353776,"va":5376318514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":56052,"parent_record_offset":54524,"ancestor_record_offset":54524,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":55932,"parent_record_offset":54524,"ancestor_record_offset":54524,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":55996,"parent_record_offset":54524,"ancestor_record_offset":54524,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":54688,"parent_record_offset":54524,"ancestor_record_offset":54524,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":570,"record_offset":56024,"parent_record_offset":54524,"ancestor_record_offset":54524,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":570,"record_offset":55848,"parent_record_offset":54524,"ancestor_record_offset":54524,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":570,"record_offset":54724,"parent_record_offset":54524,"ancestor_record_offset":54524,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":56112,"parent_record_offset":56088,"ancestor_record_offset":54524,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":56088,"parent_record_offset":56052,"ancestor_record_offset":54524,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":55644,"parent_record_offset":55616,"ancestor_record_offset":54524,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":570,"record_offset":55004,"parent_record_offset":54980,"ancestor_record_offset":54524,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":570,"record_offset":55088,"parent_record_offset":55064,"ancestor_record_offset":54524,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":570,"record_offset":55028,"parent_record_offset":55004,"ancestor_record_offset":54524,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":570,"record_offset":55204,"parent_record_offset":55180,"ancestor_record_offset":54524,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":55548,"parent_record_offset":55524,"ancestor_record_offset":54524,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":55440,"parent_record_offset":55416,"ancestor_record_offset":54524,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":56144,"parent_record_offset":56052,"ancestor_record_offset":54524,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":570,"record_offset":55728,"parent_record_offset":55700,"ancestor_record_offset":54524,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":570,"record_offset":55756,"parent_record_offset":55728,"ancestor_record_offset":54524,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":55672,"parent_record_offset":55644,"ancestor_record_offset":54524,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":55872,"parent_record_offset":55848,"ancestor_record_offset":54524,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":55896,"parent_record_offset":55872,"ancestor_record_offset":54524,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":570,"record_offset":55616,"parent_record_offset":55588,"ancestor_record_offset":54524,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":54772,"parent_record_offset":54748,"ancestor_record_offset":54524,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":54796,"parent_record_offset":54772,"ancestor_record_offset":54524,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":55588,"parent_record_offset":55368,"ancestor_record_offset":54524,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":54828,"parent_record_offset":54796,"ancestor_record_offset":54524,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":55500,"parent_record_offset":55476,"ancestor_record_offset":54524,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":55476,"parent_record_offset":55368,"ancestor_record_offset":54524,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":55416,"parent_record_offset":55392,"ancestor_record_offset":54524,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":55336,"parent_record_offset":54904,"ancestor_record_offset":54524,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":55392,"parent_record_offset":55368,"ancestor_record_offset":54524,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":54880,"parent_record_offset":54856,"ancestor_record_offset":54524,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":54856,"parent_record_offset":54796,"ancestor_record_offset":54524,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":54904,"parent_record_offset":54880,"ancestor_record_offset":54524,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":55148,"parent_record_offset":55120,"ancestor_record_offset":54524,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":55180,"parent_record_offset":55148,"ancestor_record_offset":54524,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":55120,"parent_record_offset":54904,"ancestor_record_offset":54524,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":54952,"parent_record_offset":54904,"ancestor_record_offset":54524,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":55368,"parent_record_offset":54904,"ancestor_record_offset":54524,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":54980,"parent_record_offset":54904,"ancestor_record_offset":54524,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":55064,"parent_record_offset":54904,"ancestor_record_offset":54524,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":55244,"parent_record_offset":54904,"ancestor_record_offset":54524,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":570,"record_offset":54748,"parent_record_offset":54724,"ancestor_record_offset":54524,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::Queue::with_raw
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B66
// 首条位置: {"module":139,"inlinee":11110}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":139,"record_offset":9652,"parent_record_offset":8992,"ancestor_record_offset":8992,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":139,"record_offset":10032,"parent_record_offset":8992,"ancestor_record_offset":8992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":139,"record_offset":9324,"parent_record_offset":8992,"ancestor_record_offset":8992,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":139,"record_offset":9044,"parent_record_offset":8992,"ancestor_record_offset":8992,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::impl$2::queue_submit::closure$6
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":139,"record_offset":9496,"parent_record_offset":8992,"ancestor_record_offset":8992,"inlinee":11111,"source_line":303,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$29::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":139,"record_offset":9528,"parent_record_offset":9496,"ancestor_record_offset":8992,"inlinee":5330,"source_line":3244,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::queue_submit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":139,"record_offset":9556,"parent_record_offset":9496,"ancestor_record_offset":8992,"inlinee":11113,"source_line":2464,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":139,"record_offset":9588,"parent_record_offset":9556,"ancestor_record_offset":8992,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":139,"record_offset":9612,"parent_record_offset":9588,"ancestor_record_offset":8992,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":139,"record_offset":9188,"parent_record_offset":9096,"ancestor_record_offset":8992,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":139,"record_offset":9220,"parent_record_offset":9188,"ancestor_record_offset":8992,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":139,"record_offset":9676,"parent_record_offset":9652,"ancestor_record_offset":8992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":139,"record_offset":9432,"parent_record_offset":9348,"ancestor_record_offset":8992,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":139,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":8992,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9756,"parent_record_offset":9724,"ancestor_record_offset":8992,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9376,"parent_record_offset":9348,"ancestor_record_offset":8992,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9256,"parent_record_offset":9096,"ancestor_record_offset":8992,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9960,"parent_record_offset":9928,"ancestor_record_offset":8992,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9132,"parent_record_offset":9096,"ancestor_record_offset":8992,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":8992,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9280,"parent_record_offset":9256,"ancestor_record_offset":8992,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":8992,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":8992,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":139,"record_offset":9700,"parent_record_offset":9676,"ancestor_record_offset":8992,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":139,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":8992,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":139,"record_offset":9348,"parent_record_offset":9324,"ancestor_record_offset":8992,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":139,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":8992,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":139,"record_offset":9096,"parent_record_offset":9068,"ancestor_record_offset":8992,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":139,"record_offset":9068,"parent_record_offset":9044,"ancestor_record_offset":8992,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":139,"record_offset":9928,"parent_record_offset":9904,"ancestor_record_offset":8992,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::Queue::with_raw<rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1624,"record_offset":2992,"rva":18399872,"va":5387108992}
// 编译实例 RVAs: 0x118c280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15859889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387108992,"raw_hex":"e8b100f200","size":5,"soft_fail":false,"target_va":5402968992,"va":5387109098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15861607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387108992,"raw_hex":"e86707f200","size":5,"soft_fail":false,"target_va":5402970800,"va":5387109188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15857461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387108992,"raw_hex":"e835f7f100","size":5,"soft_fail":false,"target_va":5402966736,"va":5387109270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15859116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387108992,"raw_hex":"e8acfdf100","size":5,"soft_fail":false,"target_va":5402968352,"va":5387109231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5387108992,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5387109125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1624,"record_offset":3788,"parent_record_offset":2992,"ancestor_record_offset":2992,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":4168,"parent_record_offset":2992,"ancestor_record_offset":2992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":3548,"parent_record_offset":2992,"ancestor_record_offset":2992,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1624,"record_offset":3264,"parent_record_offset":2992,"ancestor_record_offset":2992,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::impl$2::present_image::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":1624,"record_offset":3720,"parent_record_offset":2992,"ancestor_record_offset":2992,"inlinee":70482,"source_line":533,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::khr::swapchain::Device::queue_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":1624,"record_offset":3748,"parent_record_offset":3720,"ancestor_record_offset":2992,"inlinee":70483,"source_line":81,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":3412,"parent_record_offset":3324,"ancestor_record_offset":2992,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":3444,"parent_record_offset":3412,"ancestor_record_offset":2992,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":2992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":3656,"parent_record_offset":3572,"ancestor_record_offset":2992,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":3680,"parent_record_offset":3656,"ancestor_record_offset":2992,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":3892,"parent_record_offset":3860,"ancestor_record_offset":2992,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":3600,"parent_record_offset":3572,"ancestor_record_offset":2992,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":3480,"parent_record_offset":3324,"ancestor_record_offset":2992,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":4096,"parent_record_offset":4064,"ancestor_record_offset":2992,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":3356,"parent_record_offset":3324,"ancestor_record_offset":2992,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":2992,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":2992,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":2992,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":4120,"parent_record_offset":4096,"ancestor_record_offset":2992,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":2992,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1624,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":2992,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1624,"record_offset":3572,"parent_record_offset":3548,"ancestor_record_offset":2992,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1624,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":2992,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1624,"record_offset":3324,"parent_record_offset":3292,"ancestor_record_offset":2992,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1624,"record_offset":3292,"parent_record_offset":3264,"ancestor_record_offset":2992,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1624,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":2992,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$6::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B5C
// 首条位置: {"module":139,"inlinee":11100}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::Queue::update_pending_submissions
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x771B
// 首条位置: {"module":551,"inlinee":30491}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3336,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":22347,"source_line":1520,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3504,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::get_semaphore_counter_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":551,"record_offset":3220,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":30492,"source_line":679,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":3648,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":8949,"source_line":1221,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":551,"record_offset":4524,"parent_record_offset":4500,"ancestor_record_offset":4500,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3368,"parent_record_offset":3336,"ancestor_record_offset":3176,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3400,"parent_record_offset":3368,"ancestor_record_offset":3176,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3428,"parent_record_offset":3400,"ancestor_record_offset":3176,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":551,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":3176,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":551,"record_offset":3248,"parent_record_offset":3220,"ancestor_record_offset":3176,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":551,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":3176,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":551,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":3176,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":3704,"parent_record_offset":3676,"ancestor_record_offset":3176,"inlinee":4212,"source_line":1549,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$63::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":3732,"parent_record_offset":3704,"ancestor_record_offset":3176,"inlinee":4214,"source_line":1565,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$17::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":551,"record_offset":3676,"parent_record_offset":3648,"ancestor_record_offset":3176,"inlinee":30494,"source_line":77,"archive":"_pdb/A/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::Queue::allocate_command_buffer_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7968
// 首条位置: {"module":571,"inlinee":31080}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU64::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":571,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":2656,"inlinee":15668,"source_line":2665,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":571,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":2656,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::Queue::reclaim_command_buffer_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":25204,"rva":7582624,"va":5376291744}
// 编译实例 RVAs: 0x73b3a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf9b0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4636011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e89542b9ff","size":5,"soft_fail":false,"target_va":5371656624,"va":5376292630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140368e00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4009659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e845d1c2ff","size":5,"soft_fail":false,"target_va":5372284416,"va":5376294070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3542649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e887f1c9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376292356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b3c0
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-67418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e8a6f8feff","size":5,"soft_fail":false,"target_va":5376226240,"va":5376293653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c1c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-62846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e8820affff","size":5,"soft_fail":false,"target_va":5376229824,"va":5376292665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411796b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10739482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e81adfa300","size":5,"soft_fail":false,"target_va":5387032240,"va":5376292753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27301043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e8b394a001","size":5,"soft_fail":false,"target_va":5403593360,"va":5376292312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26627504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e8b04d9601","size":5,"soft_fail":false,"target_va":5402919776,"va":5376292267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4984874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e8d6efb3ff","size":5,"soft_fail":false,"target_va":5371308048,"va":5376292917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26685692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e8fc309701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376293183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26932858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e87af69a01","size":5,"soft_fail":false,"target_va":5403226032,"va":5376293169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26685919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e8df319701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376293228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26686235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e81b339701","size":5,"soft_fail":false,"target_va":5402979344,"va":5376293104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t26676757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e8150e9701","size":5,"soft_fail":false,"target_va":5402968992,"va":5376292230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t26678381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376291744,"raw_hex":"e86d149701","size":5,"soft_fail":false,"target_va":5402970800,"va":5376292414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":570,"record_offset":25404,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":22347,"source_line":1520,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":570,"record_offset":25720,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":27512,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":27304,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":26236,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":570,"record_offset":27360,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":25608,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":8949,"source_line":1221,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":25948,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":25908,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":27128,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":26064,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBufferPool::recycle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":570,"record_offset":29056,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":30933,"source_line":268,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::DescriptorPoolDiscardPile::reclaim_descriptor_pools
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":570,"record_offset":27540,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":30931,"source_line":33,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":570,"record_offset":28856,"parent_record_offset":25204,"ancestor_record_offset":25204,"inlinee":11108,"source_line":71,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":26772,"parent_record_offset":26748,"ancestor_record_offset":25204,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":26748,"parent_record_offset":26656,"ancestor_record_offset":25204,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":570,"record_offset":25848,"parent_record_offset":25720,"ancestor_record_offset":25204,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":570,"record_offset":25428,"parent_record_offset":25404,"ancestor_record_offset":25204,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":570,"record_offset":25460,"parent_record_offset":25428,"ancestor_record_offset":25204,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":570,"record_offset":25516,"parent_record_offset":25428,"ancestor_record_offset":25204,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":570,"record_offset":25540,"parent_record_offset":25516,"ancestor_record_offset":25204,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":570,"record_offset":25564,"parent_record_offset":25540,"ancestor_record_offset":25204,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":26416,"parent_record_offset":26388,"ancestor_record_offset":25204,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":26684,"parent_record_offset":26656,"ancestor_record_offset":25204,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":26388,"parent_record_offset":26356,"ancestor_record_offset":25204,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":25484,"parent_record_offset":25460,"ancestor_record_offset":25204,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":26356,"parent_record_offset":26332,"ancestor_record_offset":25204,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":26656,"parent_record_offset":26632,"ancestor_record_offset":25204,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":28728,"parent_record_offset":28692,"ancestor_record_offset":25204,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":28692,"parent_record_offset":28668,"ancestor_record_offset":25204,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":26296,"parent_record_offset":26264,"ancestor_record_offset":25204,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":27956,"parent_record_offset":27796,"ancestor_record_offset":25204,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":27796,"parent_record_offset":27768,"ancestor_record_offset":25204,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":26468,"parent_record_offset":26296,"ancestor_record_offset":25204,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":29228,"parent_record_offset":29056,"ancestor_record_offset":25204,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":26332,"parent_record_offset":26296,"ancestor_record_offset":25204,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":29096,"parent_record_offset":29056,"ancestor_record_offset":25204,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":27740,"parent_record_offset":27540,"ancestor_record_offset":25204,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":28088,"parent_record_offset":28064,"ancestor_record_offset":25204,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":570,"record_offset":26560,"parent_record_offset":26536,"ancestor_record_offset":25204,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":570,"record_offset":26608,"parent_record_offset":26584,"ancestor_record_offset":25204,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":570,"record_offset":28144,"parent_record_offset":28088,"ancestor_record_offset":25204,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":570,"record_offset":27928,"parent_record_offset":27796,"ancestor_record_offset":25204,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":570,"record_offset":28252,"parent_record_offset":28228,"ancestor_record_offset":25204,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":570,"record_offset":27768,"parent_record_offset":27740,"ancestor_record_offset":25204,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":570,"record_offset":26264,"parent_record_offset":26236,"ancestor_record_offset":25204,"inlinee":4434,"source_line":31,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$62::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":25656,"parent_record_offset":25632,"ancestor_record_offset":25204,"inlinee":4212,"source_line":1549,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$63::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":25680,"parent_record_offset":25656,"ancestor_record_offset":25204,"inlinee":4214,"source_line":1565,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":28036,"parent_record_offset":28012,"ancestor_record_offset":25204,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":28012,"parent_record_offset":27796,"ancestor_record_offset":25204,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":28064,"parent_record_offset":28036,"ancestor_record_offset":25204,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":25972,"parent_record_offset":25948,"ancestor_record_offset":25204,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":26712,"parent_record_offset":26684,"ancestor_record_offset":25204,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":25800,"parent_record_offset":25776,"ancestor_record_offset":25204,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":26536,"parent_record_offset":26264,"ancestor_record_offset":25204,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":25872,"parent_record_offset":25848,"ancestor_record_offset":25204,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":27424,"parent_record_offset":27360,"ancestor_record_offset":25204,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":28760,"parent_record_offset":28728,"ancestor_record_offset":25204,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":27452,"parent_record_offset":27360,"ancestor_record_offset":25204,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":27392,"parent_record_offset":27360,"ancestor_record_offset":25204,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":26000,"parent_record_offset":25972,"ancestor_record_offset":25204,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":26028,"parent_record_offset":25972,"ancestor_record_offset":25204,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":26172,"parent_record_offset":26088,"ancestor_record_offset":25204,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":26196,"parent_record_offset":26172,"ancestor_record_offset":25204,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":29136,"parent_record_offset":29096,"ancestor_record_offset":25204,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":29176,"parent_record_offset":29136,"ancestor_record_offset":25204,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":27208,"parent_record_offset":27176,"ancestor_record_offset":25204,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":26116,"parent_record_offset":26088,"ancestor_record_offset":25204,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":27232,"parent_record_offset":27208,"ancestor_record_offset":25204,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":26140,"parent_record_offset":26116,"ancestor_record_offset":25204,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":28788,"parent_record_offset":28728,"ancestor_record_offset":25204,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":28820,"parent_record_offset":28692,"ancestor_record_offset":25204,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":27568,"parent_record_offset":27540,"ancestor_record_offset":25204,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":27152,"parent_record_offset":27128,"ancestor_record_offset":25204,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":26088,"parent_record_offset":26064,"ancestor_record_offset":25204,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":27176,"parent_record_offset":27152,"ancestor_record_offset":25204,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$17::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":570,"record_offset":25632,"parent_record_offset":25608,"ancestor_record_offset":25204,"inlinee":30494,"source_line":77,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111A8
// 首条位置: {"module":1592,"inlinee":70056}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$4::create::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1134D
// 首条位置: {"module":1624,"inlinee":70477}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$4::create::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78DA
// 首条位置: {"module":570,"inlinee":30938}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$4::create::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 411
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18187
// 首条位置: {"module":3007,"inlinee":98695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":3007,"record_offset":17060,"parent_record_offset":16816,"ancestor_record_offset":16816,"inlinee":12811,"source_line":151,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::get_device_queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":3007,"record_offset":16932,"parent_record_offset":16816,"ancestor_record_offset":16816,"inlinee":30980,"source_line":2357,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_support
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":3007,"record_offset":16964,"parent_record_offset":16816,"ancestor_record_offset":16816,"inlinee":30981,"source_line":12,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3007,"record_offset":16892,"parent_record_offset":16816,"ancestor_record_offset":16816,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3007,"record_offset":17532,"parent_record_offset":16816,"ancestor_record_offset":16816,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::semaphore::create_raw_timeline_semaphore
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/semaphore.rs
//     evidence={"module":3007,"record_offset":17116,"parent_record_offset":16816,"ancestor_record_offset":16816,"inlinee":31032,"source_line":17,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":3007,"record_offset":17084,"parent_record_offset":17060,"ancestor_record_offset":16816,"inlinee":12812,"source_line":542,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::create_semaphore
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":3007,"record_offset":17384,"parent_record_offset":17116,"ancestor_record_offset":16816,"inlinee":30837,"source_line":2129,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::SemaphoreCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":17180,"parent_record_offset":17116,"ancestor_record_offset":16816,"inlinee":31036,"source_line":7639,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::SemaphoreTypeCreateInfo::semaphore_type
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":3007,"record_offset":17152,"parent_record_offset":17116,"ancestor_record_offset":16816,"inlinee":31034,"source_line":21895,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3007,"record_offset":17412,"parent_record_offset":17384,"ancestor_record_offset":16816,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3007,"record_offset":17000,"parent_record_offset":16964,"ancestor_record_offset":16816,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":3007,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":16816,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":3007,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":16816,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":3007,"record_offset":17232,"parent_record_offset":17208,"ancestor_record_offset":16816,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":3007,"record_offset":17256,"parent_record_offset":17232,"ancestor_record_offset":16816,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":3007,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":16816,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3007,"record_offset":17208,"parent_record_offset":17180,"ancestor_record_offset":16816,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3007,"record_offset":17280,"parent_record_offset":17256,"ancestor_record_offset":16816,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3007,"record_offset":17500,"parent_record_offset":17116,"ancestor_record_offset":16816,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$4::create::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 447
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x790D
// 首条位置: {"module":570,"inlinee":30989}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::device_uuid
// 记录形式: compiled_procedure; instances=1
// 源码行: 548
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":47560,"rva":7605952,"va":5376315072}
// 编译实例 RVAs: 0x740ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::physical_device
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":47696,"parent_record_offset":47560,"ancestor_record_offset":47560,"inlinee":30995,"source_line":600,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 562
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1624,"record_offset":4496,"rva":18400320,"va":5387109440}
// 编译实例 RVAs: 0x118c440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140368e00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14825490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387109440,"raw_hex":"e8eec71dff","size":5,"soft_fail":false,"target_va":5372284416,"va":5387109901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403889a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14696363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387109440,"raw_hex":"e855c01fff","size":5,"soft_fail":false,"target_va":5372414368,"va":5387110726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14360182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387109440,"raw_hex":"e88ae124ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5387109889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10884554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387109440,"raw_hex":"e836ea59ff","size":5,"soft_fail":false,"target_va":5376226240,"va":5387110789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073a3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10821856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387109440,"raw_hex":"e820df5aff","size":5,"soft_fail":false,"target_va":5376287712,"va":5387109563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15868563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387109440,"raw_hex":"e89322f200","size":5,"soft_fail":false,"target_va":5402979152,"va":5387110584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t15868629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387109440,"raw_hex":"e8d522f200","size":5,"soft_fail":false,"target_va":5402978944,"va":5387110310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15858772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387109440,"raw_hex":"e854fcf100","size":5,"soft_fail":false,"target_va":5402968992,"va":5387110215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15860561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387109440,"raw_hex":"e85103f200","size":5,"soft_fail":false,"target_va":5402970800,"va":5387110234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::begin
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10776701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387109440,"raw_hex":"e8838f5bff","size":5,"soft_fail":false,"target_va":5376332896,"va":5387109592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5387109440,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5387109776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1624,"record_offset":4948,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":5636,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":4712,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":4892,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$3::create_buffer::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":1624,"record_offset":4976,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":70485,"source_line":292,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":4496,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":5908,"parent_record_offset":5808,"ancestor_record_offset":4496,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":5840,"parent_record_offset":5808,"ancestor_record_offset":4496,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":5808,"parent_record_offset":5780,"ancestor_record_offset":4496,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_copy_buffer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1624,"record_offset":5604,"parent_record_offset":4976,"ancestor_record_offset":4496,"inlinee":67180,"source_line":1466,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1624,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":4496,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1624,"record_offset":5272,"parent_record_offset":5180,"ancestor_record_offset":4496,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":5872,"parent_record_offset":5840,"ancestor_record_offset":4496,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":5668,"parent_record_offset":5636,"ancestor_record_offset":4496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":4828,"parent_record_offset":4740,"ancestor_record_offset":4496,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":4496,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":5216,"parent_record_offset":5180,"ancestor_record_offset":4496,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":6740,"parent_record_offset":6704,"ancestor_record_offset":4496,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":4772,"parent_record_offset":4740,"ancestor_record_offset":4496,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":6764,"parent_record_offset":6740,"ancestor_record_offset":4496,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":4496,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":5240,"parent_record_offset":5216,"ancestor_record_offset":4496,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":6676,"parent_record_offset":6648,"ancestor_record_offset":4496,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1624,"record_offset":4740,"parent_record_offset":4712,"ancestor_record_offset":4496,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1624,"record_offset":6704,"parent_record_offset":6676,"ancestor_record_offset":4496,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1624,"record_offset":5180,"parent_record_offset":5092,"ancestor_record_offset":4496,"inlinee":30860,"source_line":54,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$4::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1624,"record_offset":5700,"parent_record_offset":5668,"ancestor_record_offset":4496,"inlinee":30578,"source_line":125,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::ManagedRef::get_tracked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":5028,"parent_record_offset":4976,"ancestor_record_offset":4496,"inlinee":30923,"source_line":327,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":5064,"parent_record_offset":5028,"ancestor_record_offset":4496,"inlinee":7654,"source_line":250,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":5152,"parent_record_offset":5092,"ancestor_record_offset":4496,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$4::track
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":5092,"parent_record_offset":5064,"ancestor_record_offset":4496,"inlinee":30867,"source_line":94,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1624,"record_offset":5124,"parent_record_offset":5092,"ancestor_record_offset":4496,"inlinee":30448,"source_line":43,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 562
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1624,"record_offset":8244,"rva":18402560,"va":5387111680}
// 编译实例 RVAs: 0x118cd00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140368e00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14827532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387111680,"raw_hex":"e8f4bf1dff","size":5,"soft_fail":false,"target_va":5372284416,"va":5387111943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403889a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14697892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387111680,"raw_hex":"e85cba1fff","size":5,"soft_fail":false,"target_va":5372414368,"va":5387112255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14362224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387111680,"raw_hex":"e890d924ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5387111931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10886067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387111680,"raw_hex":"e84de459ff","size":5,"soft_fail":false,"target_va":5376226240,"va":5387112302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073a3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10824075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387111680,"raw_hex":"e875d65aff","size":5,"soft_fail":false,"target_va":5376287712,"va":5387111782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15867012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387111680,"raw_hex":"e8841cf200","size":5,"soft_fail":false,"target_va":5402979152,"va":5387112135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15856970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387111680,"raw_hex":"e84af5f100","size":5,"soft_fail":false,"target_va":5402968992,"va":5387112017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15858686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387111680,"raw_hex":"e8fefbf100","size":5,"soft_fail":false,"target_va":5402970800,"va":5387112109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387111680,"raw_hex":"e8a1f1ffff","size":5,"soft_fail":false,"target_va":5387108144,"va":5387111818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::begin
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10778911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387111680,"raw_hex":"e8e1865bff","size":5,"soft_fail":false,"target_va":5376332896,"va":5387111802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1624,"record_offset":8704,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":8736,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":8460,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":8640,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":8244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":9008,"parent_record_offset":8908,"ancestor_record_offset":8244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":8940,"parent_record_offset":8908,"ancestor_record_offset":8244,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":8908,"parent_record_offset":8880,"ancestor_record_offset":8244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1624,"record_offset":8668,"parent_record_offset":8640,"ancestor_record_offset":8244,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":8972,"parent_record_offset":8940,"ancestor_record_offset":8244,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":8768,"parent_record_offset":8736,"ancestor_record_offset":8244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":8576,"parent_record_offset":8488,"ancestor_record_offset":8244,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":8600,"parent_record_offset":8576,"ancestor_record_offset":8244,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":9176,"parent_record_offset":9144,"ancestor_record_offset":8244,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":8520,"parent_record_offset":8488,"ancestor_record_offset":8244,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":9200,"parent_record_offset":9176,"ancestor_record_offset":8244,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":8544,"parent_record_offset":8520,"ancestor_record_offset":8244,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":9116,"parent_record_offset":9088,"ancestor_record_offset":8244,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1624,"record_offset":8488,"parent_record_offset":8460,"ancestor_record_offset":8244,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1624,"record_offset":9144,"parent_record_offset":9116,"ancestor_record_offset":8244,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$4::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1624,"record_offset":8800,"parent_record_offset":8768,"ancestor_record_offset":8244,"inlinee":30578,"source_line":125,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 562
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1624,"record_offset":7028,"rva":18401808,"va":5387110928}
// 编译实例 RVAs: 0x118ca10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140368e00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14826780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387110928,"raw_hex":"e8e4c21dff","size":5,"soft_fail":false,"target_va":5372284416,"va":5387111191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403889a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14697140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387110928,"raw_hex":"e84cbd1fff","size":5,"soft_fail":false,"target_va":5372414368,"va":5387111503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14361472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387110928,"raw_hex":"e880dc24ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5387111179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10885315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387110928,"raw_hex":"e83de759ff","size":5,"soft_fail":false,"target_va":5376226240,"va":5387111550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073a3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10823323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387110928,"raw_hex":"e865d95aff","size":5,"soft_fail":false,"target_va":5376287712,"va":5387111030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15867764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387110928,"raw_hex":"e8741ff200","size":5,"soft_fail":false,"target_va":5402979152,"va":5387111383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15857722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387110928,"raw_hex":"e83af8f100","size":5,"soft_fail":false,"target_va":5402968992,"va":5387111265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15859438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387110928,"raw_hex":"e8eefef100","size":5,"soft_fail":false,"target_va":5402970800,"va":5387111357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::begin
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10778159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387110928,"raw_hex":"e8d1895bff","size":5,"soft_fail":false,"target_va":5376332896,"va":5387111050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::impl$9::update_texture_impl::closure$2
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387110928,"raw_hex":"e8d1180000","size":5,"soft_fail":false,"target_va":5387117424,"va":5387111066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1624,"record_offset":7488,"parent_record_offset":7028,"ancestor_record_offset":7028,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":7520,"parent_record_offset":7028,"ancestor_record_offset":7028,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":7244,"parent_record_offset":7028,"ancestor_record_offset":7028,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":7424,"parent_record_offset":7028,"ancestor_record_offset":7028,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":7028,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":7792,"parent_record_offset":7692,"ancestor_record_offset":7028,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":7724,"parent_record_offset":7692,"ancestor_record_offset":7028,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":7692,"parent_record_offset":7664,"ancestor_record_offset":7028,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1624,"record_offset":7452,"parent_record_offset":7424,"ancestor_record_offset":7028,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":7756,"parent_record_offset":7724,"ancestor_record_offset":7028,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":7552,"parent_record_offset":7520,"ancestor_record_offset":7028,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":7360,"parent_record_offset":7272,"ancestor_record_offset":7028,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":7028,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":7960,"parent_record_offset":7928,"ancestor_record_offset":7028,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":7304,"parent_record_offset":7272,"ancestor_record_offset":7028,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":7984,"parent_record_offset":7960,"ancestor_record_offset":7028,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":7028,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1624,"record_offset":7900,"parent_record_offset":7872,"ancestor_record_offset":7028,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1624,"record_offset":7272,"parent_record_offset":7244,"ancestor_record_offset":7028,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1624,"record_offset":7928,"parent_record_offset":7900,"ancestor_record_offset":7028,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$4::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1624,"record_offset":7584,"parent_record_offset":7552,"ancestor_record_offset":7028,"inlinee":30578,"source_line":125,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info
// 记录形式: compiled_procedure; instances=1
// 源码行: 588
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":47724,"rva":7605984,"va":5376315104}
// 编译实例 RVAs: 0x740ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140368e00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4030974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e8027ec2ff","size":5,"soft_fail":false,"target_va":5372284416,"va":5376315385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403889a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3901920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e82076c4ff","size":5,"soft_fail":false,"target_va":5372414368,"va":5376316283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3565666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e89e97c9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376315373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b3c0
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-89933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e8b3a0feff","size":5,"soft_fail":false,"target_va":5376226240,"va":5376316168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-85079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e8a9b3feff","size":5,"soft_fail":false,"target_va":5376231152,"va":5376316226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27277501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e8bd38a001","size":5,"soft_fail":false,"target_va":5403593360,"va":5376315854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5007705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e8a796b3ff","size":5,"soft_fail":false,"target_va":5371308048,"va":5376315748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26663012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e864d89601","size":5,"soft_fail":false,"target_va":5402979152,"va":5376316135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26662888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e8e8d79601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376316051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26664014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e84edc9601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376316109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26653050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e87ab19601","size":5,"soft_fail":false,"target_va":5402968992,"va":5376315937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t26655487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376315104,"raw_hex":"e8ffba9601","size":5,"soft_fail":false,"target_va":5402970800,"va":5376315308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5376315104,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5376315520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":48072,"parent_record_offset":47724,"ancestor_record_offset":47724,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":48188,"parent_record_offset":47724,"ancestor_record_offset":47724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":47892,"parent_record_offset":47724,"ancestor_record_offset":47724,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBuffer::end
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":570,"record_offset":48680,"parent_record_offset":47724,"ancestor_record_offset":47724,"inlinee":30996,"source_line":221,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmitInfo::command_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":570,"record_offset":48932,"parent_record_offset":47724,"ancestor_record_offset":47724,"inlinee":30497,"source_line":25,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmitInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":570,"record_offset":48876,"parent_record_offset":47724,"ancestor_record_offset":47724,"inlinee":30496,"source_line":21,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":48628,"parent_record_offset":48604,"ancestor_record_offset":47724,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":48604,"parent_record_offset":48504,"ancestor_record_offset":47724,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":48536,"parent_record_offset":48504,"ancestor_record_offset":47724,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":49044,"parent_record_offset":49012,"ancestor_record_offset":47724,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":48504,"parent_record_offset":48476,"ancestor_record_offset":47724,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":48972,"parent_record_offset":48932,"ancestor_record_offset":47724,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::end_command_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":48724,"parent_record_offset":48680,"ancestor_record_offset":47724,"inlinee":30997,"source_line":2423,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":48748,"parent_record_offset":48724,"ancestor_record_offset":47724,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":48772,"parent_record_offset":48748,"ancestor_record_offset":47724,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":48844,"parent_record_offset":48680,"ancestor_record_offset":47724,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":48096,"parent_record_offset":48072,"ancestor_record_offset":47724,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":48568,"parent_record_offset":48536,"ancestor_record_offset":47724,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":48428,"parent_record_offset":48364,"ancestor_record_offset":47724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":49012,"parent_record_offset":48972,"ancestor_record_offset":47724,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":48124,"parent_record_offset":48096,"ancestor_record_offset":47724,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":48152,"parent_record_offset":48096,"ancestor_record_offset":47724,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":48808,"parent_record_offset":48680,"ancestor_record_offset":47724,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":48008,"parent_record_offset":47920,"ancestor_record_offset":47724,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":48032,"parent_record_offset":48008,"ancestor_record_offset":47724,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":48268,"parent_record_offset":48236,"ancestor_record_offset":47724,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":47952,"parent_record_offset":47920,"ancestor_record_offset":47724,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":48292,"parent_record_offset":48268,"ancestor_record_offset":47724,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":47976,"parent_record_offset":47952,"ancestor_record_offset":47724,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":48212,"parent_record_offset":48188,"ancestor_record_offset":47724,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":47920,"parent_record_offset":47892,"ancestor_record_offset":47724,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":48236,"parent_record_offset":48212,"ancestor_record_offset":47724,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$4::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":570,"record_offset":48396,"parent_record_offset":48364,"ancestor_record_offset":47724,"inlinee":30578,"source_line":125,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::submit::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":570,"record_offset":48900,"parent_record_offset":48876,"ancestor_record_offset":47724,"inlinee":30487,"source_line":10,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::physical_device
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 600, 601
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7913
// 首条位置: {"module":570,"inlinee":30995}
// 编译实例 RVAs: 0x741430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$68::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":49608,"parent_record_offset":49468,"ancestor_record_offset":49468,"inlinee":6945,"source_line":3601,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::debug_utils
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=14
// 源码行: 604, 605
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78C6
// 首条位置: {"module":570,"inlinee":30918}
// 编译实例 RVAs: 0x741440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":10996,"parent_record_offset":10972,"ancestor_record_offset":10972,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::limits
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 608
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7838
// 首条位置: {"module":565,"inlinee":30776}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 612
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x794F
// 首条位置: {"module":570,"inlinee":31055}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":84148,"parent_record_offset":84124,"ancestor_record_offset":84124,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::rt
// 记录形式: inlinee_declaration; instances=2
// 源码行: 627
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x796F
// 首条位置: {"module":1593,"inlinee":31087}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::wait_idle
// 记录形式: compiled_procedure; instances=1
// 源码行: 631
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":49804,"rva":7607392,"va":5376316512}
// 编译实例 RVAs: 0x741460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26661260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e88cd19601","size":5,"soft_fail":false,"target_va":5402978688,"va":5376317423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-38389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e80b6affff","size":5,"soft_fail":false,"target_va":5376279232,"va":5376317616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26663109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e8c5d89601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376317014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-37156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e8dc6effff","size":5,"soft_fail":false,"target_va":5376279424,"va":5376316575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26664006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e846dc9601","size":5,"soft_fail":false,"target_va":5402980656,"va":5376316645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1036484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e8c4d00f00","size":5,"soft_fail":false,"target_va":5377353776,"va":5376317287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26646573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e82d989601","size":5,"soft_fail":false,"target_va":5402963904,"va":5376317326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26647956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e8949d9601","size":5,"soft_fail":false,"target_va":5402965344,"va":5376317383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t39729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e8319b0000","size":5,"soft_fail":false,"target_va":5376356912,"va":5376317178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t43610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e85aaa0000","size":5,"soft_fail":false,"target_va":5376360560,"va":5376316945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t39593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e8a99a0000","size":5,"soft_fail":false,"target_va":5376356144,"va":5376316546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hf5f0f90a77fe959e (.llvm.16944548595553555407)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10732470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376316512,"raw_hex":"e8b6c3a300","size":5,"soft_fail":false,"target_va":5387049440,"va":5376316965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-38184","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376316512,"raw_hex":"e9d86affff","size":5,"soft_fail":false,"target_va":5376279232,"va":5376317411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5376316512,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5376317353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::device_wait_idle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":52964,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":30998,"source_line":1578,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":50660,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":52736,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":52992,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":53196,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":570,"record_offset":52792,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":570,"record_offset":52764,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":570,"record_offset":50560,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":51704,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":49976,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":50692,"parent_record_offset":49804,"ancestor_record_offset":49804,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":52088,"parent_record_offset":52060,"ancestor_record_offset":49804,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":52140,"parent_record_offset":52116,"ancestor_record_offset":49804,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":52116,"parent_record_offset":52088,"ancestor_record_offset":49804,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":52028,"parent_record_offset":52000,"ancestor_record_offset":49804,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":52060,"parent_record_offset":52028,"ancestor_record_offset":49804,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":52232,"parent_record_offset":52060,"ancestor_record_offset":49804,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":52344,"parent_record_offset":52320,"ancestor_record_offset":49804,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":52164,"parent_record_offset":52140,"ancestor_record_offset":49804,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":570,"record_offset":50196,"parent_record_offset":50100,"ancestor_record_offset":49804,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":51924,"parent_record_offset":51896,"ancestor_record_offset":49804,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":51288,"parent_record_offset":50828,"ancestor_record_offset":49804,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":51972,"parent_record_offset":51800,"ancestor_record_offset":49804,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":570,"record_offset":50508,"parent_record_offset":50480,"ancestor_record_offset":49804,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":52296,"parent_record_offset":52000,"ancestor_record_offset":49804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":51064,"parent_record_offset":51040,"ancestor_record_offset":49804,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":51040,"parent_record_offset":51016,"ancestor_record_offset":49804,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":51248,"parent_record_offset":51216,"ancestor_record_offset":49804,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":52188,"parent_record_offset":52164,"ancestor_record_offset":49804,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":52696,"parent_record_offset":51768,"ancestor_record_offset":49804,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":52900,"parent_record_offset":52816,"ancestor_record_offset":49804,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":52924,"parent_record_offset":52900,"ancestor_record_offset":49804,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":50132,"parent_record_offset":50100,"ancestor_record_offset":49804,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":50160,"parent_record_offset":50132,"ancestor_record_offset":49804,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":51016,"parent_record_offset":50992,"ancestor_record_offset":49804,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":51344,"parent_record_offset":51320,"ancestor_record_offset":49804,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":50992,"parent_record_offset":50884,"ancestor_record_offset":49804,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":570,"record_offset":50100,"parent_record_offset":50072,"ancestor_record_offset":49804,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":570,"record_offset":50724,"parent_record_offset":50692,"ancestor_record_offset":49804,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":570,"record_offset":50856,"parent_record_offset":50828,"ancestor_record_offset":49804,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":570,"record_offset":50828,"parent_record_offset":50796,"ancestor_record_offset":49804,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":50772,"parent_record_offset":50748,"ancestor_record_offset":49804,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":50796,"parent_record_offset":50772,"ancestor_record_offset":49804,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":50748,"parent_record_offset":50724,"ancestor_record_offset":49804,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":570,"record_offset":51320,"parent_record_offset":50796,"ancestor_record_offset":49804,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":50296,"parent_record_offset":50264,"ancestor_record_offset":49804,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":50456,"parent_record_offset":50296,"ancestor_record_offset":49804,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":50264,"parent_record_offset":50240,"ancestor_record_offset":49804,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":50012,"parent_record_offset":49976,"ancestor_record_offset":49804,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":50480,"parent_record_offset":50456,"ancestor_record_offset":49804,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":50072,"parent_record_offset":50040,"ancestor_record_offset":49804,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":50040,"parent_record_offset":50012,"ancestor_record_offset":49804,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":50884,"parent_record_offset":50856,"ancestor_record_offset":49804,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":51132,"parent_record_offset":50884,"ancestor_record_offset":49804,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":51104,"parent_record_offset":50884,"ancestor_record_offset":49804,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":570,"record_offset":50936,"parent_record_offset":50884,"ancestor_record_offset":49804,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":53096,"parent_record_offset":53064,"ancestor_record_offset":49804,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":52844,"parent_record_offset":52816,"ancestor_record_offset":49804,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":50588,"parent_record_offset":50560,"ancestor_record_offset":49804,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":53120,"parent_record_offset":53096,"ancestor_record_offset":49804,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":52868,"parent_record_offset":52844,"ancestor_record_offset":49804,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":50620,"parent_record_offset":50588,"ancestor_record_offset":49804,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":570,"record_offset":53040,"parent_record_offset":53016,"ancestor_record_offset":49804,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":570,"record_offset":52816,"parent_record_offset":52792,"ancestor_record_offset":49804,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":570,"record_offset":53064,"parent_record_offset":53040,"ancestor_record_offset":49804,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":51736,"parent_record_offset":51704,"ancestor_record_offset":49804,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":570,"record_offset":51836,"parent_record_offset":51800,"ancestor_record_offset":49804,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":52000,"parent_record_offset":51800,"ancestor_record_offset":49804,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":570,"record_offset":52268,"parent_record_offset":52000,"ancestor_record_offset":49804,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":570,"record_offset":51864,"parent_record_offset":51836,"ancestor_record_offset":49804,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":570,"record_offset":51896,"parent_record_offset":51864,"ancestor_record_offset":49804,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":570,"record_offset":51800,"parent_record_offset":51768,"ancestor_record_offset":49804,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":570,"record_offset":51768,"parent_record_offset":51736,"ancestor_record_offset":49804,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 647
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":53376,"rva":7608528,"va":5376317648}
// 编译实例 RVAs: 0x7418d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26651152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317648,"raw_hex":"e810aa9601","size":5,"soft_fail":false,"target_va":5402968992,"va":5376317835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26652868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317648,"raw_hex":"e8c4b09601","size":5,"soft_fail":false,"target_va":5402970800,"va":5376317927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317648,"raw_hex":"e874fbffff","size":5,"soft_fail":false,"target_va":5376316512,"va":5376317671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::Queue::reclaim_command_buffer_resources
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-25997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317648,"raw_hex":"e8739affff","size":5,"soft_fail":false,"target_va":5376291744,"va":5376317736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-101819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317648,"raw_hex":"e84572feff","size":5,"soft_fail":false,"target_va":5376215872,"va":5376317686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-99341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376317648,"raw_hex":"e8f37bfeff","size":5,"soft_fail":false,"target_va":5376218384,"va":5376317720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t26652931","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376317648,"raw_hex":"e903b19601","size":5,"soft_fail":false,"target_va":5402970800,"va":5376317864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":53980,"parent_record_offset":53376,"ancestor_record_offset":53376,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":53544,"parent_record_offset":53376,"ancestor_record_offset":53376,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":53724,"parent_record_offset":53376,"ancestor_record_offset":53376,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::reclaim_command_buffer_resources_all_queues
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":570,"record_offset":53780,"parent_record_offset":53376,"ancestor_record_offset":53376,"inlinee":30505,"source_line":123,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":53812,"parent_record_offset":53780,"ancestor_record_offset":53376,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":53748,"parent_record_offset":53724,"ancestor_record_offset":53376,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":53940,"parent_record_offset":53916,"ancestor_record_offset":53376,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":53884,"parent_record_offset":53840,"ancestor_record_offset":53376,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":53660,"parent_record_offset":53572,"ancestor_record_offset":53376,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":53684,"parent_record_offset":53660,"ancestor_record_offset":53376,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":53916,"parent_record_offset":53840,"ancestor_record_offset":53376,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":53840,"parent_record_offset":53780,"ancestor_record_offset":53376,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":54068,"parent_record_offset":54036,"ancestor_record_offset":53376,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":53604,"parent_record_offset":53572,"ancestor_record_offset":53376,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":54092,"parent_record_offset":54068,"ancestor_record_offset":53376,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":53628,"parent_record_offset":53604,"ancestor_record_offset":53376,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":54008,"parent_record_offset":53980,"ancestor_record_offset":53376,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":53572,"parent_record_offset":53544,"ancestor_record_offset":53376,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":54036,"parent_record_offset":54008,"ancestor_record_offset":53376,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::validate_physical_device_requirements
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 658
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78D6
// 首条位置: {"module":570,"inlinee":30934}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":570,"record_offset":30624,"parent_record_offset":30592,"ancestor_record_offset":30592,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":570,"record_offset":30672,"parent_record_offset":30648,"ancestor_record_offset":30592,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":30648,"parent_record_offset":30624,"ancestor_record_offset":30592,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_global_allocator
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 684
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2985
// 首条位置: {"module":127,"inlinee":10629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":13476,"parent_record_offset":13444,"ancestor_record_offset":13444,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":13760,"parent_record_offset":13444,"ancestor_record_offset":13444,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":13504,"parent_record_offset":13444,"ancestor_record_offset":13444,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":13676,"parent_record_offset":13444,"ancestor_record_offset":13444,"inlinee":10631,"source_line":639,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::shut_down_everything::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":13732,"parent_record_offset":13444,"ancestor_record_offset":13444,"inlinee":10633,"source_line":893,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":13700,"parent_record_offset":13676,"ancestor_record_offset":13444,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":13612,"parent_record_offset":13528,"ancestor_record_offset":13444,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":13636,"parent_record_offset":13612,"ancestor_record_offset":13444,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13840,"parent_record_offset":13808,"ancestor_record_offset":13444,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13556,"parent_record_offset":13528,"ancestor_record_offset":13444,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":13444,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":13444,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":13784,"parent_record_offset":13760,"ancestor_record_offset":13444,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":127,"record_offset":13528,"parent_record_offset":13504,"ancestor_record_offset":13444,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":127,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":13444,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 684
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1332,"record_offset":6224,"rva":15313088,"va":5384022208}
// 编译实例 RVAs: 0xe9a8c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140edb280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t264523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022208,"raw_hex":"e84b090400","size":5,"soft_fail":false,"target_va":5384286848,"va":5384022320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t18946567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022208,"raw_hex":"e8071a2101","size":5,"soft_fail":false,"target_va":5402968992,"va":5384022420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t18948359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022208,"raw_hex":"e807212101","size":5,"soft_fail":false,"target_va":5402970800,"va":5384022436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_allocator_report
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7796825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022208,"raw_hex":"e8a70789ff","size":5,"soft_fail":false,"target_va":5376225488,"va":5384022308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_device_local_bytes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7796760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022208,"raw_hex":"e8e80789ff","size":5,"soft_fail":false,"target_va":5376225520,"va":5384022275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_system_bytes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7796755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022208,"raw_hex":"e8ed0789ff","size":5,"soft_fail":false,"target_va":5376225536,"va":5384022286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_total_allocated_bytes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7796830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022208,"raw_hex":"e8a20789ff","size":5,"soft_fail":false,"target_va":5376225472,"va":5384022297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":6812,"parent_record_offset":6224,"ancestor_record_offset":6224,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] country_core::meta_commander::commander_update::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/meta_commander.rs
//     evidence={"module":1332,"record_offset":6776,"parent_record_offset":6224,"ancestor_record_offset":6224,"inlinee":62144,"source_line":331,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1332,"record_offset":6540,"parent_record_offset":6224,"ancestor_record_offset":6224,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1332,"record_offset":6720,"parent_record_offset":6224,"ancestor_record_offset":6224,"inlinee":10631,"source_line":639,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":6224,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":6656,"parent_record_offset":6568,"ancestor_record_offset":6224,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":6680,"parent_record_offset":6656,"ancestor_record_offset":6224,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1332,"record_offset":6900,"parent_record_offset":6868,"ancestor_record_offset":6224,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1332,"record_offset":6600,"parent_record_offset":6568,"ancestor_record_offset":6224,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1332,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":6224,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1332,"record_offset":6624,"parent_record_offset":6600,"ancestor_record_offset":6224,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1332,"record_offset":6840,"parent_record_offset":6812,"ancestor_record_offset":6224,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1332,"record_offset":6568,"parent_record_offset":6540,"ancestor_record_offset":6224,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1332,"record_offset":6868,"parent_record_offset":6840,"ancestor_record_offset":6224,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env$0,tuple$<> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 684
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":139,"record_offset":3920,"rva":2529712,"va":5371238832}
// 编译实例 RVAs: 0x2699b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140024d80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2379307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e8d5b1dbff","size":5,"soft_fail":false,"target_va":5368860032,"va":5371239334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14019f810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-827940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e8dc5df3ff","size":5,"soft_fail":false,"target_va":5370411024,"va":5371238959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14019fa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-827488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e8a05ff3ff","size":5,"soft_fail":false,"target_va":5370411584,"va":5371239067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1510681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e8190d1700","size":5,"soft_fail":false,"target_va":5372749696,"va":5371239010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1910918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e886281d00","size":5,"soft_fail":false,"target_va":5373149840,"va":5371238917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: device.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32354367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e83fb0ed01","size":5,"soft_fail":false,"target_va":5403593360,"va":5371238988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31738934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e8364ce401","size":5,"soft_fail":false,"target_va":5402978144,"va":5371239205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t413326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e88e4e0600","size":5,"soft_fail":false,"target_va":5371652432,"va":5371239101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::visualizer::AllocatorVisualizer::render_breakdown_window
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5168789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e895de4e00","size":5,"soft_fail":false,"target_va":5376407728,"va":5371238934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::visualizer::AllocatorVisualizer::render_memory_block_visualization_windows
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5168552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e8a8dd4e00","size":5,"soft_fail":false,"target_va":5376407680,"va":5371239123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31729815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e89728e401","size":5,"soft_fail":false,"target_va":5402968992,"va":5371239172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t31731607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238832,"raw_hex":"e8972fe401","size":5,"soft_fail":false,"target_va":5402970800,"va":5371239188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":139,"record_offset":5036,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":139,"record_offset":4156,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":139,"record_offset":4336,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":10631,"source_line":639,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_debug.rs
//     evidence={"module":139,"record_offset":4392,"parent_record_offset":3920,"ancestor_record_offset":3920,"inlinee":11104,"source_line":812,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":4592,"parent_record_offset":4568,"ancestor_record_offset":3920,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":3920,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":4532,"parent_record_offset":4500,"ancestor_record_offset":3920,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":139,"record_offset":4568,"parent_record_offset":4532,"ancestor_record_offset":3920,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":139,"record_offset":4500,"parent_record_offset":4468,"ancestor_record_offset":3920,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":139,"record_offset":4888,"parent_record_offset":4852,"ancestor_record_offset":3920,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":139,"record_offset":4852,"parent_record_offset":4824,"ancestor_record_offset":3920,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":139,"record_offset":4360,"parent_record_offset":4336,"ancestor_record_offset":3920,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":139,"record_offset":4692,"parent_record_offset":4468,"ancestor_record_offset":3920,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":139,"record_offset":4920,"parent_record_offset":4888,"ancestor_record_offset":3920,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":139,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":3920,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":139,"record_offset":4272,"parent_record_offset":4184,"ancestor_record_offset":3920,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":139,"record_offset":4296,"parent_record_offset":4272,"ancestor_record_offset":3920,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":5128,"parent_record_offset":5092,"ancestor_record_offset":3920,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":4216,"parent_record_offset":4184,"ancestor_record_offset":3920,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":3920,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":3920,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":4948,"parent_record_offset":4888,"ancestor_record_offset":3920,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":139,"record_offset":4980,"parent_record_offset":4852,"ancestor_record_offset":3920,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::window::Window::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":139,"record_offset":4440,"parent_record_offset":4392,"ancestor_record_offset":3920,"inlinee":11105,"source_line":81,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] egui::containers::window::Window::show
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/containers/window.rs
//     evidence={"module":139,"record_offset":4468,"parent_record_offset":4392,"ancestor_record_offset":3920,"inlinee":11106,"source_line":365,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":139,"record_offset":5064,"parent_record_offset":5036,"ancestor_record_offset":3920,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":139,"record_offset":4184,"parent_record_offset":4156,"ancestor_record_offset":3920,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":139,"record_offset":5092,"parent_record_offset":5064,"ancestor_record_offset":3920,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
// 记录形式: compiled_procedure; instances=1
// 源码行: 692
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":54352,"rva":7608816,"va":5376317936}
// 编译实例 RVAs: 0x7419f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] rhapsody::raw::physical_device::PhysicalDevice::refresh_memory_budget
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-103542","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376317936,"raw_hex":"e98a6bfeff","size":5,"soft_fail":false,"target_va":5376214416,"va":5376317953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":54496,"parent_record_offset":54352,"ancestor_record_offset":54352,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::create_raw_device
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 699
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78E3
// 首条位置: {"module":570,"inlinee":30947}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::instance::Instance::get_physical_device_features2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/instance.rs
//     evidence={"module":570,"record_offset":37556,"parent_record_offset":37020,"ancestor_record_offset":37020,"inlinee":30956,"source_line":151,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DeviceCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":38076,"parent_record_offset":37020,"ancestor_record_offset":37020,"inlinee":30958,"source_line":1383,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PhysicalDeviceFeatures2::push_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":37148,"parent_record_offset":37020,"ancestor_record_offset":37020,"inlinee":30955,"source_line":11056,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2798::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":37092,"parent_record_offset":37020,"ancestor_record_offset":37020,"inlinee":30951,"source_line":30394,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$2906::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":37064,"parent_record_offset":37020,"ancestor_record_offset":37020,"inlinee":30949,"source_line":31432,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$646::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":37120,"parent_record_offset":37020,"ancestor_record_offset":37020,"inlinee":30953,"source_line":11031,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":38280,"parent_record_offset":37020,"ancestor_record_offset":37020,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":570,"record_offset":37296,"parent_record_offset":37272,"ancestor_record_offset":37020,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":570,"record_offset":37200,"parent_record_offset":37176,"ancestor_record_offset":37020,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":570,"record_offset":37224,"parent_record_offset":37200,"ancestor_record_offset":37020,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":570,"record_offset":37272,"parent_record_offset":37248,"ancestor_record_offset":37020,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":37176,"parent_record_offset":37148,"ancestor_record_offset":37020,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":37248,"parent_record_offset":37224,"ancestor_record_offset":37020,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::required_extensions
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 864
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78D8
// 首条位置: {"module":570,"inlinee":30936}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":30860,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":31012,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":33124,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":31096,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":31236,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":31124,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":570,"record_offset":33096,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":31668,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":9936,"source_line":274,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":32900,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":31604,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":570,"record_offset":31756,"parent_record_offset":30740,"ancestor_record_offset":30740,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":30916,"parent_record_offset":30892,"ancestor_record_offset":30740,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":30940,"parent_record_offset":30916,"ancestor_record_offset":30740,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":31548,"parent_record_offset":31520,"ancestor_record_offset":30740,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":30892,"parent_record_offset":30860,"ancestor_record_offset":30740,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":31520,"parent_record_offset":31456,"ancestor_record_offset":30740,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":31488,"parent_record_offset":31456,"ancestor_record_offset":30740,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":31320,"parent_record_offset":31288,"ancestor_record_offset":30740,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":31456,"parent_record_offset":31428,"ancestor_record_offset":30740,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":31036,"parent_record_offset":31012,"ancestor_record_offset":30740,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":31288,"parent_record_offset":31236,"ancestor_record_offset":30740,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":31060,"parent_record_offset":31036,"ancestor_record_offset":30740,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":32696,"parent_record_offset":32668,"ancestor_record_offset":30740,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":570,"record_offset":32056,"parent_record_offset":32032,"ancestor_record_offset":30740,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":570,"record_offset":32140,"parent_record_offset":32116,"ancestor_record_offset":30740,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":31696,"parent_record_offset":31668,"ancestor_record_offset":30740,"inlinee":9938,"source_line":739,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen::strlen_rt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":31720,"parent_record_offset":31696,"ancestor_record_offset":30740,"inlinee":9940,"source_line":752,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":570,"record_offset":32080,"parent_record_offset":32056,"ancestor_record_offset":30740,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":33268,"parent_record_offset":33236,"ancestor_record_offset":30740,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":570,"record_offset":32256,"parent_record_offset":32232,"ancestor_record_offset":30740,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":31380,"parent_record_offset":31236,"ancestor_record_offset":30740,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":31196,"parent_record_offset":31172,"ancestor_record_offset":30740,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":32600,"parent_record_offset":32576,"ancestor_record_offset":30740,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":32492,"parent_record_offset":32468,"ancestor_record_offset":30740,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":31636,"parent_record_offset":31604,"ancestor_record_offset":30740,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":30968,"parent_record_offset":30940,"ancestor_record_offset":30740,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":31352,"parent_record_offset":31236,"ancestor_record_offset":30740,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":570,"record_offset":32780,"parent_record_offset":32752,"ancestor_record_offset":30740,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":570,"record_offset":32808,"parent_record_offset":32780,"ancestor_record_offset":30740,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":31148,"parent_record_offset":31124,"ancestor_record_offset":30740,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":570,"record_offset":31172,"parent_record_offset":31148,"ancestor_record_offset":30740,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":570,"record_offset":32724,"parent_record_offset":32696,"ancestor_record_offset":30740,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":570,"record_offset":32668,"parent_record_offset":32640,"ancestor_record_offset":30740,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":31812,"parent_record_offset":31784,"ancestor_record_offset":30740,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":31840,"parent_record_offset":31812,"ancestor_record_offset":30740,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":32640,"parent_record_offset":32420,"ancestor_record_offset":30740,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":570,"record_offset":31876,"parent_record_offset":31840,"ancestor_record_offset":30740,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32552,"parent_record_offset":32528,"ancestor_record_offset":30740,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32528,"parent_record_offset":32420,"ancestor_record_offset":30740,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32468,"parent_record_offset":32444,"ancestor_record_offset":30740,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32388,"parent_record_offset":31952,"ancestor_record_offset":30740,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32444,"parent_record_offset":32420,"ancestor_record_offset":30740,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":31928,"parent_record_offset":31904,"ancestor_record_offset":30740,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":31904,"parent_record_offset":31840,"ancestor_record_offset":30740,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":31952,"parent_record_offset":31928,"ancestor_record_offset":30740,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":32200,"parent_record_offset":32172,"ancestor_record_offset":30740,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":32232,"parent_record_offset":32200,"ancestor_record_offset":30740,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":570,"record_offset":32172,"parent_record_offset":31952,"ancestor_record_offset":30740,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32004,"parent_record_offset":31952,"ancestor_record_offset":30740,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":570,"record_offset":32420,"parent_record_offset":31952,"ancestor_record_offset":30740,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":32032,"parent_record_offset":31952,"ancestor_record_offset":30740,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":32116,"parent_record_offset":31952,"ancestor_record_offset":30740,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":570,"record_offset":32296,"parent_record_offset":31952,"ancestor_record_offset":30740,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":570,"record_offset":31784,"parent_record_offset":31756,"ancestor_record_offset":30740,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::impl$5::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 958
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":78812,"rva":7530528,"va":5376239648}
// 编译实例 RVAs: 0x72e820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26729168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376239648,"raw_hex":"e8d0da9701","size":5,"soft_fail":false,"target_va":5402968992,"va":5376239819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26730884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376239648,"raw_hex":"e884e19701","size":5,"soft_fail":false,"target_va":5402970800,"va":5376239911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t76836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376239648,"raw_hex":"e8242c0100","size":5,"soft_fail":false,"target_va":5376316512,"va":5376239671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::Queue::reclaim_command_buffer_resources
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t52020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376239648,"raw_hex":"e834cb0000","size":5,"soft_fail":false,"target_va":5376291744,"va":5376239719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376239648,"raw_hex":"e8f5a2ffff","size":5,"soft_fail":false,"target_va":5376215872,"va":5376239686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t26730950","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376239648,"raw_hex":"e9c6e19701","size":5,"soft_fail":false,"target_va":5402970800,"va":5376239845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":79320,"parent_record_offset":78812,"ancestor_record_offset":78812,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":556,"record_offset":78940,"parent_record_offset":78812,"ancestor_record_offset":78812,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::reclaim_command_buffer_resources_all_queues
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":556,"record_offset":79120,"parent_record_offset":78812,"ancestor_record_offset":78812,"inlinee":30505,"source_line":123,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":556,"record_offset":79152,"parent_record_offset":79120,"ancestor_record_offset":78812,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":79280,"parent_record_offset":79256,"ancestor_record_offset":78812,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":556,"record_offset":79224,"parent_record_offset":79180,"ancestor_record_offset":78812,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":556,"record_offset":79056,"parent_record_offset":78968,"ancestor_record_offset":78812,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":556,"record_offset":79080,"parent_record_offset":79056,"ancestor_record_offset":78812,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":79256,"parent_record_offset":79180,"ancestor_record_offset":78812,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":79180,"parent_record_offset":79120,"ancestor_record_offset":78812,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":79408,"parent_record_offset":79376,"ancestor_record_offset":78812,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":79000,"parent_record_offset":78968,"ancestor_record_offset":78812,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":79432,"parent_record_offset":79408,"ancestor_record_offset":78812,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":79024,"parent_record_offset":79000,"ancestor_record_offset":78812,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":556,"record_offset":79348,"parent_record_offset":79320,"ancestor_record_offset":78812,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":556,"record_offset":78968,"parent_record_offset":78940,"ancestor_record_offset":78812,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":556,"record_offset":79376,"parent_record_offset":79348,"ancestor_record_offset":78812,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00556.jsonl.gz"}
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
// 完整结构及原始类型记录: device.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::device::DeviceCreateError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::device::DeviceCreateError>", "constructor": "enum2$", "arguments": ["rhapsody::raw::device::DeviceCreateError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<enum2$<rhapsody::raw::device::DeviceCreateError> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["enum2$<rhapsody::raw::device::DeviceCreateError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::device::GraphicsDevice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 63; 过程引用数: 83
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env$0,tuple$<> >", "constructor": "rhapsody::raw::device::GraphicsDevice::with_global_allocator", "arguments": ["tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >", "constructor": "rhapsody::raw::device::GraphicsDevice::queue_submit", "arguments": ["alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_texture", "arguments": ["ref$<slice2$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_texture", "arguments": ["enum2$<rhapsody::raw::texture::TextureInitialData>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<rhapsody::raw::device::GraphicsDevice,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["rhapsody::raw::device::GraphicsDevice", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::device::GraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::device::GraphicsDevice>", "constructor": "ref$", "arguments": ["rhapsody::raw::device::GraphicsDevice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>", "constructor": "tuple$", "arguments": ["ref$<rhapsody::raw::device::GraphicsDevice>", "usize", "usize"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$7,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$7", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$5,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$5", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$0,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$0", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$6,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$6", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$9,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$9", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$11,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$11", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$2,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$2", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$1,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$1", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$4,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$4", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$3,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$3", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$10,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$10", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$8,tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::render::nani::gpu_world::impl$3::gpu_world_subset_factory::closure_env$8", "tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> >", "constructor": "rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set", "arguments": ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > >", "constructor": "rhapsody::raw::device::GraphicsDevice::with_global_allocator", "arguments": ["country_core::meta_commander::commander_update::closure_env$3", "tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_raster_pipeline", "arguments": ["alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >", "constructor": "rhapsody::raw::device::GraphicsDevice::create_compute_pipeline", "arguments": ["alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0>", "constructor": "rhapsody::raw::device::GraphicsDevice::with_setup_cb", "arguments": ["rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2>", "constructor": "rhapsody::raw::device::GraphicsDevice::with_setup_cb", "arguments": ["rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0>", "constructor": "rhapsody::raw::device::GraphicsDevice::with_setup_cb", "arguments": ["rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::Fn<tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>,assoc$<Output,enum2$<country_core::render::nani::gpu_world::GpuWorldSubset> > >", "constructor": "core::ops::function::Fn", "arguments": ["tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>", "assoc$<Output,enum2$<country_core::render::nani::gpu_world::GpuWorldSubset> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::Fn<tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>,assoc$<Output,enum2$<country_core::render::nani::gpu_world::GpuWorldSubset> > > >", "constructor": "dyn$", "arguments": ["core::ops::function::Fn<tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>,assoc$<Output,enum2$<country_core::render::nani::gpu_world::GpuWorldSubset> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>,assoc$<Output,enum2$<country_core::render::nani::gpu_world::GpuWorldSubset> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::Fn<tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>,assoc$<Output,enum2$<country_core::render::nani::gpu_world::GpuWorldSubset> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>,assoc$<Output,enum2$<country_core::render::nani::gpu_world::GpuWorldSubset> > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<ref$<rhapsody::raw::device::GraphicsDevice>,usize,usize>,assoc$<Output,enum2$<country_core::render::nani::gpu_world::GpuWorldSubset> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::device::Queue
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::device::Queue,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::device::Queue", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::device::Queue,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::device::Queue,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::device::Queue>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::device::Queue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::raw::device::Queue,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["rhapsody::raw::device::Queue", "core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::Queue::with_raw<rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >", "constructor": "rhapsody::raw::device::Queue::with_raw", "arguments": ["rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0", "enum2$<core::result::Result<bool,ash::vk::enums::Result> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["rhapsody::raw::device::Queue", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::raw::device::Queue", "alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<rhapsody::raw::device::Queue,core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["rhapsody::raw::device::Queue", "core::ops::range::Range<usize>", "rhapsody::raw::device::impl$4::create::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::device::TimelineReclaim
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 25
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>", "constructor": "rhapsody::raw::device::TimelineReclaim", "arguments": ["rhapsody::raw::command_buffer::CommandBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::impl$1::drop::Dropper<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >", "constructor": "alloc::collections::vec_deque::impl$1::drop::Dropper", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::impl$1::drop::Dropper<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::impl$1::drop::Dropper<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >", "constructor": "slice2$", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<slice2$<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["slice2$<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: device.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: device.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create
// 信息 ID: a1c5a97994a4dfbbd4f46460
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::create_device @ 0x73319f -> 0x73bd10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::debug_utils
// 信息 ID: 8ba49dc54636b0234e7ed4b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x26afc2 -> 0x741440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x26c1ee -> 0x741440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0xe9e2b2 -> 0x741440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0xe9f4de -> 0x741440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x114888e -> 0x741440 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_mapped_buffer; module=570, inline_record=10972; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer; module=570, inline_record=11512; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer; module=570, inline_record=11848; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::set_buffer_debug_name; module=570, inline_record=12968; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::update_buffer_bytes; module=570, inline_record=15640; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_command_buffer; module=570, inline_record=17768; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::begin_label; module=570, inline_record=24180; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::end_label; module=570, inline_record=25092; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::device_uuid
// 信息 ID: 7b6b60a9ce03dcb9df349dda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info
// 信息 ID: 2633151e4fe8971fdcfdddb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2deb9b -> 0x740ee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::limits
// 信息 ID: 67be5bcfeeefff9053a693fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::new; module=565, inline_record=3624; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer; module=565, inline_record=5268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
// 信息 ID: 2786a10706512466a4a5689e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=570, inline_record=84124; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >,rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0> >; module=1594, inline_record=2156; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0> >; module=1594, inline_record=6684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::physical_device
// 信息 ID: b5881e9cd38ee562542b1541
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::vram::calculate_max_resolution_scale @ 0x2b10e4 -> 0x741430 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text @ 0x2b1409 -> 0x741430 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::device_uuid; module=570, inline_record=47696; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
// 信息 ID: ef8972789e38ff0bee13814a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24a6f1 -> 0x7419f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x3040ce -> 0x7419f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::rt
// 信息 ID: be8b511e6900661181e7ed82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::validate_physical_device_requirements
// 信息 ID: 9cc662bf2e623efeaf0c3007
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=30592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_idle
// 信息 ID: e22bc86d37cbbb0fd36b4b24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::finish_screenshot_capture @ 0x15ce98 -> 0x741460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2e015e -> 0x741460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2e0174 -> 0x741460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::impl$5::drop @ 0x72e837 -> 0x741460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0x7418e7 -> 0x741460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::immediate_empty_bucket @ 0x741f06 -> 0x741460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
// 信息 ID: 448722c3b37aacf039af1396
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24af13 -> 0x7418d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x302b19 -> 0x7418d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_global_allocator
// 信息 ID: 8c24501a2ed192418852c486
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::run; module=127, inline_record=13444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > >
// 信息 ID: 9d73c97f3269beb92c01359f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["gpu_allocator::allocator::AllocatorReport"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<gpu_allocator::allocator::AllocatorReport>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "u64", "u64", "enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_global_allocator -> ["country_core::meta_commander::commander_update::closure_env$3", "tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env$0,tuple$<> >
// 信息 ID: e0eb872e1536dacfd7269b2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_global_allocator -> ["tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env$0", "tuple$<>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x34eb2f -> 0x2699b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0>
// 信息 ID: ab091376d26122f03fd91bc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer @ 0x73994f -> 0x118c440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0>
// 信息 ID: 05ca8ab2d8d159210fceccfd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_buffer_bytes @ 0x73a11f -> 0x118cd00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2>
// 信息 ID: bb06c5d6d18ade72295f520c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::device::GraphicsDevice::with_setup_cb -> ["rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_texture_impl @ 0x744775 -> 0x118ca10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::allocate_command_buffer_seq
// 信息 ID: 35107944b3d4bb2af936d419
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::command_buffer::CommandBuffer::begin; module=571, inline_record=2656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::reclaim_command_buffer_resources
// 信息 ID: 93bdb6e0d7044ec655022521
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > @ 0x26a4c8 -> 0x73b3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::impl$5::drop @ 0x72e867 -> 0x73b3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::impl$5::drop @ 0x72e893 -> 0x73b3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0x741928 -> 0x73b3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources @ 0x741953 -> 0x73b3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_submit_done @ 0x7434e0 -> 0x73b3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::wait_submit_done @ 0x743516 -> 0x73b3a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::update_pending_submissions
// 信息 ID: e9ad13609984be0bbc55c6d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions; module=551, inline_record=3176; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions; module=551, inline_record=3884; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::submit::SubmissionEngine::update_queue_pending_submissions; module=551, inline_record=4500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::with_raw
// 信息 ID: ec37d122345b41c156c7bc39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >; module=139, inline_record=8992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::Queue::with_raw<rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0,enum2$<core::result::Result<bool,ash::vk::enums::Result> > >
// 信息 ID: 9b5e2a4fb2cd20f6dc5e26cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::result::Result -> ["bool", "ash::vk::enums::Result"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::result::Result<bool,ash::vk::enums::Result>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::Queue::with_raw -> ["rhapsody::raw::managed::swapchain::impl$2::present_image::closure_env$0", "enum2$<core::result::Result<bool,ash::vk::enums::Result> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::swapchain::ManagedSwapchain::present @ 0x118adb0 -> 0x118c280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::create_raw_device
// 信息 ID: 0aa5dad3b3ca055e28cbdd19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=37020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure$0
// 信息 ID: 389d86110753bef3683dd3dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::device::impl$1::reclaim_command_buffer_resources", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0 <- rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>; module=1592, inline_record=19144; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>; module=1592, inline_record=19756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$4::create::closure$0
// 信息 ID: b79701014ede93e83808fe3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::device::impl$4::create", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$4::create::closure$1
// 信息 ID: a2eda5a53e8dc6a2f013e56c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::device::impl$4::create", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=34896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$4::create::closure$2
// 信息 ID: 5820e94644461f2e36403a02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::raw::device::impl$4::create", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::device::impl$4::create::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::raw::device::Queue,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::device::impl$4::create::closure_env$2 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::device::Queue,core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::device::impl$4::create::closure_env$2 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::device::Queue,core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::device::Queue,core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::device::Queue,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::device::Queue,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,rhapsody::raw::device::impl$4::create::closure_env$2> > > >; module=3007, inline_record=16816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$4::create::closure$3
// 信息 ID: 9fafe0058dced416d213036f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "rhapsody::raw::device::impl$4::create", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=44420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$5::drop
// 信息 ID: 4a30535f36c00c82cf54f218
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x37c58f -> 0x72e820 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x6e4e4f -> 0x72e820 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x8cd83f -> 0x72e820 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x9a591f -> 0x72e820 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0xa6295f -> 0x72e820 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0xd027af -> 0x72e820 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x155e15f -> 0x72e820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::impl$6::fmt
// 信息 ID: c8b14f65504f1d327b998f9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::device::DeviceCreateError> >; module=139, inline_record=392; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::device::DeviceCreateError> >; module=1974, inline_record=412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::required_extensions
// 信息 ID: 12534366bfafcf1d41e7e8e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=30740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::required_extensions::closure$0
// 信息 ID: 3b1d074f60088276e63b6835
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::device::required_extensions", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0x73bf57 -> 0x741a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0x73bf76 -> 0x741a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
