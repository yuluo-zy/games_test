// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/sdl_event_loop.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 9912254934778fb6ee60f05a54ff8082eabe2f75ab3a7cccfdc7e5558857331e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sdl_event_loop.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 8, 177, 184
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use smol_str; // namespace candidate
//   package identities: [{"name":"smol_str","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use usdl; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"},{"name":"usdl","source":"local_or_path","version":"0.1.0"}]
// use usdl_sys; // namespace candidate
//   package identities: [{"name":"usdl-sys","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::convert_keyboard_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":27212,"rva":5335552,"va":5374044672}
// 编译实例 RVAs: 0x516a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] smol_str::Repr::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t556147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374044672,"raw_hex":"e8737c0800","size":5,"soft_fail":false,"target_va":5374602048,"va":5374045896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::keyboard::Keycode::name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1787712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374044672,"raw_hex":"e840471b00","size":5,"soft_fail":false,"target_va":5375833600,"va":5374045883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":27352,"parent_record_offset":27212,"ancestor_record_offset":27212,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":28212,"parent_record_offset":27212,"ancestor_record_offset":27212,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":177,"record_offset":28116,"parent_record_offset":27892,"ancestor_record_offset":27212,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":177,"record_offset":27380,"parent_record_offset":27352,"ancestor_record_offset":27212,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::SmolStr::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":177,"record_offset":28164,"parent_record_offset":28140,"ancestor_record_offset":27212,"inlinee":15309,"source_line":81,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$26::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":177,"record_offset":28140,"parent_record_offset":28116,"ancestor_record_offset":27212,"inlinee":15307,"source_line":325,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_event_loop::convert_sdl_keycode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":177,"record_offset":27892,"parent_record_offset":27864,"ancestor_record_offset":27212,"inlinee":15314,"source_line":676,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_event_loop::convert_sdl_scancode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":177,"record_offset":27404,"parent_record_offset":27380,"ancestor_record_offset":27212,"inlinee":15313,"source_line":563,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::run_event_loop
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":8436,"rva":5322048,"va":5374031168}
// 编译实例 RVAs: 0x513540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10616539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8dbfea100","size":5,"soft_fail":false,"target_va":5384648032,"va":5374031488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10575485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e87d5ea100","size":5,"soft_fail":false,"target_va":5384609408,"va":5374033918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=20
//     evidence={"assembly":"\tcallq\t1647309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8cd221900","size":5,"soft_fail":false,"target_va":5375678704,"va":5374031390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140085b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4782504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e85806b7ff","size":5,"soft_fail":false,"target_va":5369256768,"va":5374039267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140088c40
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4768673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e85f3cb7ff","size":5,"soft_fail":false,"target_va":5369269312,"va":5374037980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009f150
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4671844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e89cb6b8ff","size":5,"soft_fail":false,"target_va":5369360720,"va":5374032559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a5380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4650414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8520ab9ff","size":5,"soft_fail":false,"target_va":5369385856,"va":5374036265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400adf70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4609935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e871a8b9ff","size":5,"soft_fail":false,"target_va":5369421680,"va":5374031610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c26a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4531020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8b4dcbaff","size":5,"soft_fail":false,"target_va":5369505440,"va":5374036455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c3eb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4524873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8b7f4baff","size":5,"soft_fail":false,"target_va":5369511600,"va":5374036468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c5a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4518559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8610dbbff","size":5,"soft_fail":false,"target_va":5369518704,"va":5374037258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cadc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4492786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e80e72bbff","size":5,"soft_fail":false,"target_va":5369540032,"va":5374032813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14052e0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t108597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e835a80100","size":5,"soft_fail":false,"target_va":5374140576,"va":5374031974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca240
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1797225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8696c1b00","size":5,"soft_fail":false,"target_va":5375828544,"va":5374031314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cb770
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1797534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e89e6d1b00","size":5,"soft_fail":false,"target_va":5375833968,"va":5374036429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140957020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4471219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8b3394400","size":5,"soft_fail":false,"target_va":5378502688,"va":5374031464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f33320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10611312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e870eaa100","size":5,"soft_fail":false,"target_va":5384647456,"va":5374036139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow::h581f7130cbfacad7 (.llvm.12452156013871766364)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t657995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e84b0a0a00","size":5,"soft_fail":false,"target_va":5374690992,"va":5374032992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t9605042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8b28f9200","size":5,"soft_fail":false,"target_va":5383637632,"va":5374032585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42585634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e822ce8902","size":5,"soft_fail":false,"target_va":5416622208,"va":5374036569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tiny_glade..game_os_events..OsWindowEvent$GT$::h7879e610f95d2740 (.llvm.927496943212755707)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4336671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8e1d3bdff","size":5,"soft_fail":false,"target_va":5369701552,"va":5374038218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tiny_glade::game_os_events::GameEvent> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e81c290000","size":5,"soft_fail":false,"target_va":5374049232,"va":5374038703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CursorVisibilityState::mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4493465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e899904400","size":5,"soft_fail":false,"target_va":5378529680,"va":5374036210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CursorVisibilityState::os_cursor_visible
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4494188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e86c934400","size":5,"soft_fail":false,"target_va":5378529600,"va":5374035407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CustomCursorSsboGpuData::update_visibility
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4497311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e89f9f4400","size":5,"soft_fail":false,"target_va":5378529824,"va":5374032508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::Sender$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4333423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e891e0bdff","size":5,"soft_fail":false,"target_va":5369698176,"va":5374031594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t248442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e87aca0300","size":5,"soft_fail":false,"target_va":5374283088,"va":5374034641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t259528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8c8f50300","size":5,"soft_fail":false,"target_va":5374291584,"va":5374032051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4228813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e83379bfff","size":5,"soft_fail":false,"target_va":5369805616,"va":5374034424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4218656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8e0a0bfff","size":5,"soft_fail":false,"target_va":5369813376,"va":5374032027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1433291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e83521eaff","size":5,"soft_fail":false,"target_va":5372601248,"va":5374034534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1404888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e82890eaff","size":5,"soft_fail":false,"target_va":5372627184,"va":5374032067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10696470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e81637a300","size":5,"soft_fail":false,"target_va":5384728160,"va":5374031685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_event_loop::extract_available_monitors_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8763a0000","size":5,"soft_fail":false,"target_va":5374046512,"va":5374031541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_setup::enumerate_displays
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1156587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8155aeeff","size":5,"soft_fail":false,"target_va":5372876144,"va":5374032726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_setup::find_best_fullscreen_video_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1156310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e82a5beeff","size":5,"soft_fail":false,"target_va":5372877088,"va":5374033393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::mouse::MouseUtil::capture
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1798392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8f8701b00","size":5,"soft_fail":false,"target_va":5375830800,"va":5374032403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::mouse::MouseUtil::set_relative_mouse_mode
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1798391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8f7701b00","size":5,"soft_fail":false,"target_va":5375830784,"va":5374032388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::mouse::MouseUtil::show_cursor
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1795241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8a9641b00","size":5,"soft_fail":false,"target_va":5375830752,"va":5374035506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::mouse::MouseUtil::warp_mouse_in_window
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1795404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e84c651b00","size":5,"soft_fail":false,"target_va":5375830768,"va":5374035359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::PixelFormatEnum::byte_size_per_pixel
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1799067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e89b731b00","size":5,"soft_fail":false,"target_va":5375832512,"va":5374033440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::display_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1801292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e84c7c1b00","size":5,"soft_fail":false,"target_va":5375832640,"va":5374031343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::minimize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1801190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8e67b1b00","size":5,"soft_fail":false,"target_va":5375833440,"va":5374032245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::set_display_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1799593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8a9751b00","size":5,"soft_fail":false,"target_va":5375833136,"va":5374033538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::set_fullscreen
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1799433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e809751b00","size":5,"soft_fail":false,"target_va":5375833024,"va":5374033586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::set_grab
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1801033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8497b1b00","size":5,"soft_fail":false,"target_va":5375833456,"va":5374032418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1801112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8987b1b00","size":5,"soft_fail":false,"target_va":5375833344,"va":5374032227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1801644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e8ac7d1b00","size":5,"soft_fail":false,"target_va":5375833360,"va":5374031711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::vulkan_drawable_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1801530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374031168,"raw_hex":"e83a7d1b00","size":5,"soft_fail":false,"target_va":5375833264,"va":5374031729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":16500,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":15291,"source_line":1808,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":11484,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":5253,"source_line":2205,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":177,"record_offset":16588,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":15295,"source_line":239,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":8992,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":12059,"source_line":165,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":17404,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":15279,"source_line":1446,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":13004,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":9756,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":16472,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":9728,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":9260,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":12004,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":9200,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":9076,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":13920,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":11666,"source_line":646,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":12552,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":8644,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":177,"record_offset":13060,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":12920,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":177,"record_offset":9132,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":7498,"source_line":789,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":177,"record_offset":9048,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::sdl_setup::video_mode_from_sdl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":177,"record_offset":12420,"parent_record_offset":8436,"ancestor_record_offset":8436,"inlinee":13040,"source_line":267,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":8436,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":8900,"parent_record_offset":8876,"ancestor_record_offset":8436,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":8436,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":8852,"parent_record_offset":8796,"ancestor_record_offset":8436,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":16792,"parent_record_offset":16588,"ancestor_record_offset":8436,"inlinee":12111,"source_line":208,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":11580,"parent_record_offset":11552,"ancestor_record_offset":8436,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":11552,"parent_record_offset":11512,"ancestor_record_offset":8436,"inlinee":5255,"source_line":236,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":9016,"parent_record_offset":8992,"ancestor_record_offset":8436,"inlinee":11703,"source_line":781,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":11852,"parent_record_offset":11680,"ancestor_record_offset":8436,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::push_back_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":11512,"parent_record_offset":11484,"ancestor_record_offset":8436,"inlinee":5254,"source_line":2223,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":11680,"parent_record_offset":11512,"ancestor_record_offset":8436,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":177,"record_offset":11716,"parent_record_offset":11680,"ancestor_record_offset":8436,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::Drain::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":177,"record_offset":16528,"parent_record_offset":16500,"ancestor_record_offset":8436,"inlinee":15293,"source_line":37,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":177,"record_offset":17300,"parent_record_offset":17276,"ancestor_record_offset":8436,"inlinee":4882,"source_line":94,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":177,"record_offset":17348,"parent_record_offset":17324,"ancestor_record_offset":8436,"inlinee":4884,"source_line":115,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":11608,"parent_record_offset":11580,"ancestor_record_offset":8436,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":11876,"parent_record_offset":11852,"ancestor_record_offset":8436,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":11636,"parent_record_offset":11608,"ancestor_record_offset":8436,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":8436,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":8796,"parent_record_offset":8772,"ancestor_record_offset":8436,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":11924,"parent_record_offset":11900,"ancestor_record_offset":8436,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":11900,"parent_record_offset":11876,"ancestor_record_offset":8436,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":8772,"parent_record_offset":8748,"ancestor_record_offset":8436,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":177,"record_offset":9404,"parent_record_offset":9368,"ancestor_record_offset":8436,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":177,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":8436,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":9856,"parent_record_offset":9796,"ancestor_record_offset":8436,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":8436,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":10512,"parent_record_offset":10484,"ancestor_record_offset":8436,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":10484,"parent_record_offset":10460,"ancestor_record_offset":8436,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":11160,"parent_record_offset":11136,"ancestor_record_offset":8436,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":10596,"parent_record_offset":10568,"ancestor_record_offset":8436,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":12488,"parent_record_offset":12420,"ancestor_record_offset":8436,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":177,"record_offset":10568,"parent_record_offset":10540,"ancestor_record_offset":8436,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":177,"record_offset":10540,"parent_record_offset":10512,"ancestor_record_offset":8436,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":177,"record_offset":9976,"parent_record_offset":9796,"ancestor_record_offset":8436,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":177,"record_offset":9888,"parent_record_offset":9856,"ancestor_record_offset":8436,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":177,"record_offset":9912,"parent_record_offset":9888,"ancestor_record_offset":8436,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":177,"record_offset":9224,"parent_record_offset":9200,"ancestor_record_offset":8436,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":177,"record_offset":11304,"parent_record_offset":11272,"ancestor_record_offset":8436,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":177,"record_offset":16684,"parent_record_offset":16656,"ancestor_record_offset":8436,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":177,"record_offset":10012,"parent_record_offset":9976,"ancestor_record_offset":8436,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":13028,"parent_record_offset":13004,"ancestor_record_offset":8436,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":10060,"parent_record_offset":9796,"ancestor_record_offset":8436,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":177,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":8436,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=92
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":8676,"parent_record_offset":8644,"ancestor_record_offset":8436,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":177,"record_offset":9944,"parent_record_offset":9856,"ancestor_record_offset":8436,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":177,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":8436,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":177,"record_offset":9436,"parent_record_offset":9404,"ancestor_record_offset":8436,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":177,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":8436,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":11964,"parent_record_offset":11680,"ancestor_record_offset":8436,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":13944,"parent_record_offset":13920,"ancestor_record_offset":8436,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":13796,"parent_record_offset":13744,"ancestor_record_offset":8436,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":8436,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":12944,"parent_record_offset":12920,"ancestor_record_offset":8436,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":12968,"parent_record_offset":12944,"ancestor_record_offset":8436,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":9464,"parent_record_offset":9404,"ancestor_record_offset":8436,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":9496,"parent_record_offset":9368,"ancestor_record_offset":8436,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":177,"record_offset":17460,"parent_record_offset":17404,"ancestor_record_offset":8436,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":13108,"parent_record_offset":13084,"ancestor_record_offset":8436,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":177,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":8436,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":13132,"parent_record_offset":13108,"ancestor_record_offset":8436,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":177,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":8436,"inlinee":5895,"source_line":916,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Sender::send
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":177,"record_offset":13744,"parent_record_offset":13708,"ancestor_record_offset":8436,"inlinee":5892,"source_line":393,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::impl$4::send::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":177,"record_offset":13836,"parent_record_offset":13796,"ancestor_record_offset":8436,"inlinee":5887,"source_line":399,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_event_loop::run_event_loop::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":177,"record_offset":9796,"parent_record_offset":9756,"ancestor_record_offset":8436,"inlinee":15288,"source_line":174,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":177,"record_offset":10092,"parent_record_offset":10060,"ancestor_record_offset":8436,"inlinee":15289,"source_line":181,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_event_loop::run_event_loop::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":177,"record_offset":12032,"parent_record_offset":12004,"ancestor_record_offset":8436,"inlinee":15290,"source_line":183,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::mode_bit_depth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":177,"record_offset":12456,"parent_record_offset":12420,"ancestor_record_offset":8436,"inlinee":5410,"source_line":275,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::process_event_batch<tiny_glade::sdl_event_loop::run_event_loop::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":184,"record_offset":248,"rva":5431456,"va":5374140576}
// 编译实例 RVAs: 0x52e0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1688065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374140576,"raw_hex":"e801c21900","size":5,"soft_fail":false,"target_va":5375829056,"va":5374140986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca9f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1689492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374140576,"raw_hex":"e894c71900","size":5,"soft_fail":false,"target_va":5375830512,"va":5374141015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_event_loop::handle_sdl_event
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-100455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374140576,"raw_hex":"e89977feff","size":5,"soft_fail":false,"target_va":5374040528,"va":5374140978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::sdl::EventPump::wait_event_timeout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1688305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374140576,"raw_hex":"e8f1c21900","size":5,"soft_fail":false,"target_va":5375828928,"va":5374140618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tiny_glade::sdl_event_loop::run_event_loop::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":184,"record_offset":440,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":15580,"source_line":87,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::run_event_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3CDC
// 首条位置: {"module":184,"inlinee":15580}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::run_event_loop::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BB8
// 首条位置: {"module":177,"inlinee":15288}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":9856,"parent_record_offset":9796,"ancestor_record_offset":9796,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":177,"record_offset":9976,"parent_record_offset":9796,"ancestor_record_offset":9796,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":10060,"parent_record_offset":9796,"ancestor_record_offset":9796,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":10460,"parent_record_offset":9796,"ancestor_record_offset":9796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":10332,"parent_record_offset":10308,"ancestor_record_offset":9796,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":9796,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":10380,"parent_record_offset":10356,"ancestor_record_offset":9796,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":9796,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":10960,"parent_record_offset":10932,"ancestor_record_offset":9796,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":10284,"parent_record_offset":10260,"ancestor_record_offset":9796,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":10260,"parent_record_offset":10236,"ancestor_record_offset":9796,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":10512,"parent_record_offset":10484,"ancestor_record_offset":9796,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":10484,"parent_record_offset":10460,"ancestor_record_offset":9796,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":11160,"parent_record_offset":11136,"ancestor_record_offset":9796,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":10596,"parent_record_offset":10568,"ancestor_record_offset":9796,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":177,"record_offset":10568,"parent_record_offset":10540,"ancestor_record_offset":9796,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":177,"record_offset":10540,"parent_record_offset":10512,"ancestor_record_offset":9796,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":177,"record_offset":9888,"parent_record_offset":9856,"ancestor_record_offset":9796,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":177,"record_offset":9912,"parent_record_offset":9888,"ancestor_record_offset":9796,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":177,"record_offset":11304,"parent_record_offset":11272,"ancestor_record_offset":9796,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":177,"record_offset":10012,"parent_record_offset":9976,"ancestor_record_offset":9796,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":177,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":9796,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":9796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":177,"record_offset":9944,"parent_record_offset":9856,"ancestor_record_offset":9796,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":177,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":9796,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":177,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":9796,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":177,"record_offset":10092,"parent_record_offset":10060,"ancestor_record_offset":9796,"inlinee":15289,"source_line":181,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1576
// 首条位置: {"module":8,"inlinee":5494}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":8,"record_offset":21092,"parent_record_offset":21064,"ancestor_record_offset":21064,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":8,"record_offset":21120,"parent_record_offset":21092,"ancestor_record_offset":21064,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":8,"record_offset":21144,"parent_record_offset":21120,"ancestor_record_offset":21064,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":8,"record_offset":21168,"parent_record_offset":21144,"ancestor_record_offset":21064,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":8,"record_offset":21192,"parent_record_offset":21168,"ancestor_record_offset":21064,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BB9
// 首条位置: {"module":177,"inlinee":15289}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":10092,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":10332,"parent_record_offset":10308,"ancestor_record_offset":10092,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":10092,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":10380,"parent_record_offset":10356,"ancestor_record_offset":10092,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":10092,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":10284,"parent_record_offset":10260,"ancestor_record_offset":10092,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":10260,"parent_record_offset":10236,"ancestor_record_offset":10092,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":10140,"parent_record_offset":10116,"ancestor_record_offset":10092,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::run_event_loop::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BBA
// 首条位置: {"module":177,"inlinee":15290}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":12032,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":12032,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":12032,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":12032,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":12272,"parent_record_offset":12216,"ancestor_record_offset":12032,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":12244,"parent_record_offset":12216,"ancestor_record_offset":12032,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":12216,"parent_record_offset":12192,"ancestor_record_offset":12032,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":12192,"parent_record_offset":12168,"ancestor_record_offset":12032,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":12096,"parent_record_offset":12056,"ancestor_record_offset":12032,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::handle_sdl_event
// 记录形式: compiled_procedure; instances=1
// 源码行: 282
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":20528,"rva":5331408,"va":5374040528}
// 编译实例 RVAs: 0x5159d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10568534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e85643a100","size":5,"soft_fail":false,"target_va":5384609408,"va":5374040869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t1636993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e881fa1800","size":5,"soft_fail":false,"target_va":5375678704,"va":5374041706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400adf70
//     owner=unresolved_namespace; count=18
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4619571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e8cd82b9ff","size":5,"soft_fail":false,"target_va":5369421680,"va":5374041246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<bevy_input::keyboard::Key> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e8e8120000","size":5,"soft_fail":false,"target_va":5374049168,"va":5374044323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CursorVisibilityState::custom_cursor_size_px
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4486456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e838754400","size":5,"soft_fail":false,"target_va":5378529616,"va":5374043155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CursorVisibilityState::custom_cursor_visible
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4486491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e85b754400","size":5,"soft_fail":false,"target_va":5378529632,"va":5374043136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CustomCursorSsboGpuData::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4486850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e8c2764400","size":5,"soft_fail":false,"target_va":5378529920,"va":5374043065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] smol_str::Repr::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t559992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e8788b0800","size":5,"soft_fail":false,"target_va":5374602048,"va":5374042051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::Sender$LT$T$GT$::send
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t-4342890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e896bbbdff","size":5,"soft_fail":false,"target_va":5369698176,"va":5374041061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_event_loop::convert_keyboard_input
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e8e20d0000","size":5,"soft_fail":false,"target_va":5374044672,"va":5374041113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_event_loop::extract_available_monitors_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e89d0a0000","size":5,"soft_fail":false,"target_va":5374046512,"va":5374043790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_setup::monitor_match_from_sdl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1164937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e87739eeff","size":5,"soft_fail":false,"target_va":5372876624,"va":5374041556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::keyboard::KeyboardUtil::mod_state
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1792578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e8425a1b00","size":5,"soft_fail":false,"target_va":5375833568,"va":5374040985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::current_display_mode
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1787972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e844481b00","size":5,"soft_fail":false,"target_va":5375831872,"va":5374043895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::display_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1791149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e8ad541b00","size":5,"soft_fail":false,"target_va":5375832640,"va":5374041486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::fullscreen_state
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1792065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e841581b00","size":5,"soft_fail":false,"target_va":5375833088,"va":5374041018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::id
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1792416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e8a0591b00","size":5,"soft_fail":false,"target_va":5375833328,"va":5374040907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1790440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e8e8511b00","size":5,"soft_fail":false,"target_va":5375833360,"va":5374042915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::vulkan_drawable_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1790323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374040528,"raw_hex":"e873511b00","size":5,"soft_fail":false,"target_va":5375833264,"va":5374042936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t1635928","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374040528,"raw_hex":"e958f61800","size":5,"soft_fail":false,"target_va":5375678704,"va":5374042771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":25876,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":20864,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":177,"record_offset":22536,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":177,"record_offset":20724,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":23904,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":23876,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":21836,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":24888,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":12876,"source_line":1964,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":21292,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":21152,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":11666,"source_line":646,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":21488,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":177,"record_offset":23484,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":177,"record_offset":20920,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":20780,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":177,"record_offset":21124,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::sdl_event_loop::convert_sdl_mouse_btn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":177,"record_offset":22352,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":15305,"source_line":547,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] usdl::keyboard::Mod::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":177,"record_offset":21032,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":15297,"source_line":691,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] usdl_sys::video::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl-sys/src/video.rs
//     evidence={"module":177,"record_offset":21096,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":15301,"source_line":31,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":21720,"parent_record_offset":21696,"ancestor_record_offset":20528,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":21744,"parent_record_offset":21720,"ancestor_record_offset":20528,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":21768,"parent_record_offset":21744,"ancestor_record_offset":20528,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":21696,"parent_record_offset":21640,"ancestor_record_offset":20528,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":21668,"parent_record_offset":21640,"ancestor_record_offset":20528,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":21640,"parent_record_offset":21616,"ancestor_record_offset":20528,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":20528,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":177,"record_offset":23508,"parent_record_offset":23484,"ancestor_record_offset":20528,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":177,"record_offset":20748,"parent_record_offset":20724,"ancestor_record_offset":20528,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":20888,"parent_record_offset":20864,"ancestor_record_offset":20528,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":20528,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":21316,"parent_record_offset":21292,"ancestor_record_offset":20528,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":21176,"parent_record_offset":21152,"ancestor_record_offset":20528,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":20804,"parent_record_offset":20780,"ancestor_record_offset":20528,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":177,"record_offset":20828,"parent_record_offset":20804,"ancestor_record_offset":20528,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":20968,"parent_record_offset":20944,"ancestor_record_offset":20528,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":177,"record_offset":20944,"parent_record_offset":20920,"ancestor_record_offset":20528,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":177,"record_offset":20992,"parent_record_offset":20968,"ancestor_record_offset":20528,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::SmolStr::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":177,"record_offset":22584,"parent_record_offset":22560,"ancestor_record_offset":20528,"inlinee":15309,"source_line":81,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$26::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":177,"record_offset":22560,"parent_record_offset":22536,"ancestor_record_offset":20528,"inlinee":15307,"source_line":325,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_event_loop::handle_sdl_event::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":177,"record_offset":23932,"parent_record_offset":23904,"ancestor_record_offset":20528,"inlinee":15312,"source_line":444,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_event_loop::handle_sdl_event::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":177,"record_offset":21868,"parent_record_offset":21836,"ancestor_record_offset":20528,"inlinee":15303,"source_line":326,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] usdl::keyboard::_::InternalBitFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":177,"record_offset":21060,"parent_record_offset":21032,"ancestor_record_offset":20528,"inlinee":15299,"source_line":691,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::handle_sdl_event::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 326
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BC7
// 首条位置: {"module":177,"inlinee":15303}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::handle_sdl_event::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 444
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BD0
// 首条位置: {"module":177,"inlinee":15312}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::extract_available_monitors_info
// 记录形式: compiled_procedure; instances=1
// 源码行: 503
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":177,"record_offset":35120,"rva":5337392,"va":5374046512}
// 编译实例 RVAs: 0x517130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t1631464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e8e8e41800","size":5,"soft_fail":false,"target_va":5375678704,"va":5374047235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b4f00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4598303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e8e1d5b9ff","size":5,"soft_fail":false,"target_va":5369450240,"va":5374048538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b64a0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4592571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e845ecb9ff","size":5,"soft_fail":false,"target_va":5369455776,"va":5374048342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c5a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4529733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e8bbe1baff","size":5,"soft_fail":false,"target_va":5369518704,"va":5374048432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42303631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e88f808502","size":5,"soft_fail":false,"target_va":5416350832,"va":5374047196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42303584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e860808502","size":5,"soft_fail":false,"target_va":5416351168,"va":5374047579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::sdl_setup::enumerate_displays
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1170480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e8d023eeff","size":5,"soft_fail":false,"target_va":5372876144,"va":5374046619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::PixelFormatEnum::byte_size_per_pixel
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1785366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e8163e1b00","size":5,"soft_fail":false,"target_va":5375832512,"va":5374047141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::desktop_display_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1784730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e89a3b1b00","size":5,"soft_fail":false,"target_va":5375832064,"va":5374047329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::display_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1784232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e8a8391b00","size":5,"soft_fail":false,"target_va":5375831328,"va":5374047091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::VideoSubsystem::num_display_modes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1784785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374046512,"raw_hex":"e8d13b1b00","size":5,"soft_fail":false,"target_va":5375831744,"va":5374046954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":35828,"parent_record_offset":35120,"ancestor_record_offset":35120,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":35440,"parent_record_offset":35120,"ancestor_record_offset":35120,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":35324,"parent_record_offset":35120,"ancestor_record_offset":35120,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":35680,"parent_record_offset":35120,"ancestor_record_offset":35120,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":177,"record_offset":35856,"parent_record_offset":35120,"ancestor_record_offset":35120,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":177,"record_offset":37252,"parent_record_offset":35120,"ancestor_record_offset":35120,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":36276,"parent_record_offset":35120,"ancestor_record_offset":35120,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":177,"record_offset":36900,"parent_record_offset":35120,"ancestor_record_offset":35120,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::impl$16::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":177,"record_offset":35300,"parent_record_offset":35120,"ancestor_record_offset":35120,"inlinee":11548,"source_line":71,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::sdl_setup::video_mode_from_sdl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":177,"record_offset":36188,"parent_record_offset":35120,"ancestor_record_offset":35120,"inlinee":13040,"source_line":267,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":36476,"parent_record_offset":36452,"ancestor_record_offset":35120,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":36500,"parent_record_offset":36476,"ancestor_record_offset":35120,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":36524,"parent_record_offset":36500,"ancestor_record_offset":35120,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":177,"record_offset":36452,"parent_record_offset":36396,"ancestor_record_offset":35120,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":35556,"parent_record_offset":35524,"ancestor_record_offset":35120,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":36424,"parent_record_offset":36396,"ancestor_record_offset":35120,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":36396,"parent_record_offset":36372,"ancestor_record_offset":35120,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":35604,"parent_record_offset":35580,"ancestor_record_offset":35120,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":35580,"parent_record_offset":35556,"ancestor_record_offset":35120,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":177,"record_offset":36372,"parent_record_offset":36348,"ancestor_record_offset":35120,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":35524,"parent_record_offset":35476,"ancestor_record_offset":35120,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":177,"record_offset":35476,"parent_record_offset":35440,"ancestor_record_offset":35120,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":37456,"parent_record_offset":37432,"ancestor_record_offset":35120,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":177,"record_offset":37916,"parent_record_offset":37892,"ancestor_record_offset":35120,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":36244,"parent_record_offset":36188,"ancestor_record_offset":35120,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":35920,"parent_record_offset":35888,"ancestor_record_offset":35120,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":177,"record_offset":35888,"parent_record_offset":35856,"ancestor_record_offset":35120,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":177,"record_offset":35352,"parent_record_offset":35324,"ancestor_record_offset":35120,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":177,"record_offset":35376,"parent_record_offset":35352,"ancestor_record_offset":35120,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":177,"record_offset":38044,"parent_record_offset":38016,"ancestor_record_offset":35120,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":36300,"parent_record_offset":36276,"ancestor_record_offset":35120,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":177,"record_offset":35408,"parent_record_offset":35324,"ancestor_record_offset":35120,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":177,"record_offset":35740,"parent_record_offset":35680,"ancestor_record_offset":35120,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":177,"record_offset":35768,"parent_record_offset":35680,"ancestor_record_offset":35120,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":177,"record_offset":35712,"parent_record_offset":35680,"ancestor_record_offset":35120,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":35792,"parent_record_offset":35768,"ancestor_record_offset":35120,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":177,"record_offset":35644,"parent_record_offset":35476,"ancestor_record_offset":35120,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_event_loop::extract_available_monitors_info::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_event_loop.rs
//     evidence={"module":177,"record_offset":37276,"parent_record_offset":37252,"ancestor_record_offset":35120,"inlinee":15316,"source_line":522,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::mode_bit_depth
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":177,"record_offset":36216,"parent_record_offset":36188,"ancestor_record_offset":35120,"inlinee":5410,"source_line":275,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::video_mode_from_sdl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":177,"record_offset":37300,"parent_record_offset":37276,"ancestor_record_offset":35120,"inlinee":13040,"source_line":267,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::extract_available_monitors_info::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 522
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BD4
// 首条位置: {"module":177,"inlinee":15316}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::sdl_setup::video_mode_from_sdl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":177,"record_offset":37300,"parent_record_offset":37276,"ancestor_record_offset":37276,"inlinee":13040,"source_line":267,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":177,"record_offset":37364,"parent_record_offset":37300,"ancestor_record_offset":37276,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::sdl_setup::mode_bit_depth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/sdl_setup.rs
//     evidence={"module":177,"record_offset":37332,"parent_record_offset":37300,"ancestor_record_offset":37276,"inlinee":5410,"source_line":275,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::convert_sdl_mouse_btn
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 547
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BC9
// 首条位置: {"module":177,"inlinee":15305}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::convert_sdl_scancode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 563
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BD1
// 首条位置: {"module":177,"inlinee":15313}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::sdl_event_loop::convert_sdl_keycode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 676
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3BD2
// 首条位置: {"module":177,"inlinee":15314}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":177,"record_offset":28116,"parent_record_offset":27892,"ancestor_record_offset":27892,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::SmolStr::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":177,"record_offset":28164,"parent_record_offset":28140,"ancestor_record_offset":27892,"inlinee":15309,"source_line":81,"archive":"_pdb/C/modules/00177.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$26::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":177,"record_offset":28140,"parent_record_offset":28116,"ancestor_record_offset":27892,"inlinee":15307,"source_line":325,"archive":"_pdb/C/modules/00177.jsonl.gz"}
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
// 完整结构及原始类型记录: sdl_event_loop.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sdl_event_loop.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sdl_event_loop.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)
// 候选参数类型: ["ref$<tuple$<i64,usdl::video::DisplayMode> >", "ref$<tuple$<i64,usdl::video::DisplayMode> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::convert_keyboard_input
// 信息 ID: e15b6620ab41105628fadc34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::convert_sdl_keycode
// 信息 ID: 7402a4d5ba731caf6bf6876e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::convert_keyboard_input; module=177, inline_record=27892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::convert_sdl_mouse_btn
// 信息 ID: e651f7ce0fca3b7fcf1000a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::handle_sdl_event; module=177, inline_record=22352; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::handle_sdl_event; module=177, inline_record=23152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::convert_sdl_scancode
// 信息 ID: 4148b47e59fe957155242293
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::convert_keyboard_input; module=177, inline_record=27404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::extract_available_monitors_info
// 信息 ID: c03d1cd293777e24e0770753
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_event_loop::run_event_loop @ 0x5136b5 -> 0x517130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::extract_available_monitors_info::closure$0
// 信息 ID: c94b5d4a2cbbf6aba67035d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::sdl_event_loop::extract_available_monitors_info", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::extract_available_monitors_info; module=177, inline_record=37276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::handle_sdl_event
// 信息 ID: 92e02c951f9c4572351b61b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_event_loop::process_event_batch<tiny_glade::sdl_event_loop::run_event_loop::closure_env$0> @ 0x52e371 -> 0x5159d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_event_loop::process_event_batch<tiny_glade::sdl_event_loop::run_event_loop::closure_env$0> @ 0x52e232 -> 0x5159d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::handle_sdl_event::closure$2
// 信息 ID: 2e645f3cf97fe556e1da1070
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::sdl_event_loop::handle_sdl_event", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::handle_sdl_event; module=177, inline_record=23932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::handle_sdl_event::closure$3
// 信息 ID: 9048b2497a3cf3c8dff3ad80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::sdl_event_loop::handle_sdl_event", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::handle_sdl_event; module=177, inline_record=21868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::process_event_batch<tiny_glade::sdl_event_loop::run_event_loop::closure_env$0>
// 信息 ID: 0c91658a16069b8c6858e853
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tiny_glade::sdl_event_loop::process_event_batch -> ["tiny_glade::sdl_event_loop::run_event_loop::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_event_loop::run_event_loop @ 0x513866 -> 0x52e0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::run_event_loop
// 信息 ID: ae30375ba9fde9b5dc8538f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x430392 -> 0x513540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::run_event_loop::closure$0
// 信息 ID: 364754db43f67bf7292c9d9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::sdl_event_loop::run_event_loop", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::sdl_event_loop::run_event_loop::closure_env$0 <- tiny_glade::sdl_event_loop::process_event_batch<tiny_glade::sdl_event_loop::run_event_loop::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::process_event_batch<tiny_glade::sdl_event_loop::run_event_loop::closure_env$0>; module=184, inline_record=440; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::process_event_batch<tiny_glade::sdl_event_loop::run_event_loop::closure_env$0>; module=184, inline_record=500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::run_event_loop::closure$2
// 信息 ID: a0ee77bd639555dc4c189604
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::sdl_event_loop::run_event_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::run_event_loop; module=177, inline_record=9796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure$0
// 信息 ID: 90f34236dc25f33be829401f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::sdl_event_loop::run_event_loop", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure_env$0 <- alloc::vec::into_iter::impl$6::try_fold<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tiny_glade::sdl_setup::SdlMonitorInfo,tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::try_fold<tiny_glade::sdl_setup::SdlMonitorInfo,alloc::alloc::Global,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tiny_glade::sdl_setup::SdlMonitorInfo,tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tiny_glade::sdl_setup::SdlMonitorInfo,tuple$<> > > >; module=8, inline_record=21064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::run_event_loop::closure$2::closure$1
// 信息 ID: f4796ef61ba2afe4291c09f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::sdl_event_loop::run_event_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::run_event_loop; module=177, inline_record=10092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::sdl_event_loop::run_event_loop::closure$3
// 信息 ID: 89af7f77c369579fd933962f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::sdl_event_loop::run_event_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::sdl_event_loop::run_event_loop; module=177, inline_record=12032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
