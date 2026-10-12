// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/vulkan_config_main.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b002deeab33f7de5f6863ec98178a4d3a7027257643de7cc7e85da2734784a36
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: vulkan_config_main.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 165
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
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
// 首条位置: {"module":165,"record_offset":5012,"rva":4978064,"va":5373687184}
// 编译实例 RVAs: 0x4bf590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42926685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e85d028f02","size":5,"soft_fail":false,"target_va":5416616112,"va":5373689422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42926781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8bd028f02","size":5,"soft_fail":false,"target_va":5416616304,"va":5373689518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2598530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e882a62700","size":5,"soft_fail":false,"target_va":5376286320,"va":5373687785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1991086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8ae611e00","size":5,"soft_fail":false,"target_va":5375678688,"va":5373687597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t1991281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e871621e00","size":5,"soft_fail":false,"target_va":5375678704,"va":5373687418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4965395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8ed3bb4ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5373687582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4459565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8d3f3bbff","size":5,"soft_fail":false,"target_va":5369231648,"va":5373691208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4426301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8c375bcff","size":5,"soft_fail":false,"target_va":5369265120,"va":5373691416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008c0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4407114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8b6c0bcff","size":5,"soft_fail":false,"target_va":5369282736,"va":5373689845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4351254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8ea9abdff","size":5,"soft_fail":false,"target_va":5369338848,"va":5373690097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a040
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4350781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8c39cbdff","size":5,"soft_fail":false,"target_va":5369339968,"va":5373690744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4333085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8e3e1bdff","size":5,"soft_fail":false,"target_va":5369357424,"va":5373690504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a28c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4315421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8e326beff","size":5,"soft_fail":false,"target_va":5369374912,"va":5373690328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a6f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4296989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8e36ebeff","size":5,"soft_fail":false,"target_va":5369393024,"va":5373690008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400afd60
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4261735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e899f8beff","size":5,"soft_fail":false,"target_va":5369429344,"va":5373691074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b1e10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4251505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e88f20bfff","size":5,"soft_fail":false,"target_va":5369437712,"va":5373689212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b3700
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4246493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e82334bfff","size":5,"soft_fail":false,"target_va":5369444096,"va":5373690584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b6390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4233726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e80266bfff","size":5,"soft_fail":false,"target_va":5369455504,"va":5373689225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fa8e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1856009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8f7ade3ff","size":5,"soft_fail":false,"target_va":5371832544,"va":5373688548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fbfa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1850229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e88bc4e3ff","size":5,"soft_fail":false,"target_va":5371838368,"va":5373688592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c2980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8c72f0000","size":5,"soft_fail":false,"target_va":5373700480,"va":5373688244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43369240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e818c39502","size":5,"soft_fail":false,"target_va":5417057056,"va":5373687811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42931225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e819148f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5373689539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42931109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8a5138f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5373689680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10361836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8ec1b9e00","size":5,"soft_fail":false,"target_va":5384051040,"va":5373689199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10935208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8a8dba600","size":5,"soft_fail":false,"target_va":5384624080,"va":5373688867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<core::option::Option<tuple$<rhapsody::raw::init::GraphicsLayerAllowlist,tg_gpu_selector::Workarounds> > >,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e840050000","size":5,"soft_fail":false,"target_va":5373692192,"va":5373690843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::raw::init::GraphicsInitError> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8f3080000","size":5,"soft_fail":false,"target_va":5373692544,"va":5373690248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t42932219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8fb178f02","size":5,"soft_fail":false,"target_va":5416621632,"va":5373689408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::GraphicsInstanceLoader::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9758712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8f8e79400","size":5,"soft_fail":false,"target_va":5383446224,"va":5373687507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::GraphicsInstanceLoader::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9758709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8f5e79400","size":5,"soft_fail":false,"target_va":5383446208,"va":5373687494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::LoadedGraphicsInstance::implicit_layers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t9764174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e84efd9400","size":5,"soft_fail":false,"target_va":5383452672,"va":5373688493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42930258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e852108f02","size":5,"soft_fail":false,"target_va":5416619696,"va":5373689433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::run_vulkan_configuration_gui
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2125237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8b56d2000","size":5,"soft_fail":false,"target_va":5375814064,"va":5373688822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::app_config::write_game_config_to_disk
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-282766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e872affbff","size":5,"soft_fail":false,"target_va":5373406368,"va":5373689129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-752181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373687184,"raw_hex":"e8cb85f4ff","size":5,"soft_fail":false,"target_va":5372935104,"va":5373687280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373687184,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5373687388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":165,"record_offset":9172,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":165,"record_offset":9052,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":165,"record_offset":9456,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":165,"record_offset":8984,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":165,"record_offset":6036,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::impl$24::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/init.rs
//     evidence={"module":165,"record_offset":9136,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":11527,"source_line":93,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":165,"record_offset":5196,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":13712,"source_line":200,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":165,"record_offset":7644,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":13624,"source_line":305,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":165,"record_offset":7700,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":13626,"source_line":508,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":165,"record_offset":6180,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":13604,"source_line":205,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":165,"record_offset":6780,"parent_record_offset":5012,"ancestor_record_offset":5012,"inlinee":13609,"source_line":307,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":165,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":5012,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":165,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":5012,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":165,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":5012,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":165,"record_offset":5700,"parent_record_offset":5676,"ancestor_record_offset":5012,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":165,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":5012,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":165,"record_offset":5724,"parent_record_offset":5700,"ancestor_record_offset":5012,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":165,"record_offset":6564,"parent_record_offset":6536,"ancestor_record_offset":5012,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":165,"record_offset":5652,"parent_record_offset":5520,"ancestor_record_offset":5012,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":165,"record_offset":9260,"parent_record_offset":9212,"ancestor_record_offset":5012,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":165,"record_offset":9212,"parent_record_offset":9172,"ancestor_record_offset":5012,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":165,"record_offset":5520,"parent_record_offset":5492,"ancestor_record_offset":5012,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":165,"record_offset":6476,"parent_record_offset":6448,"ancestor_record_offset":5012,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":165,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":5012,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":165,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":5012,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":165,"record_offset":6536,"parent_record_offset":6504,"ancestor_record_offset":5012,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":165,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":5012,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":165,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":5012,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":165,"record_offset":6356,"parent_record_offset":6328,"ancestor_record_offset":5012,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":165,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":5012,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":165,"record_offset":6412,"parent_record_offset":6384,"ancestor_record_offset":5012,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":165,"record_offset":6328,"parent_record_offset":6300,"ancestor_record_offset":5012,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":165,"record_offset":6300,"parent_record_offset":6264,"ancestor_record_offset":5012,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":165,"record_offset":6852,"parent_record_offset":6824,"ancestor_record_offset":5012,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":165,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":5012,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":165,"record_offset":7940,"parent_record_offset":7896,"ancestor_record_offset":5012,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":165,"record_offset":10384,"parent_record_offset":10356,"ancestor_record_offset":5012,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":165,"record_offset":6448,"parent_record_offset":6412,"ancestor_record_offset":5012,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":165,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":5012,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":165,"record_offset":9560,"parent_record_offset":9536,"ancestor_record_offset":5012,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":165,"record_offset":5552,"parent_record_offset":5520,"ancestor_record_offset":5012,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":165,"record_offset":6824,"parent_record_offset":6780,"ancestor_record_offset":5012,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":165,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":5012,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":165,"record_offset":5616,"parent_record_offset":5580,"ancestor_record_offset":5012,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":165,"record_offset":5580,"parent_record_offset":5552,"ancestor_record_offset":5012,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":165,"record_offset":10060,"parent_record_offset":10032,"ancestor_record_offset":5012,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":165,"record_offset":6692,"parent_record_offset":6412,"ancestor_record_offset":5012,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":165,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":5012,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":165,"record_offset":5400,"parent_record_offset":5196,"ancestor_record_offset":5012,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":165,"record_offset":7988,"parent_record_offset":7964,"ancestor_record_offset":5012,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":165,"record_offset":8480,"parent_record_offset":7896,"ancestor_record_offset":5012,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":165,"record_offset":5368,"parent_record_offset":5284,"ancestor_record_offset":5012,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":8400,"parent_record_offset":8372,"ancestor_record_offset":5012,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":8780,"parent_record_offset":8592,"ancestor_record_offset":5012,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":5012,"inlinee":4686,"source_line":788,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":8092,"parent_record_offset":8060,"ancestor_record_offset":5012,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":8872,"parent_record_offset":8840,"ancestor_record_offset":5012,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":10420,"parent_record_offset":10384,"ancestor_record_offset":5012,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":8304,"parent_record_offset":8272,"ancestor_record_offset":5012,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":8116,"parent_record_offset":8092,"ancestor_record_offset":5012,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":8328,"parent_record_offset":8304,"ancestor_record_offset":5012,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":5012,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":5012,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":5012,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":165,"record_offset":10476,"parent_record_offset":10384,"ancestor_record_offset":5012,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":165,"record_offset":8272,"parent_record_offset":8244,"ancestor_record_offset":5012,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":165,"record_offset":8244,"parent_record_offset":8216,"ancestor_record_offset":5012,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":165,"record_offset":8592,"parent_record_offset":8552,"ancestor_record_offset":5012,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":165,"record_offset":8372,"parent_record_offset":8180,"ancestor_record_offset":5012,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":165,"record_offset":8180,"parent_record_offset":8152,"ancestor_record_offset":5012,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":165,"record_offset":8024,"parent_record_offset":7896,"ancestor_record_offset":5012,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":165,"record_offset":8152,"parent_record_offset":8024,"ancestor_record_offset":5012,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":165,"record_offset":8552,"parent_record_offset":8520,"ancestor_record_offset":5012,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":165,"record_offset":8060,"parent_record_offset":8024,"ancestor_record_offset":5012,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":165,"record_offset":8840,"parent_record_offset":8552,"ancestor_record_offset":5012,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":165,"record_offset":8216,"parent_record_offset":8180,"ancestor_record_offset":5012,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":165,"record_offset":5284,"parent_record_offset":5196,"ancestor_record_offset":5012,"inlinee":13716,"source_line":120,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":165,"record_offset":5228,"parent_record_offset":5196,"ancestor_record_offset":5012,"inlinee":13714,"source_line":69,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":165,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":5012,"inlinee":13715,"source_line":80,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoop::window_target
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":165,"record_offset":7668,"parent_record_offset":7644,"ancestor_record_offset":5012,"inlinee":13625,"source_line":210,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":165,"record_offset":8940,"parent_record_offset":7896,"ancestor_record_offset":5012,"inlinee":12895,"source_line":348,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":165,"record_offset":7820,"parent_record_offset":7700,"ancestor_record_offset":5012,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":165,"record_offset":7764,"parent_record_offset":7700,"ancestor_record_offset":5012,"inlinee":13627,"source_line":100,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":165,"record_offset":7896,"parent_record_offset":7860,"ancestor_record_offset":5012,"inlinee":12814,"source_line":164,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$31::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":165,"record_offset":6216,"parent_record_offset":6180,"ancestor_record_offset":5012,"inlinee":13606,"source_line":116,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":165,"record_offset":6264,"parent_record_offset":6216,"ancestor_record_offset":5012,"inlinee":13608,"source_line":162,"archive":"_pdb/C/modules/00165.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":165,"record_offset":7860,"parent_record_offset":7820,"ancestor_record_offset":5012,"inlinee":13629,"source_line":514,"archive":"_pdb/C/modules/00165.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::vulkan_config_main::run_vulkan_configuration_app
// 信息 ID: d3decdfc741e1db27a0b235d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x6437c4 -> 0x4bf590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
