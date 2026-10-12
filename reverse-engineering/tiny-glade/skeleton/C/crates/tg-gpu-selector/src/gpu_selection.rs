// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-gpu-selector/src/gpu_selection.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 125ecaeb497655f42f2206cae42108faa45fd38a77328abba5c2a20f366d383e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gpu_selection.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 137, 272, 981, 986, 988
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use iced_core; // namespace candidate
//   package identities: [{"name":"iced_core","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_runtime; // namespace candidate
//   package identities: [{"name":"iced_runtime","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_widget; // namespace candidate
//   package identities: [{"name":"iced_widget","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use open; // namespace candidate
//   package identities: [{"name":"open","source":"registry+https://github.com/rust-lang/crates.io-index","version":"5.0.0"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_gpu_selector; // namespace candidate
//   package identities: [{"name":"tg-gpu-selector","source":"local_or_path","version":"0.1.0"}]
// use tg_iced; // namespace candidate
//   package identities: [{"name":"tg-iced","source":"local_or_path","version":"0.1.0"}]
// use webbrowser; // namespace candidate
//   package identities: [{"name":"webbrowser","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.10"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::device_driver_advice
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":272,"record_offset":9248,"rva":7072896,"va":5375782016}
// 编译实例 RVAs: 0x6bec80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=57
//     evidence={"assembly":"\tcallq\t-103558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e87a6bfeff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375782241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=50
//     evidence={"assembly":"\tcallq\t-103749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8bb6afeff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375782448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=57
//     target_aliases=1699 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7060035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8bd4594ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375782222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007e380
//     owner=unresolved_namespace; count=24
//     target_aliases=39 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6569069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e893c39bff","size":5,"soft_fail":false,"target_va":5369226112,"va":5375795176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=20
//     target_aliases=66 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6486234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e826079dff","size":5,"soft_fail":false,"target_va":5369308864,"va":5375795093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac4c0
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9350845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8bdae8e00","size":5,"soft_fail":false,"target_va":5385143488,"va":5375792638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fdba90
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9550475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e88bba9100","size":5,"soft_fail":false,"target_va":5385337488,"va":5375787008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fdf1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9567237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e805fc9100","size":5,"soft_fail":false,"target_va":5385351584,"va":5375784342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe1450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9573201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e851139200","size":5,"soft_fail":false,"target_va":5385360464,"va":5375787258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=13
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40570210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8620d6b02","size":5,"soft_fail":false,"target_va":5416353040,"va":5375782825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=17
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41274784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8a0cd7502","size":5,"soft_fail":false,"target_va":5417057056,"va":5375782267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=45
//     evidence={"assembly":"\tcallq\t40831874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8820b6f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5375788890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t9012690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8d2858900","size":5,"soft_fail":false,"target_va":5384794864,"va":5375782169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t40838580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8b4256f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375782209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40828752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e850ff6e02","size":5,"soft_fail":false,"target_va":5416621600,"va":5375792843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$tg_gpu_selector..gpu_selection..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9545208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8f8a59100","size":5,"soft_fail":false,"target_va":5385342352,"va":5375797139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9384478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e81e328f00","size":5,"soft_fail":false,"target_va":5385173056,"va":5375788573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::container::Container$LT$Message$C$Theme$C$Renderer$GT$::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9566888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8a8fa9100","size":5,"soft_fail":false,"target_va":5385349360,"va":5375782467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9383926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8f62f8f00","size":5,"soft_fail":false,"target_va":5385172352,"va":5375788421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::row::Row$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9567505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e811fd9100","size":5,"soft_fail":false,"target_va":5385354384,"va":5375786874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7352639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e83f317000","size":5,"soft_fail":false,"target_va":5383134720,"va":5375782076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::driver_support
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7670545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8110b7500","size":5,"soft_fail":false,"target_va":5383453280,"va":5375782730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::gpu_selection::get_vendor_driver_update_links
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t19664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375782016,"raw_hex":"e8d04c0000","size":5,"soft_fail":false,"target_va":5375803584,"va":5375783915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375782016,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375787451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":26440,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":24976,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":9484,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":17440,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::new
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":11240,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::push_str
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":11268,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":18776,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":272,"record_offset":10916,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":272,"record_offset":15236,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":272,"record_offset":18256,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":272,"record_offset":22768,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":15736,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":272,"record_offset":37912,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18945,"source_line":77,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":272,"record_offset":13680,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":272,"record_offset":33272,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18712,"source_line":112,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":272,"record_offset":33908,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18714,"source_line":104,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":272,"record_offset":33304,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18713,"source_line":127,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":272,"record_offset":33936,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18716,"source_line":307,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":36680,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18756,"source_line":333,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::max_width
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":272,"record_offset":10884,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18940,"source_line":97,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":272,"record_offset":33244,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18710,"source_line":116,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":272,"record_offset":18832,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18801,"source_line":68,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":272,"record_offset":10856,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18765,"source_line":57,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":272,"record_offset":32588,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18751,"source_line":322,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":272,"record_offset":32332,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18758,"source_line":95,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":272,"record_offset":9568,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":18939,"source_line":7,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=57
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":10064,"parent_record_offset":10040,"ancestor_record_offset":9248,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=57
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":10088,"parent_record_offset":10064,"ancestor_record_offset":9248,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":10460,"parent_record_offset":10436,"ancestor_record_offset":9248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":10484,"parent_record_offset":10460,"ancestor_record_offset":9248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=57
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":10112,"parent_record_offset":10088,"ancestor_record_offset":9248,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":10508,"parent_record_offset":10484,"ancestor_record_offset":9248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=57
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":10040,"parent_record_offset":9888,"ancestor_record_offset":9248,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=50
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":10436,"parent_record_offset":10380,"ancestor_record_offset":9248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=45
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":11072,"parent_record_offset":11024,"ancestor_record_offset":9248,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":11024,"parent_record_offset":10988,"ancestor_record_offset":9248,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":33400,"parent_record_offset":33372,"ancestor_record_offset":9248,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":17912,"parent_record_offset":17876,"ancestor_record_offset":9248,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":9532,"parent_record_offset":9508,"ancestor_record_offset":9248,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":11496,"parent_record_offset":11460,"ancestor_record_offset":9248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":11400,"parent_record_offset":11376,"ancestor_record_offset":9248,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":9248,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":11884,"parent_record_offset":11860,"ancestor_record_offset":9248,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":10408,"parent_record_offset":10380,"ancestor_record_offset":9248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":10380,"parent_record_offset":10356,"ancestor_record_offset":9248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":11860,"parent_record_offset":11828,"ancestor_record_offset":9248,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":9248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":11520,"parent_record_offset":11496,"ancestor_record_offset":9248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":9248,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":9888,"parent_record_offset":9856,"ancestor_record_offset":9248,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":9856,"parent_record_offset":9832,"ancestor_record_offset":9248,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":9248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":9928,"parent_record_offset":9888,"ancestor_record_offset":9248,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":26464,"parent_record_offset":26440,"ancestor_record_offset":9248,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":9732,"parent_record_offset":9708,"ancestor_record_offset":9248,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":9756,"parent_record_offset":9732,"ancestor_record_offset":9248,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":9248,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":9708,"parent_record_offset":9684,"ancestor_record_offset":9248,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":272,"record_offset":9684,"parent_record_offset":9660,"ancestor_record_offset":9248,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$114::spec_to_string
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":9636,"parent_record_offset":9612,"ancestor_record_offset":9248,"inlinee":10264,"source_line":3043,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$125::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":24280,"parent_record_offset":24252,"ancestor_record_offset":9248,"inlinee":18944,"source_line":3043,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":23088,"parent_record_offset":23060,"ancestor_record_offset":9248,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":9612,"parent_record_offset":9568,"ancestor_record_offset":9248,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":9660,"parent_record_offset":9636,"ancestor_record_offset":9248,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":11340,"parent_record_offset":11316,"ancestor_record_offset":9248,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":11460,"parent_record_offset":11340,"ancestor_record_offset":9248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":11292,"parent_record_offset":11268,"ancestor_record_offset":9248,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":26488,"parent_record_offset":26464,"ancestor_record_offset":9248,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":11584,"parent_record_offset":11340,"ancestor_record_offset":9248,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":11376,"parent_record_offset":11340,"ancestor_record_offset":9248,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":9808,"parent_record_offset":9780,"ancestor_record_offset":9248,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":272,"record_offset":11316,"parent_record_offset":11292,"ancestor_record_offset":9248,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":272,"record_offset":33432,"parent_record_offset":33400,"ancestor_record_offset":9248,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":272,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":9248,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":272,"record_offset":17996,"parent_record_offset":17876,"ancestor_record_offset":9248,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":272,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":9248,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":272,"record_offset":18960,"parent_record_offset":18936,"ancestor_record_offset":9248,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":272,"record_offset":33496,"parent_record_offset":33460,"ancestor_record_offset":9248,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":272,"record_offset":33460,"parent_record_offset":33432,"ancestor_record_offset":9248,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":272,"record_offset":9976,"parent_record_offset":9952,"ancestor_record_offset":9248,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":272,"record_offset":18604,"parent_record_offset":18576,"ancestor_record_offset":9248,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":272,"record_offset":18292,"parent_record_offset":18256,"ancestor_record_offset":9248,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":272,"record_offset":9508,"parent_record_offset":9484,"ancestor_record_offset":9248,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":272,"record_offset":10168,"parent_record_offset":9780,"ancestor_record_offset":9248,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":10192,"parent_record_offset":10168,"ancestor_record_offset":9248,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=162
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":10284,"parent_record_offset":9568,"ancestor_record_offset":9248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":272,"record_offset":11936,"parent_record_offset":11724,"ancestor_record_offset":9248,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=44
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":272,"record_offset":10988,"parent_record_offset":10952,"ancestor_record_offset":9248,"inlinee":18717,"source_line":32,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":272,"record_offset":10256,"parent_record_offset":9568,"ancestor_record_offset":9248,"inlinee":18708,"source_line":113,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":272,"record_offset":25212,"parent_record_offset":25176,"ancestor_record_offset":9248,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":18936,"parent_record_offset":18856,"ancestor_record_offset":9248,"inlinee":18805,"source_line":145,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":18904,"parent_record_offset":18880,"ancestor_record_offset":9248,"inlinee":18804,"source_line":50,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":18880,"parent_record_offset":18856,"ancestor_record_offset":9248,"inlinee":18803,"source_line":32,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":18856,"parent_record_offset":18832,"ancestor_record_offset":9248,"inlinee":18802,"source_line":37,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":26128,"parent_record_offset":26092,"ancestor_record_offset":9248,"inlinee":18756,"source_line":333,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":272,"record_offset":10952,"parent_record_offset":10916,"ancestor_record_offset":9248,"inlinee":18811,"source_line":313,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":272,"record_offset":38540,"parent_record_offset":38504,"ancestor_record_offset":9248,"inlinee":18751,"source_line":322,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":272,"record_offset":25832,"parent_record_offset":25804,"ancestor_record_offset":9248,"inlinee":18758,"source_line":95,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::gpu_selection::device_driver_advice::impl$0::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/gpu_selection.rs
//     evidence={"module":272,"record_offset":17876,"parent_record_offset":17844,"ancestor_record_offset":9248,"inlinee":18942,"source_line":493,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::get_vendor_driver_update_links
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":272,"record_offset":56508,"rva":7094464,"va":5375803584}
// 编译实例 RVAs: 0x6c40c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-124978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375803584,"raw_hex":"e8ce17feff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375803661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7081459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375803584,"raw_hex":"e80df293ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375803646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40816891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375803584,"raw_hex":"e8fbd06e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375803898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":56656,"parent_record_offset":56508,"ancestor_record_offset":56508,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":56856,"parent_record_offset":56832,"ancestor_record_offset":56508,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":56880,"parent_record_offset":56856,"ancestor_record_offset":56508,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":56904,"parent_record_offset":56880,"ancestor_record_offset":56508,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":56832,"parent_record_offset":56804,"ancestor_record_offset":56508,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":56756,"parent_record_offset":56732,"ancestor_record_offset":56508,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":56804,"parent_record_offset":56780,"ancestor_record_offset":56508,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":56780,"parent_record_offset":56756,"ancestor_record_offset":56508,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":56680,"parent_record_offset":56656,"ancestor_record_offset":56508,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":56704,"parent_record_offset":56680,"ancestor_record_offset":56508,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":56732,"parent_record_offset":56704,"ancestor_record_offset":56508,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":272,"record_offset":56960,"parent_record_offset":56704,"ancestor_record_offset":56508,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":56984,"parent_record_offset":56960,"ancestor_record_offset":56508,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$2::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":272,"record_offset":68940,"rva":7104544,"va":5375813664}
// 编译实例 RVAs: 0x6c6820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6475087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375813664,"raw_hex":"e8b1329dff","size":5,"soft_fail":false,"target_va":5369338848,"va":5375813930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] open::that
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"open","version":"5.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9594199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375813664,"raw_hex":"e857659200","size":5,"soft_fail":false,"target_va":5385408080,"va":5375813876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7325599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375813664,"raw_hex":"e89fc76f00","size":5,"soft_fail":false,"target_va":5383139472,"va":5375813868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8907639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375813664,"raw_hex":"e877eb8700","size":5,"soft_fail":false,"target_va":5384721472,"va":5375813828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_print
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8907390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375813664,"raw_hex":"e87eea8700","size":5,"soft_fail":false,"target_va":5384721168,"va":5375813773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] webbrowser::open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"webbrowser","version":"0.8.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6170739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375813664,"raw_hex":"e873285e00","size":5,"soft_fail":false,"target_va":5381984528,"va":5375813784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] iced_runtime::command::Command::none
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":272,"record_offset":69132,"parent_record_offset":68940,"ancestor_record_offset":68940,"inlinee":18820,"source_line":30,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::window::close
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/window.rs
//     evidence={"module":272,"record_offset":69076,"parent_record_offset":68940,"ancestor_record_offset":68940,"inlinee":18813,"source_line":50,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::single
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":272,"record_offset":69100,"parent_record_offset":69076,"ancestor_record_offset":68940,"inlinee":18814,"source_line":35,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$2::view
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":272,"record_offset":58796,"rva":7094944,"va":5375804064}
// 编译实例 RVAs: 0x6c42a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_owned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8986582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8d61f8900","size":5,"soft_fail":false,"target_va":5384793264,"va":5375806677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=17
//     evidence={"assembly":"\tcallq\t-126180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e81c13feff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375804863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-131637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8cbfdfdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375810336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=17
//     target_aliases=1699 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7082661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e85bed93ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375804848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007e380
//     owner=unresolved_namespace; count=8
//     target_aliases=39 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6584925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8a3859bff","size":5,"soft_fail":false,"target_va":5369226112,"va":5375811032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=7
//     target_aliases=66 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6502260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e88cc89cff","size":5,"soft_fail":false,"target_va":5369308864,"va":5375811119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac4c0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9332268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e82c668e00","size":5,"soft_fail":false,"target_va":5385143488,"va":5375811215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe1450
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9553076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8b4c49100","size":5,"soft_fail":false,"target_va":5385360464,"va":5375807383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe39a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9563973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e845ef9100","size":5,"soft_fail":false,"target_va":5385370016,"va":5375806038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe6e10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9578298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e83a279200","size":5,"soft_fail":false,"target_va":5385383440,"va":5375805137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe9730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9587819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e86b4c9200","size":5,"soft_fail":false,"target_va":5385393968,"va":5375806144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40545292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e80cac6a02","size":5,"soft_fail":false,"target_va":5416351056,"va":5375805759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41249963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8ab6c7502","size":5,"soft_fail":false,"target_va":5417057056,"va":5375807088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t40810369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e881b76e02","size":5,"soft_fail":false,"target_va":5416620769,"va":5375810395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8990045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e85d2d8900","size":5,"soft_fail":false,"target_va":5384794864,"va":5375804814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t40813809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8f1c46e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375806980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40811082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e84aba6e02","size":5,"soft_fail":false,"target_va":5416621762,"va":5375810675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$tg_gpu_selector..gpu_selection..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t9530926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e82e6e9100","size":5,"soft_fail":false,"target_va":5385342352,"va":5375811421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..image..Image$LT$iced_core..image..Handle$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e89c980000","size":5,"soft_fail":false,"target_va":5375850768,"va":5375811695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_gpu_selector..gpu_selection..LocalizedError$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9534392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8b87b9100","size":5,"soft_fail":false,"target_va":5385345168,"va":5375810771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_core::image::Handle::from_data
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t144427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e82b340200","size":5,"soft_fail":false,"target_va":5375951136,"va":5375806704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9363334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e886df8e00","size":5,"soft_fail":false,"target_va":5385173056,"va":5375809717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::container::Container$LT$Message$C$Theme$C$Renderer$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9538967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8978d9100","size":5,"soft_fail":false,"target_va":5385348992,"va":5375810020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9365841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e851e98e00","size":5,"soft_fail":false,"target_va":5385172352,"va":5375806506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::row::Row$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9545971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8f3a89100","size":5,"soft_fail":false,"target_va":5385354384,"va":5375808408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7330504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8c8da6f00","size":5,"soft_fail":false,"target_va":5383134720,"va":5375804211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::init::AvailableGraphicsDevice::driver_support
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7646385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8b1ac7400","size":5,"soft_fail":false,"target_va":5383453280,"va":5375806890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::gpu_selection::localize_graphics_init_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8a5f9ffff","size":5,"soft_fail":false,"target_va":5375802656,"va":5375804278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_gpu_selector::gpu_selection::render_error_to_ui
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375804064,"raw_hex":"e8379affff","size":5,"soft_fail":false,"target_va":5375779392,"va":5375805444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*32(%r13)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375804064,"raw_hex":"41ff5520","size":4,"soft_fail":false,"va":5375805480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":60020,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":59288,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":59176,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":272,"record_offset":62780,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":60552,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":61908,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":272,"record_offset":59672,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":272,"record_offset":59260,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":68044,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":272,"record_offset":62004,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":272,"record_offset":59064,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":272,"record_offset":59148,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":5530,"source_line":177,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::horizontal_alignment
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":272,"record_offset":63412,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18743,"source_line":95,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":272,"record_offset":64260,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18712,"source_line":112,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":272,"record_offset":63468,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18744,"source_line":92,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":272,"record_offset":65008,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18716,"source_line":307,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::align_items
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":59640,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18754,"source_line":100,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::padding
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":61560,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18759,"source_line":76,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":60960,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18753,"source_line":70,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":66556,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18756,"source_line":333,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_x
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":272,"record_offset":67684,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18809,"source_line":121,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":272,"record_offset":60104,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18946,"source_line":50,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":272,"record_offset":60204,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18767,"source_line":79,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::width
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":272,"record_offset":67652,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18808,"source_line":85,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":272,"record_offset":63440,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18710,"source_line":116,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":272,"record_offset":59472,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18801,"source_line":68,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":272,"record_offset":67624,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18765,"source_line":57,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::image::Image::height
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/image.rs
//     evidence={"module":272,"record_offset":61880,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18956,"source_line":62,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::image::Image::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/image.rs
//     evidence={"module":272,"record_offset":61620,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18950,"source_line":45,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::image::Image::width
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/image.rs
//     evidence={"module":272,"record_offset":61848,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18955,"source_line":56,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::image::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/image.rs
//     evidence={"module":272,"record_offset":66276,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18960,"source_line":217,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":272,"record_offset":66244,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18749,"source_line":67,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":272,"record_offset":66864,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18751,"source_line":322,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::scrollable::Scrollable::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/scrollable.rs
//     evidence={"module":272,"record_offset":67232,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18807,"source_line":51,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::impl$37::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/init.rs
//     evidence={"module":272,"record_offset":62032,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18958,"source_line":537,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":272,"record_offset":62060,"parent_record_offset":58796,"ancestor_record_offset":58796,"inlinee":18939,"source_line":7,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":59372,"parent_record_offset":59348,"ancestor_record_offset":58796,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":59396,"parent_record_offset":59372,"ancestor_record_offset":58796,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":68220,"parent_record_offset":68196,"ancestor_record_offset":58796,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":68244,"parent_record_offset":68220,"ancestor_record_offset":58796,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":59420,"parent_record_offset":59396,"ancestor_record_offset":58796,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":68268,"parent_record_offset":68244,"ancestor_record_offset":58796,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":59348,"parent_record_offset":59316,"ancestor_record_offset":58796,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":68196,"parent_record_offset":68140,"ancestor_record_offset":58796,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":59316,"parent_record_offset":59288,"ancestor_record_offset":58796,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":59780,"parent_record_offset":59744,"ancestor_record_offset":58796,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":68636,"parent_record_offset":68612,"ancestor_record_offset":58796,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":59224,"parent_record_offset":59200,"ancestor_record_offset":58796,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":60652,"parent_record_offset":60620,"ancestor_record_offset":58796,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":62344,"parent_record_offset":62316,"ancestor_record_offset":58796,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":68168,"parent_record_offset":68140,"ancestor_record_offset":58796,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":68140,"parent_record_offset":68116,"ancestor_record_offset":58796,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":60700,"parent_record_offset":60676,"ancestor_record_offset":58796,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":60676,"parent_record_offset":60652,"ancestor_record_offset":58796,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":62408,"parent_record_offset":62372,"ancestor_record_offset":58796,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":62372,"parent_record_offset":62344,"ancestor_record_offset":58796,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":68116,"parent_record_offset":68092,"ancestor_record_offset":58796,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":62448,"parent_record_offset":62408,"ancestor_record_offset":58796,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":60044,"parent_record_offset":60020,"ancestor_record_offset":58796,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":62228,"parent_record_offset":62200,"ancestor_record_offset":58796,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":62256,"parent_record_offset":62228,"ancestor_record_offset":58796,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":62284,"parent_record_offset":62256,"ancestor_record_offset":58796,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":62200,"parent_record_offset":62172,"ancestor_record_offset":58796,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":272,"record_offset":62172,"parent_record_offset":62144,"ancestor_record_offset":58796,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":62116,"parent_record_offset":62088,"ancestor_record_offset":58796,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":62088,"parent_record_offset":62060,"ancestor_record_offset":58796,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":62144,"parent_record_offset":62116,"ancestor_record_offset":58796,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":60620,"parent_record_offset":60580,"ancestor_record_offset":58796,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":60068,"parent_record_offset":60044,"ancestor_record_offset":58796,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":60580,"parent_record_offset":60552,"ancestor_record_offset":58796,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":62316,"parent_record_offset":62284,"ancestor_record_offset":58796,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":272,"record_offset":62472,"parent_record_offset":62448,"ancestor_record_offset":58796,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":272,"record_offset":61648,"parent_record_offset":61620,"ancestor_record_offset":58796,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":272,"record_offset":62520,"parent_record_offset":62496,"ancestor_record_offset":58796,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":272,"record_offset":60904,"parent_record_offset":60880,"ancestor_record_offset":58796,"inlinee":15118,"source_line":116,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":272,"record_offset":59600,"parent_record_offset":59576,"ancestor_record_offset":58796,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":272,"record_offset":62496,"parent_record_offset":62472,"ancestor_record_offset":58796,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":272,"record_offset":59200,"parent_record_offset":59176,"ancestor_record_offset":58796,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":272,"record_offset":62688,"parent_record_offset":62284,"ancestor_record_offset":58796,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":62712,"parent_record_offset":62688,"ancestor_record_offset":58796,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":59976,"parent_record_offset":59780,"ancestor_record_offset":58796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":272,"record_offset":59112,"parent_record_offset":59088,"ancestor_record_offset":58796,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":60740,"parent_record_offset":60580,"ancestor_record_offset":58796,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":272,"record_offset":61936,"parent_record_offset":61908,"ancestor_record_offset":58796,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":272,"record_offset":61964,"parent_record_offset":61936,"ancestor_record_offset":58796,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":272,"record_offset":59088,"parent_record_offset":59064,"ancestor_record_offset":58796,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=12
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":272,"record_offset":59744,"parent_record_offset":59708,"ancestor_record_offset":58796,"inlinee":18717,"source_line":32,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::Handle::from_path
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":272,"record_offset":61808,"parent_record_offset":61672,"ancestor_record_offset":58796,"inlinee":18954,"source_line":19,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::image::impl$1::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/image.rs
//     evidence={"module":272,"record_offset":61672,"parent_record_offset":61648,"ancestor_record_offset":58796,"inlinee":18952,"source_line":77,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/length.rs
//     evidence={"module":272,"record_offset":60144,"parent_record_offset":60104,"ancestor_record_offset":58796,"inlinee":18948,"source_line":51,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":59576,"parent_record_offset":59496,"ancestor_record_offset":58796,"inlinee":18805,"source_line":145,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":59544,"parent_record_offset":59520,"ancestor_record_offset":58796,"inlinee":18804,"source_line":50,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":59520,"parent_record_offset":59496,"ancestor_record_offset":58796,"inlinee":18803,"source_line":32,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":59496,"parent_record_offset":59472,"ancestor_record_offset":58796,"inlinee":18802,"source_line":37,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":59708,"parent_record_offset":59672,"ancestor_record_offset":58796,"inlinee":18756,"source_line":333,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":272,"record_offset":60268,"parent_record_offset":60232,"ancestor_record_offset":58796,"inlinee":18811,"source_line":313,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":272,"record_offset":61744,"parent_record_offset":61720,"ancestor_record_offset":58796,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":272,"record_offset":61720,"parent_record_offset":61696,"ancestor_record_offset":58796,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":272,"record_offset":61768,"parent_record_offset":61744,"ancestor_record_offset":58796,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$5::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":137,"record_offset":229540,"rva":4419664,"va":5373128784}
// 编译实例 RVAs: 0x437050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11688774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373128784,"raw_hex":"e8465bb200","size":5,"soft_fail":false,"target_va":5384817664,"va":5373128885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=5
//     target_aliases=4 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t11695230","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373128784,"raw_hex":"e97e74b200","size":5,"soft_fail":false,"target_va":5384824080,"va":5373128845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$0::program_outcome
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":272,"record_offset":58664,"rva":7094896,"va":5375804016}
// 编译实例 RVAs: 0x6c4270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::Controls::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":272,"record_offset":58500,"rva":7094784,"va":5375803904}
// 编译实例 RVAs: 0x6c4200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":58632,"parent_record_offset":58500,"ancestor_record_offset":58500,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$2::view::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFC7D
// 首条位置: {"module":986,"inlinee":64637}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::String::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":4684,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::String::push_str
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":4712,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":986,"record_offset":10276,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":986,"record_offset":23300,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":986,"record_offset":11160,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":986,"record_offset":12872,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":18756,"source_line":333,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::radio
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":986,"record_offset":4500,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":64611,"source_line":173,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::radio::Radio::font
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/radio.rs
//     evidence={"module":986,"record_offset":4652,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":64638,"source_line":175,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::radio::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/radio.rs
//     evidence={"module":986,"record_offset":11748,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":64604,"source_line":364,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":986,"record_offset":13796,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":18749,"source_line":67,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":986,"record_offset":13180,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":18751,"source_line":322,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":986,"record_offset":12028,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":18758,"source_line":95,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":986,"record_offset":5556,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":18939,"source_line":7,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":4020,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":4020,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":6488,"parent_record_offset":6464,"ancestor_record_offset":4020,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":6512,"parent_record_offset":6488,"ancestor_record_offset":4020,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":4020,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":4020,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":6072,"parent_record_offset":5924,"ancestor_record_offset":4020,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":6464,"parent_record_offset":6400,"ancestor_record_offset":4020,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":986,"record_offset":10396,"parent_record_offset":10360,"ancestor_record_offset":4020,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":986,"record_offset":10360,"parent_record_offset":10332,"ancestor_record_offset":4020,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":4940,"parent_record_offset":4904,"ancestor_record_offset":4020,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":4020,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":5864,"parent_record_offset":5836,"ancestor_record_offset":4020,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":4020,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":6432,"parent_record_offset":6400,"ancestor_record_offset":4020,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":6400,"parent_record_offset":6372,"ancestor_record_offset":4020,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":5304,"parent_record_offset":5272,"ancestor_record_offset":4020,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":4020,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":4020,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":4020,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":5924,"parent_record_offset":5892,"ancestor_record_offset":4020,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":5892,"parent_record_offset":5864,"ancestor_record_offset":4020,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":6372,"parent_record_offset":6344,"ancestor_record_offset":4020,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":5960,"parent_record_offset":5924,"ancestor_record_offset":4020,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":5744,"parent_record_offset":5716,"ancestor_record_offset":4020,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":5772,"parent_record_offset":5744,"ancestor_record_offset":4020,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":5800,"parent_record_offset":5772,"ancestor_record_offset":4020,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":5716,"parent_record_offset":5688,"ancestor_record_offset":4020,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":986,"record_offset":5688,"parent_record_offset":5660,"ancestor_record_offset":4020,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$114::spec_to_string
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":5632,"parent_record_offset":5604,"ancestor_record_offset":4020,"inlinee":10264,"source_line":3043,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":6940,"parent_record_offset":6912,"ancestor_record_offset":4020,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":5604,"parent_record_offset":5556,"ancestor_record_offset":4020,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":5660,"parent_record_offset":5632,"ancestor_record_offset":4020,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$50::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":4584,"parent_record_offset":4560,"ancestor_record_offset":4020,"inlinee":15818,"source_line":3137,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":4020,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":4904,"parent_record_offset":4784,"ancestor_record_offset":4020,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":4020,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":5028,"parent_record_offset":4784,"ancestor_record_offset":4020,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":4820,"parent_record_offset":4784,"ancestor_record_offset":4020,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":5836,"parent_record_offset":5800,"ancestor_record_offset":4020,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":986,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":4020,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":986,"record_offset":5984,"parent_record_offset":5960,"ancestor_record_offset":4020,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":986,"record_offset":4560,"parent_record_offset":4528,"ancestor_record_offset":4020,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":986,"record_offset":6032,"parent_record_offset":6008,"ancestor_record_offset":4020,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":986,"record_offset":6008,"parent_record_offset":5984,"ancestor_record_offset":4020,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":986,"record_offset":17900,"parent_record_offset":17872,"ancestor_record_offset":4020,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":986,"record_offset":4616,"parent_record_offset":4528,"ancestor_record_offset":4020,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":986,"record_offset":6200,"parent_record_offset":5800,"ancestor_record_offset":4020,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":986,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":4020,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=62
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":986,"record_offset":6288,"parent_record_offset":5556,"ancestor_record_offset":4020,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":986,"record_offset":5380,"parent_record_offset":5168,"ancestor_record_offset":4020,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":986,"record_offset":10332,"parent_record_offset":10304,"ancestor_record_offset":4020,"inlinee":18717,"source_line":32,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":986,"record_offset":16164,"parent_record_offset":16128,"ancestor_record_offset":4020,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::radio::Radio::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/radio.rs
//     evidence={"module":986,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":4020,"inlinee":64610,"source_line":110,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":986,"record_offset":10304,"parent_record_offset":10276,"ancestor_record_offset":4020,"inlinee":18758,"source_line":95,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$2::view::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 256
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFC87
// 首条位置: {"module":986,"inlinee":64647}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":986,"record_offset":55252,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":57968,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":986,"record_offset":50140,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":16752,"source_line":268,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":986,"record_offset":54744,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":58984,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":986,"record_offset":54900,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":986,"record_offset":55336,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":986,"record_offset":57684,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":986,"record_offset":56788,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":986,"record_offset":55220,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18753,"source_line":70,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":986,"record_offset":57376,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18802,"source_line":37,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":986,"record_offset":59796,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18756,"source_line":333,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":986,"record_offset":54772,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18946,"source_line":50,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":986,"record_offset":54872,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18767,"source_line":79,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":986,"record_offset":59040,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18801,"source_line":68,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::radio::Radio::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/radio.rs
//     evidence={"module":986,"record_offset":60136,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":64648,"source_line":181,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::radio::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/radio.rs
//     evidence={"module":986,"record_offset":61348,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":64604,"source_line":364,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":986,"record_offset":62840,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18749,"source_line":67,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":986,"record_offset":62192,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18751,"source_line":322,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":986,"record_offset":61628,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18758,"source_line":95,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":986,"record_offset":53876,"parent_record_offset":49756,"ancestor_record_offset":49756,"inlinee":18939,"source_line":7,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":54548,"parent_record_offset":54524,"ancestor_record_offset":49756,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":54572,"parent_record_offset":54548,"ancestor_record_offset":49756,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":53764,"parent_record_offset":53740,"ancestor_record_offset":49756,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":53788,"parent_record_offset":53764,"ancestor_record_offset":49756,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":54596,"parent_record_offset":54572,"ancestor_record_offset":49756,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":53812,"parent_record_offset":53788,"ancestor_record_offset":49756,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":54524,"parent_record_offset":54364,"ancestor_record_offset":49756,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":53740,"parent_record_offset":53684,"ancestor_record_offset":49756,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":986,"record_offset":55052,"parent_record_offset":55008,"ancestor_record_offset":49756,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":986,"record_offset":55008,"parent_record_offset":54972,"ancestor_record_offset":49756,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":986,"record_offset":55300,"parent_record_offset":55276,"ancestor_record_offset":49756,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":52920,"parent_record_offset":52888,"ancestor_record_offset":49756,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":52380,"parent_record_offset":52352,"ancestor_record_offset":49756,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":54280,"parent_record_offset":54240,"ancestor_record_offset":49756,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":52464,"parent_record_offset":52440,"ancestor_record_offset":49756,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":53712,"parent_record_offset":53684,"ancestor_record_offset":49756,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":53684,"parent_record_offset":53660,"ancestor_record_offset":49756,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":52440,"parent_record_offset":52408,"ancestor_record_offset":49756,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":52968,"parent_record_offset":52944,"ancestor_record_offset":49756,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":52944,"parent_record_offset":52920,"ancestor_record_offset":49756,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":52408,"parent_record_offset":52380,"ancestor_record_offset":49756,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":54364,"parent_record_offset":54320,"ancestor_record_offset":49756,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":54320,"parent_record_offset":54280,"ancestor_record_offset":49756,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":53660,"parent_record_offset":53636,"ancestor_record_offset":49756,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":54412,"parent_record_offset":54364,"ancestor_record_offset":49756,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":54116,"parent_record_offset":54076,"ancestor_record_offset":49756,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":54156,"parent_record_offset":54116,"ancestor_record_offset":49756,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":54196,"parent_record_offset":54156,"ancestor_record_offset":49756,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":54076,"parent_record_offset":54036,"ancestor_record_offset":49756,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":986,"record_offset":54036,"parent_record_offset":53996,"ancestor_record_offset":49756,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":52064,"parent_record_offset":50140,"ancestor_record_offset":49756,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":50232,"parent_record_offset":50140,"ancestor_record_offset":49756,"inlinee":6749,"source_line":484,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$114::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":55452,"parent_record_offset":55416,"ancestor_record_offset":49756,"inlinee":10264,"source_line":3043,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":53956,"parent_record_offset":53916,"ancestor_record_offset":49756,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":53916,"parent_record_offset":53876,"ancestor_record_offset":49756,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":53996,"parent_record_offset":53956,"ancestor_record_offset":49756,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":52304,"parent_record_offset":52128,"ancestor_record_offset":49756,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":52888,"parent_record_offset":52680,"ancestor_record_offset":49756,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":52096,"parent_record_offset":52064,"ancestor_record_offset":49756,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":52552,"parent_record_offset":52304,"ancestor_record_offset":49756,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":52352,"parent_record_offset":52304,"ancestor_record_offset":49756,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":54240,"parent_record_offset":54196,"ancestor_record_offset":49756,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":986,"record_offset":52128,"parent_record_offset":52096,"ancestor_record_offset":49756,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":986,"record_offset":54436,"parent_record_offset":54412,"ancestor_record_offset":49756,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":986,"record_offset":57480,"parent_record_offset":57456,"ancestor_record_offset":49756,"inlinee":10921,"source_line":249,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":986,"record_offset":57504,"parent_record_offset":57480,"ancestor_record_offset":49756,"inlinee":10922,"source_line":201,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":986,"record_offset":57528,"parent_record_offset":57504,"ancestor_record_offset":49756,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":986,"record_offset":57576,"parent_record_offset":57552,"ancestor_record_offset":49756,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":986,"record_offset":51048,"parent_record_offset":50504,"ancestor_record_offset":49756,"inlinee":6762,"source_line":645,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":986,"record_offset":51072,"parent_record_offset":51048,"ancestor_record_offset":49756,"inlinee":6763,"source_line":1843,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":986,"record_offset":51424,"parent_record_offset":51396,"ancestor_record_offset":49756,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":986,"record_offset":54484,"parent_record_offset":54460,"ancestor_record_offset":49756,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":986,"record_offset":59168,"parent_record_offset":59144,"ancestor_record_offset":49756,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":986,"record_offset":51516,"parent_record_offset":51212,"ancestor_record_offset":49756,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":986,"record_offset":51620,"parent_record_offset":51564,"ancestor_record_offset":49756,"inlinee":10953,"source_line":217,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":986,"record_offset":51452,"parent_record_offset":51396,"ancestor_record_offset":49756,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":986,"record_offset":51396,"parent_record_offset":51368,"ancestor_record_offset":49756,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":986,"record_offset":51564,"parent_record_offset":51540,"ancestor_record_offset":49756,"inlinee":9305,"source_line":813,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":986,"record_offset":51368,"parent_record_offset":51212,"ancestor_record_offset":49756,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":986,"record_offset":51540,"parent_record_offset":51516,"ancestor_record_offset":49756,"inlinee":9304,"source_line":978,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":986,"record_offset":54460,"parent_record_offset":54436,"ancestor_record_offset":49756,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":986,"record_offset":51476,"parent_record_offset":51452,"ancestor_record_offset":49756,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":986,"record_offset":51644,"parent_record_offset":51620,"ancestor_record_offset":49756,"inlinee":10954,"source_line":1011,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":986,"record_offset":50744,"parent_record_offset":50696,"ancestor_record_offset":49756,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":986,"record_offset":57624,"parent_record_offset":57600,"ancestor_record_offset":49756,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":986,"record_offset":57552,"parent_record_offset":57528,"ancestor_record_offset":49756,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$2::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":986,"record_offset":57600,"parent_record_offset":57576,"ancestor_record_offset":49756,"inlinee":5055,"source_line":436,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":986,"record_offset":55276,"parent_record_offset":55252,"ancestor_record_offset":49756,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":986,"record_offset":54652,"parent_record_offset":54196,"ancestor_record_offset":49756,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":986,"record_offset":52260,"parent_record_offset":52236,"ancestor_record_offset":49756,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":986,"record_offset":54676,"parent_record_offset":54652,"ancestor_record_offset":49756,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":986,"record_offset":53588,"parent_record_offset":50140,"ancestor_record_offset":49756,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":986,"record_offset":52516,"parent_record_offset":52304,"ancestor_record_offset":49756,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":986,"record_offset":52236,"parent_record_offset":52212,"ancestor_record_offset":49756,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":986,"record_offset":52212,"parent_record_offset":52188,"ancestor_record_offset":49756,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":986,"record_offset":50892,"parent_record_offset":50864,"ancestor_record_offset":49756,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":986,"record_offset":51144,"parent_record_offset":51112,"ancestor_record_offset":49756,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":986,"record_offset":51172,"parent_record_offset":51144,"ancestor_record_offset":49756,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":986,"record_offset":52164,"parent_record_offset":52128,"ancestor_record_offset":49756,"inlinee":7436,"source_line":137,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::make_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":986,"record_offset":52188,"parent_record_offset":52164,"ancestor_record_offset":49756,"inlinee":7437,"source_line":89,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":986,"record_offset":50864,"parent_record_offset":50788,"ancestor_record_offset":49756,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":986,"record_offset":50696,"parent_record_offset":50668,"ancestor_record_offset":49756,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::match_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":986,"record_offset":50260,"parent_record_offset":50140,"ancestor_record_offset":49756,"inlinee":16753,"source_line":2107,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::MatchIndicesInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":986,"record_offset":50360,"parent_record_offset":50316,"ancestor_record_offset":49756,"inlinee":16758,"source_line":1059,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$91::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":986,"record_offset":50316,"parent_record_offset":50140,"ancestor_record_offset":49756,"inlinee":16756,"source_line":511,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::TwoWaySearcher::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":986,"record_offset":51112,"parent_record_offset":50404,"ancestor_record_offset":49756,"inlinee":18836,"source_line":1461,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":986,"record_offset":50284,"parent_record_offset":50260,"ancestor_record_offset":49756,"inlinee":9296,"source_line":975,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":986,"record_offset":50504,"parent_record_offset":50404,"ancestor_record_offset":49756,"inlinee":18705,"source_line":1119,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":986,"record_offset":50404,"parent_record_offset":50360,"ancestor_record_offset":49756,"inlinee":18827,"source_line":1171,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":986,"record_offset":50668,"parent_record_offset":50632,"ancestor_record_offset":49756,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":986,"record_offset":50632,"parent_record_offset":50568,"ancestor_record_offset":49756,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":986,"record_offset":50568,"parent_record_offset":50504,"ancestor_record_offset":49756,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":986,"record_offset":50788,"parent_record_offset":50568,"ancestor_record_offset":49756,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":986,"record_offset":50956,"parent_record_offset":50788,"ancestor_record_offset":49756,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":986,"record_offset":50928,"parent_record_offset":50788,"ancestor_record_offset":49756,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":986,"record_offset":54972,"parent_record_offset":54936,"ancestor_record_offset":49756,"inlinee":18717,"source_line":32,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/length.rs
//     evidence={"module":986,"record_offset":54812,"parent_record_offset":54772,"ancestor_record_offset":49756,"inlinee":18948,"source_line":51,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":986,"record_offset":57456,"parent_record_offset":57376,"ancestor_record_offset":49756,"inlinee":18805,"source_line":145,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":986,"record_offset":57424,"parent_record_offset":57400,"ancestor_record_offset":49756,"inlinee":18804,"source_line":50,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":986,"record_offset":57400,"parent_record_offset":57376,"ancestor_record_offset":49756,"inlinee":18803,"source_line":32,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":986,"record_offset":59064,"parent_record_offset":59040,"ancestor_record_offset":49756,"inlinee":18802,"source_line":37,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":986,"record_offset":54936,"parent_record_offset":54900,"ancestor_record_offset":49756,"inlinee":18811,"source_line":313,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$2::view::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 387
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFC7F
// 首条位置: {"module":986,"inlinee":64639}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":986,"record_offset":27344,"parent_record_offset":27308,"ancestor_record_offset":27308,"inlinee":18939,"source_line":7,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":27964,"parent_record_offset":27940,"ancestor_record_offset":27308,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":27988,"parent_record_offset":27964,"ancestor_record_offset":27308,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":28012,"parent_record_offset":27988,"ancestor_record_offset":27308,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":986,"record_offset":27940,"parent_record_offset":27784,"ancestor_record_offset":27308,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":27708,"parent_record_offset":27672,"ancestor_record_offset":27308,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":27784,"parent_record_offset":27744,"ancestor_record_offset":27308,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":27744,"parent_record_offset":27708,"ancestor_record_offset":27308,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":986,"record_offset":27828,"parent_record_offset":27784,"ancestor_record_offset":27308,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":27560,"parent_record_offset":27524,"ancestor_record_offset":27308,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":27596,"parent_record_offset":27560,"ancestor_record_offset":27308,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":27632,"parent_record_offset":27596,"ancestor_record_offset":27308,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":986,"record_offset":27524,"parent_record_offset":27488,"ancestor_record_offset":27308,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":986,"record_offset":27488,"parent_record_offset":27452,"ancestor_record_offset":27308,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":27416,"parent_record_offset":27380,"ancestor_record_offset":27308,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":27380,"parent_record_offset":27344,"ancestor_record_offset":27308,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":986,"record_offset":27452,"parent_record_offset":27416,"ancestor_record_offset":27308,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":986,"record_offset":27672,"parent_record_offset":27632,"ancestor_record_offset":27308,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":986,"record_offset":27852,"parent_record_offset":27828,"ancestor_record_offset":27308,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":986,"record_offset":27900,"parent_record_offset":27876,"ancestor_record_offset":27308,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":986,"record_offset":27876,"parent_record_offset":27852,"ancestor_record_offset":27308,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":986,"record_offset":28068,"parent_record_offset":27632,"ancestor_record_offset":27308,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":986,"record_offset":28092,"parent_record_offset":28068,"ancestor_record_offset":27308,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00986.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::device_driver_advice::impl$0::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 493
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x49FE
// 首条位置: {"module":272,"inlinee":18942}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":17912,"parent_record_offset":17876,"ancestor_record_offset":17876,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":272,"record_offset":17996,"parent_record_offset":17876,"ancestor_record_offset":17876,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":18024,"parent_record_offset":17876,"ancestor_record_offset":17876,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":18144,"parent_record_offset":18120,"ancestor_record_offset":17876,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":18168,"parent_record_offset":18144,"ancestor_record_offset":17876,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":18192,"parent_record_offset":18168,"ancestor_record_offset":17876,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":18120,"parent_record_offset":18096,"ancestor_record_offset":17876,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":17960,"parent_record_offset":17936,"ancestor_record_offset":17876,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":18576,"parent_record_offset":18548,"ancestor_record_offset":18320,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":18096,"parent_record_offset":18072,"ancestor_record_offset":17876,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":18072,"parent_record_offset":18048,"ancestor_record_offset":17876,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":272,"record_offset":18604,"parent_record_offset":18576,"ancestor_record_offset":18320,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":272,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":17876,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":18048,"parent_record_offset":18024,"ancestor_record_offset":17876,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::device_driver_advice::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 526
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFC6D
// 首条位置: {"module":981,"inlinee":64621}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":981,"record_offset":2580,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":981,"record_offset":3948,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":18712,"source_line":112,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":981,"record_offset":4584,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":18714,"source_line":104,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":981,"record_offset":3980,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":18713,"source_line":127,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":981,"record_offset":4612,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":18716,"source_line":307,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":981,"record_offset":3920,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":18710,"source_line":116,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":981,"record_offset":6076,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":18749,"source_line":67,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":981,"record_offset":6108,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":18751,"source_line":322,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":981,"record_offset":1984,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":18939,"source_line":7,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":981,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":1876,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":981,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":1876,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":981,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":1876,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":981,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":1876,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":981,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":1876,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":981,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":1876,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":981,"record_offset":2360,"parent_record_offset":2332,"ancestor_record_offset":1876,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":981,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":1876,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":981,"record_offset":2696,"parent_record_offset":2652,"ancestor_record_offset":1876,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":981,"record_offset":2652,"parent_record_offset":2616,"ancestor_record_offset":1876,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":981,"record_offset":4076,"parent_record_offset":4048,"ancestor_record_offset":1876,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":981,"record_offset":2272,"parent_record_offset":2244,"ancestor_record_offset":1876,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":981,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":1876,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":981,"record_offset":2332,"parent_record_offset":2300,"ancestor_record_offset":1876,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":981,"record_offset":2300,"parent_record_offset":2272,"ancestor_record_offset":1876,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":981,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":1876,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":981,"record_offset":3560,"parent_record_offset":3524,"ancestor_record_offset":1876,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":981,"record_offset":2152,"parent_record_offset":2124,"ancestor_record_offset":1876,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":981,"record_offset":2180,"parent_record_offset":2152,"ancestor_record_offset":1876,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":981,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":1876,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":981,"record_offset":2124,"parent_record_offset":2096,"ancestor_record_offset":1876,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":981,"record_offset":2096,"parent_record_offset":2068,"ancestor_record_offset":1876,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":981,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":1876,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":981,"record_offset":2012,"parent_record_offset":1984,"ancestor_record_offset":1876,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":981,"record_offset":2068,"parent_record_offset":2040,"ancestor_record_offset":1876,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":981,"record_offset":2244,"parent_record_offset":2208,"ancestor_record_offset":1876,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":981,"record_offset":4108,"parent_record_offset":4076,"ancestor_record_offset":1876,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":981,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":1876,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":981,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":1876,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":981,"record_offset":4172,"parent_record_offset":4136,"ancestor_record_offset":1876,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":981,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":1876,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":981,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":1876,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":981,"record_offset":2488,"parent_record_offset":2208,"ancestor_record_offset":1876,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":981,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":1876,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":981,"record_offset":2848,"parent_record_offset":2652,"ancestor_record_offset":1876,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":981,"record_offset":2616,"parent_record_offset":2580,"ancestor_record_offset":1876,"inlinee":18717,"source_line":32,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":981,"record_offset":3888,"parent_record_offset":3168,"ancestor_record_offset":1876,"inlinee":18708,"source_line":113,"archive":"_pdb/C/modules/00981.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::render_error_to_ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 697
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":272,"record_offset":684,"rva":7070272,"va":5375779392}
// 编译实例 RVAs: 0x6be240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-100769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e85f76feff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375779452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-100927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e8c175feff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375779626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=8
//     target_aliases=1699 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7057250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e89e5094ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375779437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007e380
//     owner=unresolved_namespace; count=2
//     target_aliases=39 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6555573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e84bf89bff","size":5,"soft_fail":false,"target_va":5369226112,"va":5375781680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=2
//     target_aliases=66 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6473042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e8ae3a9dff","size":5,"soft_fail":false,"target_va":5369308864,"va":5375781901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40570981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e865106b02","size":5,"soft_fail":false,"target_va":5416351056,"va":5375780070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41277451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e80bd87502","size":5,"soft_fail":false,"target_va":5417057056,"va":5375779600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t40840060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e87c2b6f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5375780704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9015355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e83b908900","size":5,"soft_fail":false,"target_va":5384794864,"va":5375779504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40841246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e81e306f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375779543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9391901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375779392,"raw_hex":"e81d4f8f00","size":5,"soft_fail":false,"target_va":5385172352,"va":5375780446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":3104,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":852,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":1032,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":5200,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":272,"record_offset":2476,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":272,"record_offset":1112,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":7996,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":7612,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":18753,"source_line":70,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":272,"record_offset":1140,"parent_record_offset":684,"ancestor_record_offset":684,"inlinee":18939,"source_line":7,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":684,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":684,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":684,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":684,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":684,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":684,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":912,"parent_record_offset":880,"ancestor_record_offset":684,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":2056,"parent_record_offset":1992,"ancestor_record_offset":684,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":880,"parent_record_offset":852,"ancestor_record_offset":684,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":2584,"parent_record_offset":2548,"ancestor_record_offset":684,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":272,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":684,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":684,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":5300,"parent_record_offset":5268,"ancestor_record_offset":684,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1444,"parent_record_offset":1416,"ancestor_record_offset":684,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":2024,"parent_record_offset":1992,"ancestor_record_offset":684,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":684,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":684,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":684,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1508,"parent_record_offset":1472,"ancestor_record_offset":684,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1472,"parent_record_offset":1444,"ancestor_record_offset":684,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":684,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":1552,"parent_record_offset":1508,"ancestor_record_offset":684,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":684,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":1324,"parent_record_offset":1296,"ancestor_record_offset":684,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":684,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":1380,"parent_record_offset":1352,"ancestor_record_offset":684,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":1296,"parent_record_offset":1268,"ancestor_record_offset":684,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":272,"record_offset":1268,"parent_record_offset":1240,"ancestor_record_offset":684,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$114::spec_to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":1212,"parent_record_offset":1184,"ancestor_record_offset":684,"inlinee":10264,"source_line":3043,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":1184,"parent_record_offset":1140,"ancestor_record_offset":684,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":272,"record_offset":1240,"parent_record_offset":1212,"ancestor_record_offset":684,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":5268,"parent_record_offset":5228,"ancestor_record_offset":684,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":684,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":684,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":1416,"parent_record_offset":1380,"ancestor_record_offset":684,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":272,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":684,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":272,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":684,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":272,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":684,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":272,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":684,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":272,"record_offset":1792,"parent_record_offset":1380,"ancestor_record_offset":684,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":684,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":1880,"parent_record_offset":1140,"ancestor_record_offset":684,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":5388,"parent_record_offset":5228,"ancestor_record_offset":684,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":272,"record_offset":2548,"parent_record_offset":2512,"ancestor_record_offset":684,"inlinee":18717,"source_line":32,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":272,"record_offset":2512,"parent_record_offset":2476,"ancestor_record_offset":684,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":272,"record_offset":7680,"parent_record_offset":7644,"ancestor_record_offset":684,"inlinee":18756,"source_line":333,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$3::active
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 736
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A20
// 首条位置: {"module":272,"record_offset":69264,"rva":7058320,"va":5375767440}
// 编译实例 RVAs: 0x6bb390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9429499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767440,"raw_hex":"e8fbe18f00","size":5,"soft_fail":false,"target_va":5385196960,"va":5375767456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$3::hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 744
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":272,"record_offset":69388,"rva":7058464,"va":5375767584}
// 编译实例 RVAs: 0x6bb420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9429355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767584,"raw_hex":"e86be18f00","size":5,"soft_fail":false,"target_va":5385196960,"va":5375767600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::impl$4::active
// 记录形式: compiled_procedure; instances=1
// 源码行: 757
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":272,"record_offset":560,"rva":7070192,"va":5375779312}
// 编译实例 RVAs: 0x6be1f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_gpu_selector::gpu_selection::localize_graphics_init_error
// 记录形式: compiled_procedure; instances=1
// 源码行: 778
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":272,"record_offset":53732,"rva":7093536,"va":5375802656}
// 编译实例 RVAs: 0x6c3d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-124249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375802656,"raw_hex":"e8a71afeff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375802932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-124756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375802656,"raw_hex":"e8ac18feff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375803455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7080726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375802656,"raw_hex":"e8eaf493ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375802913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gpu_selection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41254096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375802656,"raw_hex":"e8d07c7502","size":5,"soft_fail":false,"target_va":5417057056,"va":5375802955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8991661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375802656,"raw_hex":"e8ad338900","size":5,"soft_fail":false,"target_va":5384794864,"va":5375803198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40817933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375802656,"raw_hex":"e80dd56e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375802856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7332002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375802656,"raw_hex":"e8a2e06f00","size":5,"soft_fail":false,"target_va":5383134720,"va":5375802713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":55148,"parent_record_offset":53732,"ancestor_record_offset":53732,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":272,"record_offset":53892,"parent_record_offset":53732,"ancestor_record_offset":53732,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":272,"record_offset":55232,"parent_record_offset":53732,"ancestor_record_offset":53732,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":55816,"parent_record_offset":53732,"ancestor_record_offset":53732,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":272,"record_offset":54500,"parent_record_offset":53732,"ancestor_record_offset":53732,"inlinee":5530,"source_line":177,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":54320,"parent_record_offset":54296,"ancestor_record_offset":53732,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":54344,"parent_record_offset":54320,"ancestor_record_offset":53732,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":55992,"parent_record_offset":55968,"ancestor_record_offset":53732,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":56016,"parent_record_offset":55992,"ancestor_record_offset":53732,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":54368,"parent_record_offset":54344,"ancestor_record_offset":53732,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":56040,"parent_record_offset":56016,"ancestor_record_offset":53732,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":54296,"parent_record_offset":54128,"ancestor_record_offset":53732,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":272,"record_offset":55968,"parent_record_offset":55912,"ancestor_record_offset":53732,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":272,"record_offset":55196,"parent_record_offset":55172,"ancestor_record_offset":53732,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":54064,"parent_record_offset":54036,"ancestor_record_offset":53732,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":55940,"parent_record_offset":55912,"ancestor_record_offset":53732,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":55912,"parent_record_offset":55888,"ancestor_record_offset":53732,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":54128,"parent_record_offset":54092,"ancestor_record_offset":53732,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":54092,"parent_record_offset":54064,"ancestor_record_offset":53732,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":55888,"parent_record_offset":55864,"ancestor_record_offset":53732,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":272,"record_offset":54168,"parent_record_offset":54128,"ancestor_record_offset":53732,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":53948,"parent_record_offset":53920,"ancestor_record_offset":53732,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":53976,"parent_record_offset":53948,"ancestor_record_offset":53732,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":54004,"parent_record_offset":53976,"ancestor_record_offset":53732,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":272,"record_offset":53920,"parent_record_offset":53892,"ancestor_record_offset":53732,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":272,"record_offset":54036,"parent_record_offset":54004,"ancestor_record_offset":53732,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":272,"record_offset":54196,"parent_record_offset":54168,"ancestor_record_offset":53732,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":272,"record_offset":54252,"parent_record_offset":54224,"ancestor_record_offset":53732,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":272,"record_offset":54224,"parent_record_offset":54196,"ancestor_record_offset":53732,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":272,"record_offset":55172,"parent_record_offset":55148,"ancestor_record_offset":53732,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":272,"record_offset":54424,"parent_record_offset":54004,"ancestor_record_offset":53732,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":54448,"parent_record_offset":54424,"ancestor_record_offset":53732,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00272.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":272,"record_offset":55840,"parent_record_offset":55816,"ancestor_record_offset":53732,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00272.jsonl.gz"}
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
// 完整结构及原始类型记录: gpu_selection.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_gpu_selector::gpu_selection::Controls
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State<tg_gpu_selector::gpu_selection::Controls>", "constructor": "iced_runtime::program::state::State", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::program::state::State<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::program::state::State<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_gpu_selector::gpu_selection::Controls>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::new<tg_gpu_selector::gpu_selection::Controls>", "constructor": "iced_runtime::program::state::State::new", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::update<tg_gpu_selector::gpu_selection::Controls>", "constructor": "iced_runtime::program::state::State::update", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::build_user_interface<tg_gpu_selector::gpu_selection::Controls>", "constructor": "iced_runtime::program::state::build_user_interface", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls>", "constructor": "tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0", "arguments": ["tuple$<>", "tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "tuple$<enum2$<winit::event::Event<tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0", "arguments": ["tuple$<>", "tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::EventLoop::pump_events", "arguments": ["tuple$<>", "tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$1", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<tg_gpu_selector::gpu_selection::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner_sdl::run_gui_with_options::closure$0<tg_gpu_selector::gpu_selection::Controls>", "constructor": "tg_iced::gui_runner_sdl::run_gui_with_options::closure$0", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls>", "constructor": "tg_iced::gui_runner_sdl::run_gui", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::gpu_selection::Controls>", "constructor": "tg_iced::gui_runner_winit::run_gui", "arguments": ["tg_gpu_selector::gpu_selection::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_gpu_selector::gpu_selection::HyperlinkStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::pressed<tg_gpu_selector::gpu_selection::HyperlinkStyle>", "constructor": "iced_style::button::StyleSheet::pressed", "arguments": ["tg_gpu_selector::gpu_selection::HyperlinkStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::disabled<tg_gpu_selector::gpu_selection::HyperlinkStyle>", "constructor": "iced_style::button::StyleSheet::disabled", "arguments": ["tg_gpu_selector::gpu_selection::HyperlinkStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_gpu_selector::gpu_selection::LocalizedError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_gpu_selector::gpu_selection::LocalizedError>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_gpu_selector::gpu_selection::LocalizedError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_gpu_selector::gpu_selection::Message
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 123
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_gpu_selector::gpu_selection::Message>", "constructor": "enum2$", "arguments": ["tg_gpu_selector::gpu_selection::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> >", "constructor": "iced_runtime::command::action::Action", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >", "constructor": "enum2$", "arguments": ["iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::impl$0::update::closure_env$2", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::event::Status,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::event::Status", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<tg_gpu_selector::gpu_selection::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$21::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::vec::impl$21::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::event::Status,core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$21::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::event::Status", "core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$21::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::draw::recurse<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::draw::recurse", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::layout::recurse<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::layout::recurse", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::is_over::recurse<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::is_over::recurse", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::on_event::recurse<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::on_event::recurse", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::Element", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::image::impl$1::size<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::image::Handle>", "constructor": "iced_widget::image::impl$1::size", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::image::Handle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::tag<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::tag", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::size<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::size", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::size<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::size", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::size<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::size", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_widget::space::Space,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_widget::space::Space", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::Radio", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::operate<iced_widget::space::Space,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::operate", "arguments": ["iced_widget::space::Space", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_widget::space::Space,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_widget::space::Space", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_widget::space::Space,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_widget::space::Space", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::helpers::column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::helpers::column", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::helpers::column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,alloc::vec::Vec<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "iced_widget::helpers::column", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "alloc::vec::Vec<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::Row", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::Button", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::button::Button<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::button::Button<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::Column", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::Group<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::Group", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::group::Group<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::group::Group<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::Container<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::Container", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::container::Container<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::container::Container<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::Scrollable<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::Scrollable", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::scrollable::Scrollable<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::scrollable::Scrollable<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::Container::new<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::Scrollable<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::Container::new", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::Scrollable<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::Container::new<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::Container::new", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::state<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::state", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate::closure$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate::closure$1", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::on_event", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate::closure_env$1", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> >", "constructor": "iced_core::widget::operation::Operation", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > >", "constructor": "dyn$", "arguments": ["iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > >", "constructor": "ref_mut$", "arguments": ["dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > >", "constructor": "tuple$", "arguments": ["ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::container::impl$1::operate::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::container::impl$1::operate::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::container::Container<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::container::Container<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<str$>,ref$<str$> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["tuple$<ref$<str$>,ref$<str$> >", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::impl$6::fold<tuple$<ref$<str$>,ref$<str$> >,alloc::alloc::Global,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<str$>,ref$<str$> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >", "constructor": "alloc::vec::into_iter::impl$6::fold", "arguments": ["tuple$<ref$<str$>,ref$<str$> >", "alloc::alloc::Global", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<str$>,ref$<str$> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::with_children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::row::Row::with_children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::with_children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "iced_widget::row::Row::with_children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::diff<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::diff", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::overlay<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::overlay", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::try_trait::impl$2::wrap_mut_2::closure_env$0<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::ops::try_trait::impl$2::wrap_mut_2::closure_env$0", "arguments": ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::try_trait::NeverShortCircuit<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ops::try_trait::NeverShortCircuit", "arguments": ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::iter_inner::impl$8::try_fold::closure_env$0<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::ops::try_trait::impl$2::wrap_mut_2::closure_env$0<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>,core::ops::try_trait::NeverShortCircuit<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::array::iter::iter_inner::impl$8::try_fold::closure_env$0", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::ops::try_trait::impl$2::wrap_mut_2::closure_env$0<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "core::ops::try_trait::NeverShortCircuit<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::index_range::impl$1::try_fold<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::array::iter::iter_inner::impl$8::try_fold::closure_env$0<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::ops::try_trait::impl$2::wrap_mut_2::closure_env$0<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>,core::ops::try_trait::NeverShortCircuit<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >,core::ops::try_trait::NeverShortCircuit<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::ops::index_range::impl$1::try_fold", "arguments": ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::array::iter::iter_inner::impl$8::try_fold::closure_env$0<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::ops::try_trait::impl$2::wrap_mut_2::closure_env$0<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>,core::ops::try_trait::NeverShortCircuit<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "core::ops::try_trait::NeverShortCircuit<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::push<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::column::Column::push", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::diff<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::diff", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::overlay<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::overlay", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button::new<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::button::Button::new", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::diff<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::diff", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::state<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::state", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate::closure$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate::closure$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::overlay<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::overlay", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::on_event", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::button::Button<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::button::Button<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "ref_mut$", "arguments": ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$2>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::str::iter::Lines,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$2>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::notify_on_scroll<enum2$<tg_gpu_selector::gpu_selection::Message> >", "constructor": "iced_widget::scrollable::notify_on_scroll", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::diff<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::diff", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::state<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::state", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::overlay<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::overlay", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::on_event", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::image::impl$1::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::image::Handle>", "constructor": "iced_widget::image::impl$1::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::image::Handle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::image::impl$1::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::image::Handle>", "constructor": "iced_widget::image::impl$1::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::image::Handle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_widget::image::Image<iced_core::image::Handle>,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_widget::image::Image<iced_core::image::Handle>", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::image::Image<iced_core::image::Handle>,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::image::Image<iced_core::image::Handle>", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::state<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::state", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::on_event", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::radio::Radio<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::iter_inner::impl$1::partial_drop<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "core::array::iter::iter_inner::impl$1::partial_drop", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::iter_inner::impl$1::partial_drop<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "core::array::iter::iter_inner::impl$1::partial_drop", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::layout::padded::closure_env$0", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::layout::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::radio::impl$1::layout::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::radio::impl$1::layout::closure_env$1", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::next_to_each_other<iced_widget::radio::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::radio::impl$1::layout::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::layout::next_to_each_other", "arguments": ["iced_widget::radio::impl$1::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::radio::impl$1::layout::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$1", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::Tree::diff_children_custom<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::widget::tree::Tree::diff_children_custom", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::Element", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::layout::node::Node,core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::layout::node::Node", "core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::Iter", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "ref$", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::widget::tree::Tree,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::widget::tree::Tree", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::slice::iter::IterMut<iced_core::widget::tree::Tree>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::from_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::mouse_interaction<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::mouse_interaction", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::draw<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::draw", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::on_event<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::on_event", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate::closure_env$0", "arguments": ["enum2$<tg_gpu_selector::gpu_selection::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_gpu_selector::gpu_selection::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gpu_selection.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gpu_selection.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::row::Row<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::Controls::new
// 信息 ID: 8ccc926e4142179595784c82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42df92 -> 0x6c4200 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42f581 -> 0x6c4200 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::device_driver_advice
// 信息 ID: bc82f460f7d0a11bbbc36ddb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> > @ 0xfe3f81 -> 0x6bec80 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> @ 0xfea02b -> 0x6bec80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::device_driver_advice::closure$0
// 信息 ID: 49ed4e24e2b086cd0ef50938
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_gpu_selector::gpu_selection::device_driver_advice", "captures": "unknown"}
// [闭包环境类型引用] tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0 <- alloc::vec::into_iter::impl$6::fold<tuple$<ref$<str$>,ref$<str$> >,alloc::alloc::Global,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<str$>,ref$<str$> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<tuple$<ref$<str$>,ref$<str$> >,alloc::alloc::Global,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<str$>,ref$<str$> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_gpu_selector::gpu_selection::device_driver_advice::closure_env$0,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >; module=981, inline_record=1876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::device_driver_advice::impl$0::from
// 信息 ID: 72ba20e5e3f77c6839771223
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::device_driver_advice; module=272, inline_record=17876; 不是独立调用边。
// [PDB 内联宿主] tg_gpu_selector::gpu_selection::device_driver_advice; module=272, inline_record=18320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::get_vendor_driver_update_links
// 信息 ID: daa8be8916cc06814bcfd3ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$0::program_outcome
// 信息 ID: f50a33f03372a026ea7f700b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x53018c -> 0x6c4270 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x53314c -> 0x6c4270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$2::update
// 信息 ID: 39a1cf2b2b8caf2cbfd6366f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::command::Command::batch<enum2$<tg_gpu_selector::gpu_selection::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_gpu_selector::gpu_selection::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_gpu_selector::gpu_selection::Controls> > > @ 0x52e984 -> 0x6c6820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$2::view
// 信息 ID: 4f933d5eeb06a0672cda83a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::program::state::build_user_interface<tg_gpu_selector::gpu_selection::Controls> @ 0x1e1dbe -> 0x6c42a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$2::view::closure$0
// 信息 ID: b383ccd08504fefcbfd57c45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_gpu_selector::gpu_selection::impl$2::view", "captures": "unknown"}
// [闭包环境类型引用] tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::raw::init::AvailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,ref_mut$<iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> >; module=986, inline_record=4020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$2::view::closure$1
// 信息 ID: 9fe1c334754507dc7e7249d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_gpu_selector::gpu_selection::impl$2::view", "captures": "unknown"}
// [闭包环境类型引用] tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; module=986, inline_record=49756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$2::view::closure$2
// 信息 ID: 8ba4fee6312dd3aec59d4356
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 80d49587b3c5a95afabc153a: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_gpu_selector::gpu_selection::impl$2::view", "captures": "unknown"}
// [闭包环境类型引用] tg_gpu_selector::gpu_selection::impl$2::view::closure_env$2 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$2>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$2>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; module=986, inline_record=27308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$3::active
// 信息 ID: efdde8c1720ae2aab94ab42a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_style::button::StyleSheet::pressed<tg_gpu_selector::gpu_selection::HyperlinkStyle>; module=988, inline_record=444; 不是独立调用边。
// [PDB 内联宿主] iced_style::button::StyleSheet::disabled<tg_gpu_selector::gpu_selection::HyperlinkStyle>; module=988, inline_record=648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$3::hovered
// 信息 ID: 38c94c9c1fe28156fcf47819
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$4::active
// 信息 ID: 702379a8101f6b724fa44713
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::impl$5::fmt
// 信息 ID: 89d917502d76155642f5f71d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::localize_graphics_init_error
// 信息 ID: 1f7416b96ce3597e915aaef1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> @ 0xfea068 -> 0x6c3d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_gpu_selector::gpu_selection::render_error_to_ui
// 信息 ID: bb4348710fe0abb042a0c184
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cf25d5bff7c28a097953cac: iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] 96ed5be2881841498c025ae1: iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_gpu_selector::gpu_selection::impl$2::view @ 0x6c4804 -> 0x6be240 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::raw::init::UnavailableGraphicsDevice> >,tg_gpu_selector::gpu_selection::impl$2::view::closure_env$1>,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_gpu_selector::gpu_selection::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)> @ 0xfea071 -> 0x6be240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
