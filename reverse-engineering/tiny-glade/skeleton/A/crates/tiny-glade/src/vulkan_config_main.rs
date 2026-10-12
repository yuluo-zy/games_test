// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/vulkan_config_main.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: fcd2f24ab085c95a8972522ac780b62d46fdf55fefbe4456b20b51a89442a54b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: vulkan_config_main.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 126
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::vulkan_config_main::run_vulkan_configuration_app
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":126,"record_offset":9504,"rva":2376752,"va":5371085872}
// 编译实例 RVAs: 0x244430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2178663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8673e2100","size":5,"soft_fail":false,"target_va":5373265408,"va":5371086740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2165614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e86e0b2100","size":5,"soft_fail":false,"target_va":5373252112,"va":5371086493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d84b0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1493990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e81a34e9ff","size":5,"soft_fail":false,"target_va":5369595056,"va":5371089041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e0ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1457635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e81dc2e9ff","size":5,"soft_fail":false,"target_va":5369629600,"va":5371087230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e2ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1449726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e802e1e9ff","size":5,"soft_fail":false,"target_va":5369637552,"va":5371087273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0900
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t572848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8b0bd0800","size":5,"soft_fail":false,"target_va":5371660544,"va":5371087691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da640
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t612218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e87a570900","size":5,"soft_fail":false,"target_va":5371700800,"va":5371088577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035dea0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1149818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e87a8b1100","size":5,"soft_fail":false,"target_va":5372239520,"va":5371089697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1159466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e82ab11100","size":5,"soft_fail":false,"target_va":5372249392,"va":5371089921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1185994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8ca181200","size":5,"soft_fail":false,"target_va":5372274416,"va":5371088417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1225553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e851b31200","size":5,"soft_fail":false,"target_va":5372314208,"va":5371088650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403706c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1226147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8a3b51200","size":5,"soft_fail":false,"target_va":5372315328,"va":5371089176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140375190
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1245498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e83a011300","size":5,"soft_fail":false,"target_va":5372334480,"va":5371088977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140377f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1257338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e87a2f1300","size":5,"soft_fail":false,"target_va":5372346176,"va":5371088833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140383620
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1305277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8bdea1300","size":5,"soft_fail":false,"target_va":5372392992,"va":5371087710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1321077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e875281400","size":5,"soft_fail":false,"target_va":5372408976,"va":5371087894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387c30
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1321468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8fc291400","size":5,"soft_fail":false,"target_va":5372410928,"va":5371089455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1663394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8a2611900","size":5,"soft_fail":false,"target_va":5372749696,"va":5371086297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1663573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e855621900","size":5,"soft_fail":false,"target_va":5372749712,"va":5371086134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: vulkan_config_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32506837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8d503f001","size":5,"soft_fail":false,"target_va":5403593360,"va":5371086518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31889965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e82d9ae601","size":5,"soft_fail":false,"target_va":5402978144,"va":5371088174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6276568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8d8c55f00","size":5,"soft_fail":false,"target_va":5377364336,"va":5371087763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<core::option::Option<rhapsody::raw::init::GraphicsInstanceLayerAllowlist> >,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e850e7ffff","size":5,"soft_fail":false,"target_va":5371083168,"va":5371089483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<rhapsody::raw::init::GraphicsInitError> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8baeaffff","size":5,"soft_fail":false,"target_va":5371083328,"va":5371088769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31892087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e877a2e601","size":5,"soft_fail":false,"target_va":5402980128,"va":5371088036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::GraphicsInstanceLoader::implicit_layers
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5167252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e894d84e00","size":5,"soft_fail":false,"target_va":5376254464,"va":5371087207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::GraphicsInstanceLoader::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5162461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8ddc54e00","size":5,"soft_fail":false,"target_va":5376248672,"va":5371086206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31888272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e89093e601","size":5,"soft_fail":false,"target_va":5402975328,"va":5371087051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31888471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e85794e601","size":5,"soft_fail":false,"target_va":5402975504,"va":5371087028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31888553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8a994e601","size":5,"soft_fail":false,"target_va":5402975696,"va":5371087138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::run_vulkan_configuration_gui
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1758401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8c1d41a00","size":5,"soft_fail":false,"target_va":5372845872,"va":5371087466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::app_config::write_game_config_to_disk
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t923639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8f7170e00","size":5,"soft_fail":false,"target_va":5372011296,"va":5371087652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1107947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e8ebe71000","size":5,"soft_fail":false,"target_va":5372193920,"va":5371085968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::init
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-379253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371085872,"raw_hex":"e88b36faff","size":5,"soft_fail":false,"target_va":5370707680,"va":5371086928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5371085872,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5371086105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":126,"record_offset":13588,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":126,"record_offset":13496,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":126,"record_offset":13868,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":126,"record_offset":13428,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":126,"record_offset":10328,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":126,"record_offset":13552,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":10594,"source_line":87,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":126,"record_offset":9692,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":8182,"source_line":200,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":126,"record_offset":12136,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":8226,"source_line":305,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":126,"record_offset":12164,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":8227,"source_line":508,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":126,"record_offset":10472,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":8190,"source_line":205,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":126,"record_offset":11912,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":8196,"source_line":222,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":126,"record_offset":11100,"parent_record_offset":9504,"ancestor_record_offset":9504,"inlinee":8195,"source_line":307,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":126,"record_offset":10876,"parent_record_offset":10852,"ancestor_record_offset":9504,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":126,"record_offset":10900,"parent_record_offset":10876,"ancestor_record_offset":9504,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":126,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":9504,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":126,"record_offset":13672,"parent_record_offset":13628,"ancestor_record_offset":9504,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":126,"record_offset":10852,"parent_record_offset":10824,"ancestor_record_offset":9504,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":126,"record_offset":10104,"parent_record_offset":10020,"ancestor_record_offset":9504,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":126,"record_offset":13628,"parent_record_offset":13588,"ancestor_record_offset":9504,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":126,"record_offset":10020,"parent_record_offset":9992,"ancestor_record_offset":9504,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":126,"record_offset":11808,"parent_record_offset":11780,"ancestor_record_offset":9504,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":126,"record_offset":10824,"parent_record_offset":10796,"ancestor_record_offset":9504,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":126,"record_offset":10796,"parent_record_offset":10768,"ancestor_record_offset":9504,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":126,"record_offset":10972,"parent_record_offset":10796,"ancestor_record_offset":9504,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":126,"record_offset":11780,"parent_record_offset":11752,"ancestor_record_offset":9504,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":126,"record_offset":10732,"parent_record_offset":10704,"ancestor_record_offset":9504,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":126,"record_offset":10704,"parent_record_offset":10676,"ancestor_record_offset":9504,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":126,"record_offset":10648,"parent_record_offset":10620,"ancestor_record_offset":9504,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":126,"record_offset":10676,"parent_record_offset":10648,"ancestor_record_offset":9504,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":126,"record_offset":10620,"parent_record_offset":10592,"ancestor_record_offset":9504,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":126,"record_offset":10592,"parent_record_offset":10556,"ancestor_record_offset":9504,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":126,"record_offset":11172,"parent_record_offset":11144,"ancestor_record_offset":9504,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":126,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":9504,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":126,"record_offset":12368,"parent_record_offset":12328,"ancestor_record_offset":9504,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":126,"record_offset":14624,"parent_record_offset":14596,"ancestor_record_offset":9504,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":10768,"parent_record_offset":10732,"ancestor_record_offset":9504,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":13520,"parent_record_offset":13496,"ancestor_record_offset":9504,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":13916,"parent_record_offset":13892,"ancestor_record_offset":9504,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":126,"record_offset":10048,"parent_record_offset":10020,"ancestor_record_offset":9504,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":126,"record_offset":11144,"parent_record_offset":11100,"ancestor_record_offset":9504,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":126,"record_offset":11032,"parent_record_offset":11008,"ancestor_record_offset":9504,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":126,"record_offset":10072,"parent_record_offset":10048,"ancestor_record_offset":9504,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":126,"record_offset":14412,"parent_record_offset":14384,"ancestor_record_offset":9504,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":126,"record_offset":11008,"parent_record_offset":10732,"ancestor_record_offset":9504,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":126,"record_offset":9900,"parent_record_offset":9692,"ancestor_record_offset":9504,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":126,"record_offset":12416,"parent_record_offset":12392,"ancestor_record_offset":9504,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":126,"record_offset":10928,"parent_record_offset":10900,"ancestor_record_offset":9504,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":126,"record_offset":12572,"parent_record_offset":12512,"ancestor_record_offset":9504,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":126,"record_offset":12920,"parent_record_offset":12328,"ancestor_record_offset":9504,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":126,"record_offset":9864,"parent_record_offset":9780,"ancestor_record_offset":9504,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":126,"record_offset":13396,"parent_record_offset":12164,"ancestor_record_offset":9504,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":12840,"parent_record_offset":12812,"ancestor_record_offset":9504,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":13200,"parent_record_offset":13024,"ancestor_record_offset":9504,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":9808,"parent_record_offset":9780,"ancestor_record_offset":9504,"inlinee":8187,"source_line":669,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":12512,"parent_record_offset":12484,"ancestor_record_offset":9504,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":13288,"parent_record_offset":13260,"ancestor_record_offset":9504,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":14660,"parent_record_offset":14624,"ancestor_record_offset":9504,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":12744,"parent_record_offset":12716,"ancestor_record_offset":9504,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":12544,"parent_record_offset":12512,"ancestor_record_offset":9504,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":9504,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":13224,"parent_record_offset":13200,"ancestor_record_offset":9504,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":14684,"parent_record_offset":14660,"ancestor_record_offset":9504,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":9504,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":126,"record_offset":14716,"parent_record_offset":14624,"ancestor_record_offset":9504,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":126,"record_offset":12716,"parent_record_offset":12692,"ancestor_record_offset":9504,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":126,"record_offset":12692,"parent_record_offset":12668,"ancestor_record_offset":9504,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":126,"record_offset":12452,"parent_record_offset":12328,"ancestor_record_offset":9504,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":126,"record_offset":12608,"parent_record_offset":12452,"ancestor_record_offset":9504,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":126,"record_offset":12992,"parent_record_offset":12964,"ancestor_record_offset":9504,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":126,"record_offset":13024,"parent_record_offset":12992,"ancestor_record_offset":9504,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":126,"record_offset":12812,"parent_record_offset":12636,"ancestor_record_offset":9504,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":126,"record_offset":12636,"parent_record_offset":12608,"ancestor_record_offset":9504,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":126,"record_offset":12484,"parent_record_offset":12452,"ancestor_record_offset":9504,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":126,"record_offset":13260,"parent_record_offset":12992,"ancestor_record_offset":9504,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":126,"record_offset":12668,"parent_record_offset":12636,"ancestor_record_offset":9504,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalSize::cast
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":126,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":9504,"inlinee":8200,"source_line":439,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":126,"record_offset":12032,"parent_record_offset":11996,"ancestor_record_offset":9504,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$33::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":126,"record_offset":11972,"parent_record_offset":11948,"ancestor_record_offset":9504,"inlinee":8198,"source_line":531,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":126,"record_offset":9780,"parent_record_offset":9692,"ancestor_record_offset":9504,"inlinee":8186,"source_line":120,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":126,"record_offset":9724,"parent_record_offset":9692,"ancestor_record_offset":9504,"inlinee":8184,"source_line":69,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":126,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":9504,"inlinee":8185,"source_line":80,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":126,"record_offset":13356,"parent_record_offset":12328,"ancestor_record_offset":9504,"inlinee":8244,"source_line":344,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":126,"record_offset":12260,"parent_record_offset":12164,"ancestor_record_offset":9504,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":126,"record_offset":12224,"parent_record_offset":12164,"ancestor_record_offset":9504,"inlinee":8229,"source_line":96,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":126,"record_offset":12328,"parent_record_offset":12296,"ancestor_record_offset":9504,"inlinee":8233,"source_line":160,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$30::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":126,"record_offset":10508,"parent_record_offset":10472,"ancestor_record_offset":9504,"inlinee":8192,"source_line":116,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":126,"record_offset":10556,"parent_record_offset":10508,"ancestor_record_offset":9504,"inlinee":8194,"source_line":162,"archive":"_pdb/A/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":126,"record_offset":12296,"parent_record_offset":12260,"ancestor_record_offset":9504,"inlinee":8232,"source_line":514,"archive":"_pdb/A/modules/00126.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::vulkan_config_main::run_vulkan_configuration_app
// 信息 ID: 30c9d600c476836e1eb88f82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x39a2d9 -> 0x244430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
