// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-iced/src/gui_runner_sdl.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 9944cd1e00be39b92a51111ce9c9c1adc8b1fa8954c115a13607683002533561
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_runner_sdl.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 184, 275
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use iced_core; // namespace candidate
//   package identities: [{"name":"iced_core","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_graphics; // namespace candidate
//   package identities: [{"name":"iced_graphics","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_runtime; // namespace candidate
//   package identities: [{"name":"iced_runtime","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_style; // namespace candidate
//   package identities: [{"name":"iced_style","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_tiny_skia; // namespace candidate
//   package identities: [{"name":"iced_tiny_skia","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use raw_window_handle; // namespace candidate
//   package identities: [{"name":"raw-window-handle","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.1"}]
// use tg_iced; // namespace candidate
//   package identities: [{"name":"tg-iced","source":"local_or_path","version":"0.1.0"}]
// use usdl; // namespace candidate
//   package identities: [{"name":"usdl","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_sdl::run_gui_with_options::closure$0<tg_gpu_selector::gpu_selection::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":184,"record_offset":11464,"rva":5436336,"va":5374145456}
// 编译实例 RVAs: 0x52f3b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401e2420
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3461593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374145456,"raw_hex":"e8272ecbff","size":5,"soft_fail":false,"target_va":5370684448,"va":5374146036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42205780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374145456,"raw_hex":"e854028402","size":5,"soft_fail":false,"target_va":5416351392,"va":5374145607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_iced::gui_runner_sdl::translate_mouse_button
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1668821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374145456,"raw_hex":"e8d5761900","size":5,"soft_fail":false,"target_va":5375814368,"va":5374145542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t1533171","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374145456,"raw_hex":"e9f3641700","size":5,"soft_fail":false,"target_va":5375678704,"va":5374145528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":184,"record_offset":12640,"parent_record_offset":11464,"ancestor_record_offset":11464,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":11656,"parent_record_offset":11464,"ancestor_record_offset":11464,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":11952,"parent_record_offset":11464,"ancestor_record_offset":11464,"inlinee":9582,"source_line":62,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":11840,"parent_record_offset":11816,"ancestor_record_offset":11464,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":11864,"parent_record_offset":11840,"ancestor_record_offset":11464,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":11888,"parent_record_offset":11864,"ancestor_record_offset":11464,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":11816,"parent_record_offset":11760,"ancestor_record_offset":11464,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":12512,"parent_record_offset":12480,"ancestor_record_offset":11464,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":11788,"parent_record_offset":11760,"ancestor_record_offset":11464,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":11760,"parent_record_offset":11736,"ancestor_record_offset":11464,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":12560,"parent_record_offset":12536,"ancestor_record_offset":11464,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":12536,"parent_record_offset":12512,"ancestor_record_offset":11464,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":11736,"parent_record_offset":11716,"ancestor_record_offset":11464,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":12480,"parent_record_offset":12448,"ancestor_record_offset":11464,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":11980,"parent_record_offset":11952,"ancestor_record_offset":11464,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":12012,"parent_record_offset":11980,"ancestor_record_offset":11464,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":11676,"parent_record_offset":11656,"ancestor_record_offset":11464,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":12600,"parent_record_offset":12448,"ancestor_record_offset":11464,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":184,"record_offset":12832,"rva":5437088,"va":5374146208}
// 编译实例 RVAs: 0x52f6a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10500858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8fa3aa000","size":5,"soft_fail":false,"target_va":5384648032,"va":5374147169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::configure_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1698081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e821e91900","size":5,"soft_fail":false,"target_va":5375845216,"va":5374147130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1698133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e855e91900","size":5,"soft_fail":false,"target_va":5375844784,"va":5374146646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1532160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e800611700","size":5,"soft_fail":false,"target_va":5375678688,"va":5374146523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t1530494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e87e5a1700","size":5,"soft_fail":false,"target_va":5375678704,"va":5374148205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5424321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e83f3badff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374146508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007fed0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4916527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8d1fab4ff","size":5,"soft_fail":false,"target_va":5369233104,"va":5374149626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140080710
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4913734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8ba05b5ff","size":5,"soft_fail":false,"target_va":5369235216,"va":5374148945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089690
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4880365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e81388b5ff","size":5,"soft_fail":false,"target_va":5369271952,"va":5374152312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a650
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4806643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e80da8b6ff","size":5,"soft_fail":false,"target_va":5369341520,"va":5374148158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ac970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4736057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8c7bbb7ff","size":5,"soft_fail":false,"target_va":5369416048,"va":5374152100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400af890
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4721539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e87df4b7ff","size":5,"soft_fail":false,"target_va":5369428112,"va":5374149646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4720333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e833f9b7ff","size":5,"soft_fail":false,"target_va":5369431360,"va":5374151688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b3050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4708180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8ac28b8ff","size":5,"soft_fail":false,"target_va":5369442384,"va":5374150559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bfcd0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4657065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e857f0b8ff","size":5,"soft_fail":false,"target_va":5369494736,"va":5374151796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c46b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4636020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e88c42b9ff","size":5,"soft_fail":false,"target_va":5369513648,"va":5374149663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401e2420
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3467789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8f315cbff","size":5,"soft_fail":false,"target_va":5370684448,"va":5374152232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597e60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t427474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8d2850600","size":5,"soft_fail":false,"target_va":5374574176,"va":5374146697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405980a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t425369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8997d0600","size":5,"soft_fail":false,"target_va":5374574752,"va":5374149378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c4270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1655007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8df401900","size":5,"soft_fail":false,"target_va":5375804016,"va":5374149004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1681483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e84ba81900","size":5,"soft_fail":false,"target_va":5375829056,"va":5374147568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1682917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8e5ad1900","size":5,"soft_fail":false,"target_va":5375830512,"va":5374147590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cb280
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1686315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e82bbb1900","size":5,"soft_fail":false,"target_va":5375832704,"va":5374146384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cb330
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1686436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8a4bb1900","size":5,"soft_fail":false,"target_va":5375832880,"va":5374146439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cb780
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1684457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8e9b31900","size":5,"soft_fail":false,"target_va":5375833984,"va":5374149522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42204148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8f4fb8302","size":5,"soft_fail":false,"target_va":5416351392,"va":5374147239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner_sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42910768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e830c48e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374146283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42470512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8700c8802","size":5,"soft_fail":false,"target_va":5416620769,"va":5374150252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1666798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8ee6e1900","size":5,"soft_fail":false,"target_va":5375816416,"va":5374149613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > > >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e899ebffff","size":5,"soft_fail":false,"target_va":5374143600,"va":5374148818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42471403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8eb0f8802","size":5,"soft_fail":false,"target_va":5416621632,"va":5374150224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1790004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e834501b00","size":5,"soft_fail":false,"target_va":5375936352,"va":5374146343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-3481787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e845dfcaff","size":5,"soft_fail":false,"target_va":5370664992,"va":5374146774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-3473469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8c3ffcaff","size":5,"soft_fail":false,"target_va":5370674512,"va":5374147976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_iced::gui_runner_sdl::run_gui_with_options::closure$0<tg_gpu_selector::gpu_selection::Controls>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8c8f7ffff","size":5,"soft_fail":false,"target_va":5374145456,"va":5374147555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::mouse::Cursor::from_system
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1681199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e82fa71900","size":5,"soft_fail":false,"target_va":5375830640,"va":5374149436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::mouse::Cursor::set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1681250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e862a71900","size":5,"soft_fail":false,"target_va":5375830736,"va":5374149481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::sdl::EventPump::wait_event_timeout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1681393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e8f1a71900","size":5,"soft_fail":false,"target_va":5375828928,"va":5374147530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1686545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e811bc1900","size":5,"soft_fail":false,"target_va":5375833344,"va":5374146794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::video::Window::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1687057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374146208,"raw_hex":"e811be1900","size":5,"soft_fail":false,"target_va":5375833360,"va":5374146298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5374146208,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5374149712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner_sdl::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_sdl.rs
//     evidence={"module":184,"record_offset":12992,"parent_record_offset":12832,"ancestor_record_offset":12832,"inlinee":15585,"source_line":51,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":13616,"parent_record_offset":13592,"ancestor_record_offset":12832,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":13640,"parent_record_offset":13616,"ancestor_record_offset":12832,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":12832,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":12832,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":13664,"parent_record_offset":13640,"ancestor_record_offset":12832,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":12832,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":13592,"parent_record_offset":13560,"ancestor_record_offset":12832,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":14580,"parent_record_offset":14492,"ancestor_record_offset":12832,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":13560,"parent_record_offset":13524,"ancestor_record_offset":12832,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":13524,"parent_record_offset":13496,"ancestor_record_offset":12832,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":15012,"parent_record_offset":14968,"ancestor_record_offset":12832,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":17328,"parent_record_offset":17300,"ancestor_record_offset":12832,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":13968,"parent_record_offset":13936,"ancestor_record_offset":12832,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":14520,"parent_record_offset":14492,"ancestor_record_offset":12832,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":12832,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":14016,"parent_record_offset":13992,"ancestor_record_offset":12832,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":12832,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":14468,"parent_record_offset":14444,"ancestor_record_offset":12832,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":17084,"parent_record_offset":17048,"ancestor_record_offset":12832,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":17048,"parent_record_offset":17024,"ancestor_record_offset":12832,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":13936,"parent_record_offset":13892,"ancestor_record_offset":12832,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":16504,"parent_record_offset":16480,"ancestor_record_offset":12832,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":13864,"parent_record_offset":13836,"ancestor_record_offset":12832,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":13892,"parent_record_offset":13864,"ancestor_record_offset":12832,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":15200,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":14380,"parent_record_offset":14352,"ancestor_record_offset":12832,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":15480,"parent_record_offset":15444,"ancestor_record_offset":12832,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":15444,"parent_record_offset":15416,"ancestor_record_offset":12832,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":12832,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":15256,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":15552,"parent_record_offset":15528,"ancestor_record_offset":12832,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":184,"record_offset":17360,"parent_record_offset":17328,"ancestor_record_offset":12832,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":184,"record_offset":13752,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":184,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":12832,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":184,"record_offset":15528,"parent_record_offset":15504,"ancestor_record_offset":12832,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":184,"record_offset":15504,"parent_record_offset":15480,"ancestor_record_offset":12832,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":17424,"parent_record_offset":17388,"ancestor_record_offset":12832,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":17388,"parent_record_offset":17360,"ancestor_record_offset":12832,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":184,"record_offset":14548,"parent_record_offset":14520,"ancestor_record_offset":12832,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":184,"record_offset":14208,"parent_record_offset":14184,"ancestor_record_offset":12832,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":14124,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":14184,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":16380,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":184,"record_offset":15624,"parent_record_offset":15600,"ancestor_record_offset":12832,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":14352,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":15224,"parent_record_offset":15200,"ancestor_record_offset":12832,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":15600,"parent_record_offset":15576,"ancestor_record_offset":12832,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":15328,"parent_record_offset":15256,"ancestor_record_offset":12832,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":17116,"parent_record_offset":17084,"ancestor_record_offset":12832,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":15576,"parent_record_offset":15552,"ancestor_record_offset":12832,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":15356,"parent_record_offset":15256,"ancestor_record_offset":12832,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":15296,"parent_record_offset":15256,"ancestor_record_offset":12832,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":15380,"parent_record_offset":15356,"ancestor_record_offset":12832,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":14056,"parent_record_offset":13892,"ancestor_record_offset":12832,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":184,"record_offset":13368,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":184,"record_offset":13724,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":17144,"parent_record_offset":17084,"ancestor_record_offset":12832,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":17176,"parent_record_offset":17048,"ancestor_record_offset":12832,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":184,"record_offset":14232,"parent_record_offset":14208,"ancestor_record_offset":12832,"inlinee":15594,"source_line":7,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::interaction::impl$3::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/interaction.rs
//     evidence={"module":184,"record_offset":16596,"parent_record_offset":16572,"ancestor_record_offset":12832,"inlinee":12833,"source_line":2,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::size::impl$12::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/size.rs
//     evidence={"module":184,"record_offset":13776,"parent_record_offset":13752,"ancestor_record_offset":12832,"inlinee":12823,"source_line":4,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":184,"record_offset":14968,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":15597,"source_line":104,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::debug::Debug::overlay
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/debug/null.rs
//     evidence={"module":184,"record_offset":16480,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":12831,"source_line":44,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::program::state::State::is_queue_empty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":14096,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":15590,"source_line":74,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":13836,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":9582,"source_line":62,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":184,"record_offset":14268,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":184,"record_offset":13468,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":15587,"source_line":31,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":184,"record_offset":13496,"parent_record_offset":13468,"ancestor_record_offset":12832,"inlinee":15589,"source_line":139,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] raw_window_handle::borrowed::impl$12::window_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-window-handle","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-window-handle-0.6.1/src/borrowed.rs
//     evidence={"module":184,"record_offset":13340,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":6367,"source_line":173,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] raw_window_handle::borrowed::impl$1::display_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-window-handle","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-window-handle-0.6.1/src/borrowed.rs
//     evidence={"module":184,"record_offset":13404,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":6369,"source_line":45,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_sdl::process_event_batch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_sdl.rs
//     evidence={"module":184,"record_offset":14312,"parent_record_offset":12992,"ancestor_record_offset":12832,"inlinee":15595,"source_line":299,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_sdl::run_gui_with_options::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_sdl.rs
//     evidence={"module":184,"record_offset":14152,"parent_record_offset":14124,"ancestor_record_offset":12832,"inlinee":15592,"source_line":215,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_sdl::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_sdl.rs
//     evidence={"module":184,"record_offset":16408,"parent_record_offset":16380,"ancestor_record_offset":12832,"inlinee":15598,"source_line":253,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_sdl::impl$2::display_handle
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":275,"record_offset":388,"rva":7105216,"va":5375814336}
// 编译实例 RVAs: 0x6c6ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_sdl::impl$3::window_handle
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":275,"record_offset":264,"rva":7105184,"va":5375814304}
// 编译实例 RVAs: 0x6c6aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_sdl::run_gui_with_options
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3CE1
// 首条位置: {"module":184,"inlinee":15585}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":15200,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":15256,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":184,"record_offset":13752,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":14124,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":14184,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":16380,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":14352,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":184,"record_offset":13368,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":184,"record_offset":13724,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":184,"record_offset":14968,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":15597,"source_line":104,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::debug::Debug::overlay
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/debug/null.rs
//     evidence={"module":184,"record_offset":16480,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":12831,"source_line":44,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::is_queue_empty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":14096,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":15590,"source_line":74,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":13836,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":9582,"source_line":62,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":184,"record_offset":14268,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":184,"record_offset":13468,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":15587,"source_line":31,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] raw_window_handle::borrowed::impl$12::window_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-window-handle","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-window-handle-0.6.1/src/borrowed.rs
//     evidence={"module":184,"record_offset":13340,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":6367,"source_line":173,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] raw_window_handle::borrowed::impl$1::display_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-window-handle","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-window-handle-0.6.1/src/borrowed.rs
//     evidence={"module":184,"record_offset":13404,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":6369,"source_line":45,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::gui_runner_sdl::process_event_batch
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_sdl.rs
//     evidence={"module":184,"record_offset":14312,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":15595,"source_line":299,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":13616,"parent_record_offset":13592,"ancestor_record_offset":12992,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":13640,"parent_record_offset":13616,"ancestor_record_offset":12992,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":12992,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":12992,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":13664,"parent_record_offset":13640,"ancestor_record_offset":12992,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":12992,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":13592,"parent_record_offset":13560,"ancestor_record_offset":12992,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":14580,"parent_record_offset":14492,"ancestor_record_offset":12992,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":13560,"parent_record_offset":13524,"ancestor_record_offset":12992,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":13524,"parent_record_offset":13496,"ancestor_record_offset":12992,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":15012,"parent_record_offset":14968,"ancestor_record_offset":12992,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":17328,"parent_record_offset":17300,"ancestor_record_offset":12992,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":13968,"parent_record_offset":13936,"ancestor_record_offset":12992,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":14520,"parent_record_offset":14492,"ancestor_record_offset":12992,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":12992,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":14016,"parent_record_offset":13992,"ancestor_record_offset":12992,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":12992,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":14468,"parent_record_offset":14444,"ancestor_record_offset":12992,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":17084,"parent_record_offset":17048,"ancestor_record_offset":12992,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":17048,"parent_record_offset":17024,"ancestor_record_offset":12992,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":13936,"parent_record_offset":13892,"ancestor_record_offset":12992,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":16504,"parent_record_offset":16480,"ancestor_record_offset":12992,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":13864,"parent_record_offset":13836,"ancestor_record_offset":12992,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":13892,"parent_record_offset":13864,"ancestor_record_offset":12992,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":14380,"parent_record_offset":14352,"ancestor_record_offset":12992,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":15480,"parent_record_offset":15444,"ancestor_record_offset":12992,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":15444,"parent_record_offset":15416,"ancestor_record_offset":12992,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":12992,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":15552,"parent_record_offset":15528,"ancestor_record_offset":12992,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":184,"record_offset":17360,"parent_record_offset":17328,"ancestor_record_offset":12992,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":184,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":12992,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":184,"record_offset":15528,"parent_record_offset":15504,"ancestor_record_offset":12992,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":184,"record_offset":15504,"parent_record_offset":15480,"ancestor_record_offset":12992,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":17424,"parent_record_offset":17388,"ancestor_record_offset":12992,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":17388,"parent_record_offset":17360,"ancestor_record_offset":12992,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":184,"record_offset":14548,"parent_record_offset":14520,"ancestor_record_offset":12992,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":184,"record_offset":14208,"parent_record_offset":14184,"ancestor_record_offset":12992,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":184,"record_offset":15624,"parent_record_offset":15600,"ancestor_record_offset":12992,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":14408,"parent_record_offset":14380,"ancestor_record_offset":12992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":15224,"parent_record_offset":15200,"ancestor_record_offset":12992,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":15600,"parent_record_offset":15576,"ancestor_record_offset":12992,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":15328,"parent_record_offset":15256,"ancestor_record_offset":12992,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":17116,"parent_record_offset":17084,"ancestor_record_offset":12992,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":15576,"parent_record_offset":15552,"ancestor_record_offset":12992,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":15356,"parent_record_offset":15256,"ancestor_record_offset":12992,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":15296,"parent_record_offset":15256,"ancestor_record_offset":12992,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":15380,"parent_record_offset":15356,"ancestor_record_offset":12992,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":14056,"parent_record_offset":13892,"ancestor_record_offset":12992,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":17144,"parent_record_offset":17084,"ancestor_record_offset":12992,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":17176,"parent_record_offset":17048,"ancestor_record_offset":12992,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":184,"record_offset":14232,"parent_record_offset":14208,"ancestor_record_offset":12992,"inlinee":15594,"source_line":7,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::interaction::impl$3::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/interaction.rs
//     evidence={"module":184,"record_offset":16596,"parent_record_offset":16572,"ancestor_record_offset":12992,"inlinee":12833,"source_line":2,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::size::impl$12::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/size.rs
//     evidence={"module":184,"record_offset":13776,"parent_record_offset":13752,"ancestor_record_offset":12992,"inlinee":12823,"source_line":4,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":184,"record_offset":16436,"parent_record_offset":16408,"ancestor_record_offset":12992,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":184,"record_offset":13496,"parent_record_offset":13468,"ancestor_record_offset":12992,"inlinee":15589,"source_line":139,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_sdl::run_gui_with_options::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_sdl.rs
//     evidence={"module":184,"record_offset":14152,"parent_record_offset":14124,"ancestor_record_offset":12992,"inlinee":15592,"source_line":215,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_sdl::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_sdl.rs
//     evidence={"module":184,"record_offset":16408,"parent_record_offset":16380,"ancestor_record_offset":12992,"inlinee":15598,"source_line":253,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_sdl::run_gui_with_options::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 215
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3CE8
// 首条位置: {"module":184,"inlinee":15592}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_sdl::run_gui_with_options::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 253
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3CEE
// 首条位置: {"module":184,"inlinee":15598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":184,"record_offset":16436,"parent_record_offset":16408,"ancestor_record_offset":16408,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_sdl::translate_mouse_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":275,"record_offset":512,"rva":7105248,"va":5375814368}
// 编译实例 RVAs: 0x6c6ae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_sdl::process_event_batch
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 299
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3CEB
// 首条位置: {"module":184,"inlinee":15595}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: gui_runner_sdl.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_iced::gui_runner_sdl::WindowHandles
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "iced_tiny_skia::window::compositor::impl$0::create_surface<tg_iced::gui_runner_sdl::WindowHandles>", "constructor": "iced_tiny_skia::window::compositor::impl$0::create_surface", "arguments": ["tg_iced::gui_runner_sdl::WindowHandles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gui_runner_sdl.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gui_runner_sdl.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_sdl::impl$2::display_handle
// 信息 ID: 74e2d427593fa2d77551552d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 42 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x2245f7 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x2248d5 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224cc3 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224ec7 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x225253 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x225457 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x2257e3 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x2259c9 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x225dd3 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x225fd7 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x226365 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x226501 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x226844 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x226a45 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x226d53 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x227026 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_sdl::impl$3::window_handle
// 信息 ID: 60bc285789021b167e4a601c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_sdl::process_event_batch
// 信息 ID: 4caba9d9dffee1a0a1d47164
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=184, inline_record=14312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls>
// 信息 ID: 6930bebba8db697564e9c5a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner_sdl::run_gui -> ["tg_gpu_selector::gpu_selection::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42dfe6 -> 0x52f6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42f5d3 -> 0x52f6a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_sdl::run_gui_with_options
// 信息 ID: 9d0bbe0dcc58ff9ce3115125
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=184, inline_record=12992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_sdl::run_gui_with_options::closure$0<tg_gpu_selector::gpu_selection::Controls>
// 信息 ID: 9816082d1aebfbd784b120ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner_sdl::run_gui_with_options::closure$0 -> ["tg_gpu_selector::gpu_selection::Controls"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_iced::gui_runner_sdl::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x52fbe3 -> 0x52f3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x52fc1f -> 0x52f3b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_sdl::run_gui_with_options::closure$1
// 信息 ID: b429e7cb76d97b771674c607
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_iced::gui_runner_sdl::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=184, inline_record=14152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_sdl::run_gui_with_options::closure$2
// 信息 ID: fe9adf1f5ec22375c883b8ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_iced::gui_runner_sdl::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=184, inline_record=16408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_sdl::translate_mouse_button
// 信息 ID: b6b23af173f85da1071e0a69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
