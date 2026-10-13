// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/vulkan_config_main.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f9b52487257f95597b73fd4d15bd872227250c1156f95449a346556bf85c136b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: vulkan_config_main.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 176
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_gpu_selector; // namespace candidate
//   package identities: [{"name":"tg-gpu-selector","source":"local_or_path","version":"0.1.0"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use winit; // namespace candidate
//   package identities: [{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::vulkan_config_main::run_vulkan_configuration_app
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":176,"record_offset":62972,"rva":4093104,"va":5372802224}
// 编译实例 RVAs: 0x3e74b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2817980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8bcff2a00","size":5,"soft_fail":false,"target_va":5375621104,"va":5372803119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2817683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e893fe2a00","size":5,"soft_fail":false,"target_va":5375620560,"va":5372802872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400430c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3821821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e803afc5ff","size":5,"soft_fail":false,"target_va":5368983744,"va":5372805560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b6640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1250274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e81eececff","size":5,"soft_fail":false,"target_va":5371553344,"va":5372803613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b7890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1245633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e83ffeecff","size":5,"soft_fail":false,"target_va":5371558032,"va":5372803660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1240
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2002435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8038e1e00","size":5,"soft_fail":false,"target_va":5374808640,"va":5372806200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2007731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8b3a21e00","size":5,"soft_fail":false,"target_va":5374814320,"va":5372806584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dc880
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2050403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e863491f00","size":5,"soft_fail":false,"target_va":5374855296,"va":5372804888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2095578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8daf91f00","size":5,"soft_fail":false,"target_va":5374900736,"va":5372805153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2095676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e83cfa1f00","size":5,"soft_fail":false,"target_va":5374901408,"va":5372805727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ebc10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2112163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8a33a2000","size":5,"soft_fail":false,"target_va":5374917648,"va":5372805480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405eeac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2124291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8036a2000","size":5,"soft_fail":false,"target_va":5374929600,"va":5372805304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f2450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2139267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e883a42000","size":5,"soft_fail":false,"target_va":5374944336,"va":5372805064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f97d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2169703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8671b2100","size":5,"soft_fail":false,"target_va":5374973904,"va":5372804196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fdb80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2187019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e80b5f2100","size":5,"soft_fail":false,"target_va":5374991232,"va":5372804208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe270
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2186938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8ba5e2100","size":5,"soft_fail":false,"target_va":5374993008,"va":5372806065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2278807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e897c52200","size":5,"soft_fail":false,"target_va":5375081488,"va":5372802676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2278986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e84ac62200","size":5,"soft_fail":false,"target_va":5375081504,"va":5372802513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39044266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8aac45302","size":5,"soft_fail":false,"target_va":5411847168,"va":5372802897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38736160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e820114f02","size":5,"soft_fail":false,"target_va":5411540992,"va":5372804827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38736259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e883114f02","size":5,"soft_fail":false,"target_va":5411540963,"va":5372804699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9076899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8a3808a00","size":5,"soft_fail":false,"target_va":5381881088,"va":5372804184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9851106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8e2509600","size":5,"soft_fail":false,"target_va":5382655024,"va":5372803913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsInstanceLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e890ebffff","size":5,"soft_fail":false,"target_va":5372800608,"va":5372805835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::raw::init::GraphicsInitError> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8e3e9ffff","size":5,"soft_fail":false,"target_va":5372800768,"va":5372806424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38738441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8091a4f02","size":5,"soft_fail":false,"target_va":5411542880,"va":5372804434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::GraphicsInstanceLoader::implicit_layers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8334789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8c52d7f00","size":5,"soft_fail":false,"target_va":5381138352,"va":5372803558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::GraphicsInstanceLoader::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8328402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8d2147f00","size":5,"soft_fail":false,"target_va":5381130992,"va":5372802585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38734239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e89f094f02","size":5,"soft_fail":false,"target_va":5411538608,"va":5372804364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38734409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8490a4f02","size":5,"soft_fail":false,"target_va":5411538768,"va":5372804354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38734508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8ac0a4f02","size":5,"soft_fail":false,"target_va":5411538960,"va":5372804447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::run_vulkan_configuration_gui
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2397246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e83e942400","size":5,"soft_fail":false,"target_va":5375201120,"va":5372803869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::app_config::write_game_config_to_disk
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-604633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e827c6f6ff","size":5,"soft_fail":false,"target_va":5372199488,"va":5372804116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1398458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8ba561500","size":5,"soft_fail":false,"target_va":5374200800,"va":5372802337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-3577104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372802224,"raw_hex":"e8f06ac9ff","size":5,"soft_fail":false,"target_va":5369226208,"va":5372803307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5372802224,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5372802484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":176,"record_offset":67120,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":176,"record_offset":67000,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":176,"record_offset":67388,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":176,"record_offset":66932,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":176,"record_offset":63828,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":176,"record_offset":67084,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":10696,"source_line":93,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":176,"record_offset":63160,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":10937,"source_line":200,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":176,"record_offset":65612,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":5996,"source_line":305,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":176,"record_offset":65668,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":5999,"source_line":508,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":176,"record_offset":63972,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":5917,"source_line":205,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":176,"record_offset":65388,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":5956,"source_line":222,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":176,"record_offset":64576,"parent_record_offset":62972,"ancestor_record_offset":62972,"inlinee":5922,"source_line":307,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":176,"record_offset":64408,"parent_record_offset":64384,"ancestor_record_offset":62972,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":176,"record_offset":64432,"parent_record_offset":64408,"ancestor_record_offset":62972,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":176,"record_offset":63628,"parent_record_offset":63604,"ancestor_record_offset":62972,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":176,"record_offset":67200,"parent_record_offset":67160,"ancestor_record_offset":62972,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":176,"record_offset":64384,"parent_record_offset":64356,"ancestor_record_offset":62972,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":176,"record_offset":63604,"parent_record_offset":63520,"ancestor_record_offset":62972,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":176,"record_offset":67160,"parent_record_offset":67120,"ancestor_record_offset":62972,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":176,"record_offset":63520,"parent_record_offset":63492,"ancestor_record_offset":62972,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":176,"record_offset":64296,"parent_record_offset":64268,"ancestor_record_offset":62972,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":176,"record_offset":65280,"parent_record_offset":65252,"ancestor_record_offset":62972,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":176,"record_offset":65252,"parent_record_offset":65228,"ancestor_record_offset":62972,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":176,"record_offset":64356,"parent_record_offset":64324,"ancestor_record_offset":62972,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":176,"record_offset":64324,"parent_record_offset":64296,"ancestor_record_offset":62972,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":176,"record_offset":65228,"parent_record_offset":65204,"ancestor_record_offset":62972,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":176,"record_offset":64232,"parent_record_offset":64204,"ancestor_record_offset":62972,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":176,"record_offset":64204,"parent_record_offset":64176,"ancestor_record_offset":62972,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":176,"record_offset":64148,"parent_record_offset":64120,"ancestor_record_offset":62972,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":176,"record_offset":64176,"parent_record_offset":64148,"ancestor_record_offset":62972,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":176,"record_offset":64120,"parent_record_offset":64092,"ancestor_record_offset":62972,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":176,"record_offset":64092,"parent_record_offset":64056,"ancestor_record_offset":62972,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":176,"record_offset":64648,"parent_record_offset":64620,"ancestor_record_offset":62972,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":176,"record_offset":65920,"parent_record_offset":65896,"ancestor_record_offset":62972,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":176,"record_offset":65896,"parent_record_offset":65856,"ancestor_record_offset":62972,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":176,"record_offset":68204,"parent_record_offset":68176,"ancestor_record_offset":62972,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":176,"record_offset":64268,"parent_record_offset":64232,"ancestor_record_offset":62972,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":176,"record_offset":67024,"parent_record_offset":67000,"ancestor_record_offset":62972,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":176,"record_offset":67492,"parent_record_offset":67468,"ancestor_record_offset":62972,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":176,"record_offset":63548,"parent_record_offset":63520,"ancestor_record_offset":62972,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":176,"record_offset":64620,"parent_record_offset":64576,"ancestor_record_offset":62972,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":176,"record_offset":64508,"parent_record_offset":64484,"ancestor_record_offset":62972,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":176,"record_offset":67048,"parent_record_offset":67024,"ancestor_record_offset":62972,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":176,"record_offset":63572,"parent_record_offset":63548,"ancestor_record_offset":62972,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":176,"record_offset":67936,"parent_record_offset":67908,"ancestor_record_offset":62972,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":176,"record_offset":64484,"parent_record_offset":64232,"ancestor_record_offset":62972,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":176,"record_offset":63388,"parent_record_offset":63160,"ancestor_record_offset":62972,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":176,"record_offset":65944,"parent_record_offset":65920,"ancestor_record_offset":62972,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":176,"record_offset":67308,"parent_record_offset":67280,"ancestor_record_offset":62972,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":176,"record_offset":66436,"parent_record_offset":65856,"ancestor_record_offset":62972,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":176,"record_offset":63348,"parent_record_offset":63260,"ancestor_record_offset":62972,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":66356,"parent_record_offset":66328,"ancestor_record_offset":62972,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":66728,"parent_record_offset":66540,"ancestor_record_offset":62972,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":63292,"parent_record_offset":63260,"ancestor_record_offset":62972,"inlinee":10942,"source_line":683,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":66048,"parent_record_offset":66016,"ancestor_record_offset":62972,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":66820,"parent_record_offset":66788,"ancestor_record_offset":62972,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":68240,"parent_record_offset":68204,"ancestor_record_offset":62972,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":66260,"parent_record_offset":66228,"ancestor_record_offset":62972,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":66072,"parent_record_offset":66048,"ancestor_record_offset":62972,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":66284,"parent_record_offset":66260,"ancestor_record_offset":62972,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":66752,"parent_record_offset":66728,"ancestor_record_offset":62972,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":68264,"parent_record_offset":68240,"ancestor_record_offset":62972,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":63316,"parent_record_offset":63292,"ancestor_record_offset":62972,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":176,"record_offset":68296,"parent_record_offset":68204,"ancestor_record_offset":62972,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":176,"record_offset":66228,"parent_record_offset":66200,"ancestor_record_offset":62972,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":176,"record_offset":66200,"parent_record_offset":66172,"ancestor_record_offset":62972,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":176,"record_offset":66540,"parent_record_offset":66504,"ancestor_record_offset":62972,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":176,"record_offset":66328,"parent_record_offset":66136,"ancestor_record_offset":62972,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":176,"record_offset":66136,"parent_record_offset":66108,"ancestor_record_offset":62972,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":176,"record_offset":65980,"parent_record_offset":65856,"ancestor_record_offset":62972,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":176,"record_offset":66108,"parent_record_offset":65980,"ancestor_record_offset":62972,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":176,"record_offset":66504,"parent_record_offset":66476,"ancestor_record_offset":62972,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":176,"record_offset":66016,"parent_record_offset":65980,"ancestor_record_offset":62972,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":176,"record_offset":66788,"parent_record_offset":66504,"ancestor_record_offset":62972,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":176,"record_offset":66172,"parent_record_offset":66136,"ancestor_record_offset":62972,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalSize::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":176,"record_offset":65472,"parent_record_offset":65448,"ancestor_record_offset":62972,"inlinee":6845,"source_line":439,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":176,"record_offset":65508,"parent_record_offset":65472,"ancestor_record_offset":62972,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$33::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":176,"record_offset":65448,"parent_record_offset":65424,"ancestor_record_offset":62972,"inlinee":6843,"source_line":531,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":176,"record_offset":63260,"parent_record_offset":63160,"ancestor_record_offset":62972,"inlinee":10941,"source_line":120,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":176,"record_offset":63204,"parent_record_offset":63160,"ancestor_record_offset":62972,"inlinee":10939,"source_line":69,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":176,"record_offset":63228,"parent_record_offset":63204,"ancestor_record_offset":62972,"inlinee":10940,"source_line":80,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoop::window_target
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":176,"record_offset":65636,"parent_record_offset":65612,"ancestor_record_offset":62972,"inlinee":5998,"source_line":210,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":176,"record_offset":66888,"parent_record_offset":65856,"ancestor_record_offset":62972,"inlinee":6012,"source_line":348,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":176,"record_offset":65788,"parent_record_offset":65668,"ancestor_record_offset":62972,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":176,"record_offset":65732,"parent_record_offset":65668,"ancestor_record_offset":62972,"inlinee":6001,"source_line":100,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":176,"record_offset":65856,"parent_record_offset":65824,"ancestor_record_offset":62972,"inlinee":6005,"source_line":164,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$30::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":176,"record_offset":64008,"parent_record_offset":63972,"ancestor_record_offset":62972,"inlinee":5919,"source_line":116,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":176,"record_offset":64056,"parent_record_offset":64008,"ancestor_record_offset":62972,"inlinee":5921,"source_line":162,"archive":"_pdb/B/modules/00176.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":176,"record_offset":65824,"parent_record_offset":65788,"ancestor_record_offset":62972,"inlinee":6004,"source_line":514,"archive":"_pdb/B/modules/00176.jsonl.gz"}
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
// 完整结构及原始类型记录: vulkan_config_main.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: vulkan_config_main.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: vulkan_config_main.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: tiny_glade::vulkan_config_main::run_vulkan_configuration_app
// 信息 ID: ee49c3c8b57de19ffedb621e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3dd6e9 -> 0x3e74b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
