// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/winit_setup.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1a40038391ee89bf9f5376d054546bdb097ff4a6b626ec1eabd8d8c557c75c8c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: winit_setup.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 8, 81, 170, 180, 183, 221, 228
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use winit; // namespace candidate
//   package identities: [{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::DesktopModes::capture
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B31
// 首条位置: {"module":170,"inlinee":15153}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":5724,"parent_record_offset":5588,"ancestor_record_offset":5588,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":170,"record_offset":5696,"parent_record_offset":5588,"ancestor_record_offset":5588,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":5992,"parent_record_offset":5588,"ancestor_record_offset":5588,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":170,"record_offset":5640,"parent_record_offset":5588,"ancestor_record_offset":5588,"inlinee":15154,"source_line":313,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":5588,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":5588,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":5588,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6200,"parent_record_offset":6112,"ancestor_record_offset":5588,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":170,"record_offset":5832,"parent_record_offset":5788,"ancestor_record_offset":5588,"inlinee":12111,"source_line":208,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":170,"record_offset":5788,"parent_record_offset":5756,"ancestor_record_offset":5588,"inlinee":12109,"source_line":2064,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":170,"record_offset":5888,"parent_record_offset":5788,"ancestor_record_offset":5588,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":170,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":5588,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":170,"record_offset":5756,"parent_record_offset":5724,"ancestor_record_offset":5588,"inlinee":15156,"source_line":47,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":6140,"parent_record_offset":6112,"ancestor_record_offset":5588,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":5588,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":5588,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":6168,"parent_record_offset":6140,"ancestor_record_offset":5588,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":5588,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":5588,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":5856,"parent_record_offset":5832,"ancestor_record_offset":5588,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":170,"record_offset":5664,"parent_record_offset":5640,"ancestor_record_offset":5588,"inlinee":15155,"source_line":490,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::DesktopModes::ensure
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":816,"rva":5047504,"va":5373756624}
// 编译实例 RVAs: 0x4d04d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1921220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373756624,"raw_hex":"e8c4501d00","size":5,"soft_fail":false,"target_va":5375678704,"va":5373757479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098700
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4424210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373756624,"raw_hex":"e8ee7dbcff","size":5,"soft_fail":false,"target_va":5369333504,"va":5373757709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4418017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373756624,"raw_hex":"e81f96bcff","size":5,"soft_fail":false,"target_va":5369338848,"va":5373756860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e2c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1123460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373756624,"raw_hex":"e884241100","size":5,"soft_fail":false,"target_va":5374880784,"va":5373757319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e2f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1124362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373756624,"raw_hex":"e80a281100","size":5,"soft_fail":false,"target_va":5374881552,"va":5373757185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42592854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373756624,"raw_hex":"e856ea8902","size":5,"soft_fail":false,"target_va":5416350384,"va":5373757525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1710516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373756624,"raw_hex":"e84ce6e5ff","size":5,"soft_fail":false,"target_va":5372046912,"va":5373757423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42863992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373756624,"raw_hex":"e8780d8e02","size":5,"soft_fail":false,"target_va":5416621632,"va":5373757635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::monitor::MonitorHandle::video_modes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t14288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373756624,"raw_hex":"e8d0370000","size":5,"soft_fail":false,"target_va":5373771424,"va":5373757131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2563611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373756624,"raw_hex":"e81b1e2700","size":5,"soft_fail":false,"target_va":5376320352,"va":5373756736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*52405431(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5373756624,"raw_hex":"ff15b7a41f03","size":6,"soft_fail":false,"va":5373757083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":2928,"parent_record_offset":816,"ancestor_record_offset":816,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":170,"record_offset":956,"parent_record_offset":816,"ancestor_record_offset":816,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::winit_setup::current_video_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":1144,"parent_record_offset":816,"ancestor_record_offset":816,"inlinee":15145,"source_line":45,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":816,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":816,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":816,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":816,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":170,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":816,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":170,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":816,"inlinee":4868,"source_line":1904,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::drop_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":170,"record_offset":2512,"parent_record_offset":2480,"ancestor_record_offset":816,"inlinee":4870,"source_line":1194,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":816,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":816,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":816,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":2996,"parent_record_offset":2956,"ancestor_record_offset":816,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":816,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":1056,"parent_record_offset":1036,"ancestor_record_offset":816,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":170,"record_offset":1320,"parent_record_offset":1144,"ancestor_record_offset":816,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":170,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":816,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":170,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":816,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":816,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":2044,"parent_record_offset":1996,"ancestor_record_offset":816,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":2096,"parent_record_offset":2068,"ancestor_record_offset":816,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":1888,"parent_record_offset":1848,"ancestor_record_offset":816,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":1944,"parent_record_offset":1848,"ancestor_record_offset":816,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":1848,"parent_record_offset":1144,"ancestor_record_offset":816,"inlinee":15146,"source_line":3288,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by_key::key::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":816,"inlinee":15149,"source_line":3295,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":1996,"parent_record_offset":1944,"ancestor_record_offset":816,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":170,"record_offset":2596,"parent_record_offset":2512,"ancestor_record_offset":816,"inlinee":4641,"source_line":807,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":170,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":816,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":170,"record_offset":1728,"parent_record_offset":1628,"ancestor_record_offset":816,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":170,"record_offset":2124,"parent_record_offset":2096,"ancestor_record_offset":816,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":170,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":816,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":2168,"parent_record_offset":2044,"ancestor_record_offset":816,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":816,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":2892,"parent_record_offset":1848,"ancestor_record_offset":816,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":1696,"parent_record_offset":1664,"ancestor_record_offset":816,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":816,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":170,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":816,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":3108,"parent_record_offset":2956,"ancestor_record_offset":816,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":816,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":1664,"parent_record_offset":1628,"ancestor_record_offset":816,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":1268,"parent_record_offset":1228,"ancestor_record_offset":816,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":170,"record_offset":2540,"parent_record_offset":2512,"ancestor_record_offset":816,"inlinee":4871,"source_line":684,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":170,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":816,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":170,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":816,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::winit_setup::current_video_mode::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":816,"inlinee":15151,"source_line":53,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::winit_setup::impl$0::ensure::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":1036,"parent_record_offset":956,"ancestor_record_offset":816,"inlinee":15141,"source_line":34,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":816,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":1452,"parent_record_offset":1372,"ancestor_record_offset":816,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":816,"inlinee":10979,"source_line":466,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::MonitorHandle::refresh_rate_millihertz
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":1604,"parent_record_offset":1144,"ancestor_record_offset":816,"inlinee":13013,"source_line":137,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":1196,"parent_record_offset":1144,"ancestor_record_offset":816,"inlinee":13616,"source_line":118,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::impl$14::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":816,"inlinee":15143,"source_line":102,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::refresh_rate_millihertz
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":816,"inlinee":13000,"source_line":194,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":1228,"parent_record_offset":1196,"ancestor_record_offset":816,"inlinee":13355,"source_line":185,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":816,"inlinee":13598,"source_line":87,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::impl$0::ensure::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B25
// 首条位置: {"module":170,"inlinee":15141}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":1056,"parent_record_offset":1036,"ancestor_record_offset":1036,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::impl$14::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":1036,"inlinee":15143,"source_line":102,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":1036,"inlinee":13598,"source_line":87,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::current_video_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B29
// 首条位置: {"module":170,"inlinee":15145}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":170,"record_offset":1320,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":1848,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":15146,"source_line":3288,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::refresh_rate_millihertz
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":1604,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":13013,"source_line":137,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":1196,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":13616,"source_line":118,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":1144,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":1144,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":1144,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":1144,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":170,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":1144,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":170,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":1144,"inlinee":4868,"source_line":1904,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::drop_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":170,"record_offset":2512,"parent_record_offset":2480,"ancestor_record_offset":1144,"inlinee":4870,"source_line":1194,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":170,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":1144,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":170,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":1144,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":170,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":1144,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1144,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":2044,"parent_record_offset":1996,"ancestor_record_offset":1144,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":2096,"parent_record_offset":2068,"ancestor_record_offset":1144,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":1888,"parent_record_offset":1848,"ancestor_record_offset":1144,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":1944,"parent_record_offset":1848,"ancestor_record_offset":1144,"inlinee":15147,"source_line":3322,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by_key::key::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1144,"inlinee":15149,"source_line":3295,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":1996,"parent_record_offset":1944,"ancestor_record_offset":1144,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":170,"record_offset":2596,"parent_record_offset":2512,"ancestor_record_offset":1144,"inlinee":4641,"source_line":807,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":170,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":1144,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":170,"record_offset":1728,"parent_record_offset":1628,"ancestor_record_offset":1144,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":170,"record_offset":2124,"parent_record_offset":2096,"ancestor_record_offset":1144,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":170,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":1144,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":2168,"parent_record_offset":2044,"ancestor_record_offset":1144,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":1144,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":2892,"parent_record_offset":1848,"ancestor_record_offset":1144,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":1696,"parent_record_offset":1664,"ancestor_record_offset":1144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":1144,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":1144,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":1664,"parent_record_offset":1628,"ancestor_record_offset":1144,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":1268,"parent_record_offset":1228,"ancestor_record_offset":1144,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":170,"record_offset":2540,"parent_record_offset":2512,"ancestor_record_offset":1144,"inlinee":4871,"source_line":684,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":170,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":1144,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::winit_setup::current_video_mode::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":1144,"inlinee":15151,"source_line":53,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":1144,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":1452,"parent_record_offset":1372,"ancestor_record_offset":1144,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":1144,"inlinee":10979,"source_line":466,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::refresh_rate_millihertz
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":1144,"inlinee":13000,"source_line":194,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":1228,"parent_record_offset":1196,"ancestor_record_offset":1144,"inlinee":13355,"source_line":185,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::current_video_mode::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42C1
// 首条位置: {"module":228,"inlinee":17089}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":58600,"parent_record_offset":58224,"ancestor_record_offset":58224,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::winit_setup::impl$2::from_winit
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":228,"record_offset":58252,"parent_record_offset":58224,"ancestor_record_offset":58224,"inlinee":10977,"source_line":262,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":58720,"parent_record_offset":58696,"ancestor_record_offset":58224,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":58744,"parent_record_offset":58720,"ancestor_record_offset":58224,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":58768,"parent_record_offset":58744,"ancestor_record_offset":58224,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":58696,"parent_record_offset":58672,"ancestor_record_offset":58224,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":228,"record_offset":58672,"parent_record_offset":58648,"ancestor_record_offset":58224,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":228,"record_offset":58312,"parent_record_offset":58252,"ancestor_record_offset":58224,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":228,"record_offset":58412,"parent_record_offset":58388,"ancestor_record_offset":58224,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":58624,"parent_record_offset":58600,"ancestor_record_offset":58224,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":228,"record_offset":58364,"parent_record_offset":58336,"ancestor_record_offset":58224,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":228,"record_offset":58444,"parent_record_offset":58364,"ancestor_record_offset":58224,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":228,"record_offset":58336,"parent_record_offset":58312,"ancestor_record_offset":58224,"inlinee":10979,"source_line":466,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":228,"record_offset":58284,"parent_record_offset":58252,"ancestor_record_offset":58224,"inlinee":5416,"source_line":51,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::current_video_mode::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3CD9
// 首条位置: {"module":183,"inlinee":15577}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":228,"record_offset":58160,"parent_record_offset":58136,"ancestor_record_offset":58136,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":228,"record_offset":58184,"parent_record_offset":58160,"ancestor_record_offset":58136,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::current_video_mode::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B2F
// 首条位置: {"module":170,"inlinee":15151}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":2240,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::setup_window
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":3148,"rva":5048672,"va":5373757792}
// 编译实例 RVAs: 0x4d0960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42854088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8c8e68d02","size":5,"soft_fail":false,"target_va":5416616112,"va":5373762019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42854184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e828e78d02","size":5,"soft_fail":false,"target_va":5416616304,"va":5373762115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..vec_deque..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::try_fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-765907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e82d50f4ff","size":5,"soft_fail":false,"target_va":5372993408,"va":5373759310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t10850136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8588fa500","size":5,"soft_fail":false,"target_va":5384609408,"va":5373759267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2527626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e88a912600","size":5,"soft_fail":false,"target_va":5376286320,"va":5373758689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1920167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8a74c1d00","size":5,"soft_fail":false,"target_va":5375678688,"va":5373758516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t1920229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8e54c1d00","size":5,"soft_fail":false,"target_va":5375678704,"va":5373758470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5036314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8e626b3ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5373758501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4498840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8685abbff","size":5,"soft_fail":false,"target_va":5369265120,"va":5373763955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4421016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8688abcff","size":5,"soft_fail":false,"target_va":5369338848,"va":5373759859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e470
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4405576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8b8c6bcff","size":5,"soft_fail":false,"target_va":5369357424,"va":5373762995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a28c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4387925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8ab0bbdff","size":5,"soft_fail":false,"target_va":5369374912,"va":5373762832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b3700
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4319455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e82117beff","size":5,"soft_fail":false,"target_va":5369444096,"va":5373763546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140139520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3770293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e84b78c6ff","size":5,"soft_fail":false,"target_va":5369992480,"va":5373762768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030aa20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1862695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8d993e3ff","size":5,"soft_fail":false,"target_va":5371898400,"va":5373761090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c2980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-61002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8b611ffff","size":5,"soft_fail":false,"target_va":5373700480,"va":5373761477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b7090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t942105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e819600e00","size":5,"soft_fail":false,"target_va":5374701712,"va":5373759602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140741180
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2557479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e827062700","size":5,"soft_fail":false,"target_va":5376315776,"va":5373758292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42594104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e838ef8902","size":5,"soft_fail":false,"target_va":5416356240,"va":5373762131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43298332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e81cae9402","size":5,"soft_fail":false,"target_va":5417057056,"va":5373758719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42858577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e851f88d02","size":5,"soft_fail":false,"target_va":5416620794,"va":5373762212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2527838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e85e922600","size":5,"soft_fail":false,"target_va":5376289744,"va":5373761901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42859393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e881fb8d02","size":5,"soft_fail":false,"target_va":5416621408,"va":5373762010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<image::dynimage::DynamicImage>,enum2$<image::error::ImageError> > > >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t8185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8f91f0000","size":5,"soft_fail":false,"target_va":5373769520,"va":5373761330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8bd1f0000","size":5,"soft_fail":false,"target_va":5373770080,"va":5373761950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42859526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e806fc8d02","size":5,"soft_fail":false,"target_va":5416621632,"va":5373762101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e820300000","size":5,"soft_fail":false,"target_va":5373770192,"va":5373757867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42859009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e801fa8d02","size":5,"soft_fail":false,"target_va":5416621376,"va":5373762362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9262492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e89c558d00","size":5,"soft_fail":false,"target_va":5383023680,"va":5373761183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::reader::Reader$LT$std..io..buffered..bufreader..BufReader$LT$std..fs..File$GT$$GT$::open_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9223121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8d1bb8c00","size":5,"soft_fail":false,"target_va":5382984048,"va":5373760922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10205568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e880b99b00","size":5,"soft_fail":false,"target_va":5383963792,"va":5373758219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10214075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8bbda9b00","size":5,"soft_fail":false,"target_va":5383972224,"va":5373758144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10203106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8e2af9b00","size":5,"soft_fail":false,"target_va":5383964912,"va":5373761801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10204968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e828b79b00","size":5,"soft_fail":false,"target_va":5383962816,"va":5373757843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42857661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8bdf48d02","size":5,"soft_fail":false,"target_va":5416619696,"va":5373762030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42857182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8def28d02","size":5,"soft_fail":false,"target_va":5416619360,"va":5373762173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::dpi_scaling::platform::logical_monitor_position
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t935588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8a4460e00","size":5,"soft_fail":false,"target_va":5374695552,"va":5373759959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::dpi_scaling::platform::logical_monitor_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t935222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e836450e00","size":5,"soft_fail":false,"target_va":5374695120,"va":5373759893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::winit_setup::DesktopModes::ensure
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e800f9ffff","size":5,"soft_fail":false,"target_va":5373756624,"va":5373758411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::winit_setup::find_best_fullscreen_video_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e8c50e0000","size":5,"soft_fail":false,"target_va":5373764064,"va":5373760278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::icon::Icon::from_rgba
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2557263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e84f052700","size":5,"soft_fail":false,"target_va":5376318528,"va":5373761260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2560336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e850112700","size":5,"soft_fail":false,"target_va":5376320688,"va":5373760347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2560540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e81c122700","size":5,"soft_fail":false,"target_va":5376320352,"va":5373759807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::primary_monitor
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2560768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373757792,"raw_hex":"e800132700","size":5,"soft_fail":false,"target_va":5376320336,"va":5373759563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*52401152(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5373757792,"raw_hex":"ff1500941f03","size":6,"soft_fail":false,"va":5373761674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":12028,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":10612,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":13037,"source_line":2344,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":8520,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":8492,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":9660,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":10032,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":14784,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":13048,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":170,"record_offset":13020,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":12341,"source_line":688,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":170,"record_offset":12900,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":12322,"source_line":1083,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":12084,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":11944,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":170,"record_offset":4780,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":170,"record_offset":3384,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":170,"record_offset":3768,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::dpi_scaling::platform::window_position_from_logical_monitor_coords
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/dpi_scaling.rs
//     evidence={"module":170,"record_offset":10668,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":15162,"source_line":70,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::dpi_scaling::platform::window_size_from_logical_windowed_resolution
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/dpi_scaling.rs
//     evidence={"module":170,"record_offset":10400,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":15161,"source_line":63,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::winit_setup::DesktopModes::capture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":5588,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":15153,"source_line":23,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::winit_setup::impl$2::from_winit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":11596,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":10977,"source_line":262,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":170,"record_offset":13444,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":13624,"source_line":305,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":10460,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":13617,"source_line":125,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":13500,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":13626,"source_line":508,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":7000,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":13604,"source_line":205,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_fullscreen
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":12196,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":13613,"source_line":318,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":10432,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":13610,"source_line":222,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_position
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":11204,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":13622,"source_line":274,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":7600,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":13609,"source_line":307,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":8464,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":15157,"source_line":340,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_window_icon
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":13140,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":15168,"source_line":406,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":3148,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":3148,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":3148,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":3148,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":7456,"parent_record_offset":7432,"ancestor_record_offset":3148,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":3148,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":7384,"parent_record_offset":7356,"ancestor_record_offset":3148,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":6200,"parent_record_offset":6112,"ancestor_record_offset":3148,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":170,"record_offset":12380,"parent_record_offset":12356,"ancestor_record_offset":3148,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":170,"record_offset":5832,"parent_record_offset":5788,"ancestor_record_offset":3148,"inlinee":12111,"source_line":208,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":170,"record_offset":5788,"parent_record_offset":5756,"ancestor_record_offset":3148,"inlinee":12109,"source_line":2064,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":170,"record_offset":5888,"parent_record_offset":5788,"ancestor_record_offset":3148,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":170,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":3148,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":170,"record_offset":5756,"parent_record_offset":5724,"ancestor_record_offset":3148,"inlinee":15156,"source_line":47,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":7296,"parent_record_offset":7268,"ancestor_record_offset":3148,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":6140,"parent_record_offset":6112,"ancestor_record_offset":3148,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":3148,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":7356,"parent_record_offset":7324,"ancestor_record_offset":3148,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":7324,"parent_record_offset":7296,"ancestor_record_offset":3148,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":3148,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":7176,"parent_record_offset":7148,"ancestor_record_offset":3148,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":7204,"parent_record_offset":7176,"ancestor_record_offset":3148,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":7232,"parent_record_offset":7204,"ancestor_record_offset":3148,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":7148,"parent_record_offset":7120,"ancestor_record_offset":3148,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":170,"record_offset":7120,"parent_record_offset":7084,"ancestor_record_offset":3148,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":7672,"parent_record_offset":7644,"ancestor_record_offset":3148,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":170,"record_offset":13764,"parent_record_offset":13740,"ancestor_record_offset":3148,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":170,"record_offset":13740,"parent_record_offset":13696,"ancestor_record_offset":3148,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":170,"record_offset":13300,"parent_record_offset":13272,"ancestor_record_offset":3148,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":7268,"parent_record_offset":7232,"ancestor_record_offset":3148,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":170,"record_offset":5108,"parent_record_offset":5080,"ancestor_record_offset":3148,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":170,"record_offset":5136,"parent_record_offset":5108,"ancestor_record_offset":3148,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":170,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":3148,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":170,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":3148,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":170,"record_offset":5196,"parent_record_offset":5080,"ancestor_record_offset":3148,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":170,"record_offset":5308,"parent_record_offset":5284,"ancestor_record_offset":3148,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":8676,"parent_record_offset":8552,"ancestor_record_offset":3148,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":170,"record_offset":7644,"parent_record_offset":7600,"ancestor_record_offset":3148,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$106::from
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":170,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":3148,"inlinee":12892,"source_line":79,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":170,"record_offset":11756,"parent_record_offset":11732,"ancestor_record_offset":3148,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":5724,"parent_record_offset":5588,"ancestor_record_offset":3148,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":3148,"inlinee":5091,"source_line":115,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":170,"record_offset":5696,"parent_record_offset":5588,"ancestor_record_offset":3148,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":8900,"parent_record_offset":8552,"ancestor_record_offset":3148,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":170,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":3148,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":6168,"parent_record_offset":6140,"ancestor_record_offset":3148,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":3148,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":170,"record_offset":4928,"parent_record_offset":4888,"ancestor_record_offset":3148,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":3148,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":170,"record_offset":3716,"parent_record_offset":3688,"ancestor_record_offset":3148,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":170,"record_offset":5544,"parent_record_offset":4888,"ancestor_record_offset":3148,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":170,"record_offset":7512,"parent_record_offset":7232,"ancestor_record_offset":3148,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":3148,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=62
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":5260,"parent_record_offset":5028,"ancestor_record_offset":3148,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":170,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":3148,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":170,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":3148,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":170,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3148,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":170,"record_offset":4372,"parent_record_offset":4344,"ancestor_record_offset":3148,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":5856,"parent_record_offset":5832,"ancestor_record_offset":3148,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":10516,"parent_record_offset":10484,"ancestor_record_offset":3148,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":14280,"parent_record_offset":13696,"ancestor_record_offset":3148,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":10548,"parent_record_offset":10484,"ancestor_record_offset":3148,"inlinee":8040,"source_line":1590,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":12952,"parent_record_offset":12900,"ancestor_record_offset":3148,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":12988,"parent_record_offset":12900,"ancestor_record_offset":3148,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":170,"record_offset":4472,"parent_record_offset":4444,"ancestor_record_offset":3148,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":170,"record_offset":4444,"parent_record_offset":4416,"ancestor_record_offset":3148,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":170,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":3148,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":170,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":3148,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":170,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":3148,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":170,"record_offset":4092,"parent_record_offset":3976,"ancestor_record_offset":3148,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":170,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":3148,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":170,"record_offset":3804,"parent_record_offset":3768,"ancestor_record_offset":3148,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":170,"record_offset":3944,"parent_record_offset":3912,"ancestor_record_offset":3148,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":170,"record_offset":3912,"parent_record_offset":3876,"ancestor_record_offset":3148,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":170,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":3148,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":170,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":3148,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":170,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":3148,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":170,"record_offset":4416,"parent_record_offset":3876,"ancestor_record_offset":3148,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":170,"record_offset":3488,"parent_record_offset":3456,"ancestor_record_offset":3148,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":170,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":3148,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":170,"record_offset":3612,"parent_record_offset":3580,"ancestor_record_offset":3148,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":170,"record_offset":3664,"parent_record_offset":3612,"ancestor_record_offset":3148,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":170,"record_offset":3580,"parent_record_offset":3552,"ancestor_record_offset":3148,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":170,"record_offset":3428,"parent_record_offset":3384,"ancestor_record_offset":3148,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":170,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":3148,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":170,"record_offset":3976,"parent_record_offset":3944,"ancestor_record_offset":3148,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":170,"record_offset":4260,"parent_record_offset":3976,"ancestor_record_offset":3148,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":170,"record_offset":4232,"parent_record_offset":3976,"ancestor_record_offset":3148,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":170,"record_offset":4036,"parent_record_offset":3976,"ancestor_record_offset":3148,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":14200,"parent_record_offset":14172,"ancestor_record_offset":3148,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":14580,"parent_record_offset":14392,"ancestor_record_offset":3148,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":13892,"parent_record_offset":13860,"ancestor_record_offset":3148,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":14672,"parent_record_offset":14640,"ancestor_record_offset":3148,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":13336,"parent_record_offset":13300,"ancestor_record_offset":3148,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":3148,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":13916,"parent_record_offset":13892,"ancestor_record_offset":3148,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":3148,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":3148,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":13360,"parent_record_offset":13336,"ancestor_record_offset":3148,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":14696,"parent_record_offset":14672,"ancestor_record_offset":3148,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":13392,"parent_record_offset":13300,"ancestor_record_offset":3148,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] image::io::reader::Reader::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/io/reader.rs
//     evidence={"module":170,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":3148,"inlinee":11158,"source_line":293,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":8780,"parent_record_offset":8756,"ancestor_record_offset":3148,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":8732,"parent_record_offset":8552,"ancestor_record_offset":3148,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":8756,"parent_record_offset":8732,"ancestor_record_offset":3148,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":3148,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":8592,"parent_record_offset":8552,"ancestor_record_offset":3148,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":170,"record_offset":4816,"parent_record_offset":4780,"ancestor_record_offset":3148,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":170,"record_offset":14916,"parent_record_offset":14884,"ancestor_record_offset":3148,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":170,"record_offset":15368,"parent_record_offset":15164,"ancestor_record_offset":3148,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":170,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":3148,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":170,"record_offset":5028,"parent_record_offset":4888,"ancestor_record_offset":3148,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":170,"record_offset":5232,"parent_record_offset":5028,"ancestor_record_offset":3148,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":170,"record_offset":14072,"parent_record_offset":14044,"ancestor_record_offset":3148,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":170,"record_offset":14044,"parent_record_offset":14016,"ancestor_record_offset":3148,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":170,"record_offset":14392,"parent_record_offset":14352,"ancestor_record_offset":3148,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":170,"record_offset":14172,"parent_record_offset":13980,"ancestor_record_offset":3148,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":170,"record_offset":13980,"parent_record_offset":13952,"ancestor_record_offset":3148,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":170,"record_offset":13824,"parent_record_offset":13696,"ancestor_record_offset":3148,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":170,"record_offset":13952,"parent_record_offset":13824,"ancestor_record_offset":3148,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":170,"record_offset":14352,"parent_record_offset":14320,"ancestor_record_offset":3148,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":170,"record_offset":13860,"parent_record_offset":13824,"ancestor_record_offset":3148,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":170,"record_offset":14640,"parent_record_offset":14352,"ancestor_record_offset":3148,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":170,"record_offset":4984,"parent_record_offset":4956,"ancestor_record_offset":3148,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":170,"record_offset":14016,"parent_record_offset":13980,"ancestor_record_offset":3148,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":170,"record_offset":4888,"parent_record_offset":4852,"ancestor_record_offset":3148,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":170,"record_offset":4852,"parent_record_offset":4816,"ancestor_record_offset":3148,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::winit_setup::setup_window::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":8552,"parent_record_offset":8520,"ancestor_record_offset":3148,"inlinee":15159,"source_line":74,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::winit_setup::setup_window::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":9688,"parent_record_offset":9660,"ancestor_record_offset":3148,"inlinee":15160,"source_line":88,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::LogicalPosition::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":10992,"parent_record_offset":10968,"ancestor_record_offset":3148,"inlinee":15167,"source_line":202,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":11312,"parent_record_offset":11288,"ancestor_record_offset":3148,"inlinee":12883,"source_line":281,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":3148,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$12::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":3148,"inlinee":15164,"source_line":223,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":11788,"parent_record_offset":11708,"ancestor_record_offset":3148,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":3148,"inlinee":10979,"source_line":466,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":11288,"parent_record_offset":11264,"ancestor_record_offset":3148,"inlinee":12881,"source_line":573,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$37::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":3148,"inlinee":15166,"source_line":580,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$5::from_f64
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":10796,"parent_record_offset":10716,"ancestor_record_offset":3148,"inlinee":12885,"source_line":140,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":170,"record_offset":5640,"parent_record_offset":5588,"ancestor_record_offset":3148,"inlinee":15154,"source_line":313,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":170,"record_offset":9968,"parent_record_offset":9688,"ancestor_record_offset":3148,"inlinee":13614,"source_line":329,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":11628,"parent_record_offset":11596,"ancestor_record_offset":3148,"inlinee":5416,"source_line":51,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoop::window_target
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":170,"record_offset":13468,"parent_record_offset":13444,"ancestor_record_offset":3148,"inlinee":13625,"source_line":210,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":170,"record_offset":5664,"parent_record_offset":5640,"ancestor_record_offset":3148,"inlinee":15155,"source_line":490,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":170,"record_offset":9992,"parent_record_offset":9968,"ancestor_record_offset":3148,"inlinee":13615,"source_line":494,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":10484,"parent_record_offset":10460,"ancestor_record_offset":3148,"inlinee":13354,"source_line":211,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":170,"record_offset":14740,"parent_record_offset":13696,"ancestor_record_offset":3148,"inlinee":12895,"source_line":348,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":170,"record_offset":13620,"parent_record_offset":13500,"ancestor_record_offset":3148,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":170,"record_offset":13564,"parent_record_offset":13500,"ancestor_record_offset":3148,"inlinee":13627,"source_line":100,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":170,"record_offset":13696,"parent_record_offset":13660,"ancestor_record_offset":3148,"inlinee":12814,"source_line":164,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$31::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":7036,"parent_record_offset":7000,"ancestor_record_offset":3148,"inlinee":13606,"source_line":116,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":7084,"parent_record_offset":7036,"ancestor_record_offset":3148,"inlinee":13608,"source_line":162,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":170,"record_offset":13660,"parent_record_offset":13620,"ancestor_record_offset":3148,"inlinee":13629,"source_line":514,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::setup_window::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B37
// 首条位置: {"module":170,"inlinee":15159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":8676,"parent_record_offset":8552,"ancestor_record_offset":8552,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":8900,"parent_record_offset":8552,"ancestor_record_offset":8552,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":8956,"parent_record_offset":8552,"ancestor_record_offset":8552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":8732,"parent_record_offset":8552,"ancestor_record_offset":8552,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":8592,"parent_record_offset":8552,"ancestor_record_offset":8552,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":170,"record_offset":8844,"parent_record_offset":8552,"ancestor_record_offset":8552,"inlinee":15154,"source_line":313,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":9192,"parent_record_offset":9168,"ancestor_record_offset":8552,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":9216,"parent_record_offset":9192,"ancestor_record_offset":8552,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":8552,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":9168,"parent_record_offset":9076,"ancestor_record_offset":8552,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":8552,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":8552,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":8552,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":8552,"inlinee":5091,"source_line":115,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":9136,"parent_record_offset":9104,"ancestor_record_offset":8552,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":8552,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":8980,"parent_record_offset":8956,"ancestor_record_offset":8552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":8552,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":8552,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":8780,"parent_record_offset":8756,"ancestor_record_offset":8552,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":8756,"parent_record_offset":8732,"ancestor_record_offset":8552,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":8552,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":170,"record_offset":8868,"parent_record_offset":8844,"ancestor_record_offset":8552,"inlinee":15155,"source_line":490,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::setup_window::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42C8
// 首条位置: {"module":228,"inlinee":17096}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":228,"record_offset":65072,"parent_record_offset":64940,"ancestor_record_offset":64940,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":65796,"parent_record_offset":64940,"ancestor_record_offset":64940,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::render_config::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":228,"record_offset":65236,"parent_record_offset":64940,"ancestor_record_offset":64940,"inlinee":5488,"source_line":36,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":228,"record_offset":65128,"parent_record_offset":64940,"ancestor_record_offset":64940,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":228,"record_offset":64988,"parent_record_offset":64940,"ancestor_record_offset":64940,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":66020,"parent_record_offset":65996,"ancestor_record_offset":64940,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":66044,"parent_record_offset":66020,"ancestor_record_offset":64940,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":66068,"parent_record_offset":66044,"ancestor_record_offset":64940,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":65996,"parent_record_offset":65940,"ancestor_record_offset":64940,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":65968,"parent_record_offset":65940,"ancestor_record_offset":64940,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":65940,"parent_record_offset":65916,"ancestor_record_offset":64940,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":65916,"parent_record_offset":65892,"ancestor_record_offset":64940,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":228,"record_offset":65300,"parent_record_offset":65268,"ancestor_record_offset":64940,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":228,"record_offset":65328,"parent_record_offset":65300,"ancestor_record_offset":64940,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":228,"record_offset":65428,"parent_record_offset":65236,"ancestor_record_offset":64940,"inlinee":10467,"source_line":10,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\equality.rs
//     evidence={"module":228,"record_offset":65452,"parent_record_offset":65428,"ancestor_record_offset":64940,"inlinee":10469,"source_line":153,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":65096,"parent_record_offset":65072,"ancestor_record_offset":64940,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":65268,"parent_record_offset":65236,"ancestor_record_offset":64940,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":65820,"parent_record_offset":65796,"ancestor_record_offset":64940,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":228,"record_offset":65356,"parent_record_offset":65328,"ancestor_record_offset":64940,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":228,"record_offset":65384,"parent_record_offset":65356,"ancestor_record_offset":64940,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":228,"record_offset":65012,"parent_record_offset":64988,"ancestor_record_offset":64940,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":228,"record_offset":65036,"parent_record_offset":65012,"ancestor_record_offset":64940,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":228,"record_offset":65176,"parent_record_offset":65152,"ancestor_record_offset":64940,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":228,"record_offset":65152,"parent_record_offset":65128,"ancestor_record_offset":64940,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":228,"record_offset":65200,"parent_record_offset":65176,"ancestor_record_offset":64940,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::setup_window::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B38
// 首条位置: {"module":170,"inlinee":15160}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":9800,"parent_record_offset":9688,"ancestor_record_offset":9688,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":9856,"parent_record_offset":9688,"ancestor_record_offset":9688,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":9716,"parent_record_offset":9688,"ancestor_record_offset":9688,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":170,"record_offset":9968,"parent_record_offset":9688,"ancestor_record_offset":9688,"inlinee":13614,"source_line":329,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":9824,"parent_record_offset":9800,"ancestor_record_offset":9688,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":9740,"parent_record_offset":9716,"ancestor_record_offset":9688,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":9764,"parent_record_offset":9740,"ancestor_record_offset":9688,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":9688,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":9688,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":9928,"parent_record_offset":9904,"ancestor_record_offset":9688,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":170,"record_offset":9992,"parent_record_offset":9968,"ancestor_record_offset":9688,"inlinee":13615,"source_line":494,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::find_best_fullscreen_video_mode
// 记录形式: compiled_procedure; instances=1
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":16392,"rva":5054944,"va":5373764064}
// 编译实例 RVAs: 0x4d21e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_string_lossy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11027917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8cd45a800","size":5,"soft_fail":false,"target_va":5384792224,"va":5373764302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42848581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e845d18d02","size":5,"soft_fail":false,"target_va":5416616112,"va":5373767526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42848642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e882d18d02","size":5,"soft_fail":false,"target_va":5416616304,"va":5373767657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10844000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e86077a500","size":5,"soft_fail":false,"target_va":5384609408,"va":5373765403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1914304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8c0351d00","size":5,"soft_fail":false,"target_va":5375678688,"va":5373764379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t1914264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e898351d00","size":5,"soft_fail":false,"target_va":5375678704,"va":5373764435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5042173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e80310b3ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5373764360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081b80
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4527848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e818e9baff","size":5,"soft_fail":false,"target_va":5369240448,"va":5373768291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4504242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e84e45bbff","size":5,"soft_fail":false,"target_va":5369265120,"va":5373769357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098540
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4434718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8e254bcff","size":5,"soft_fail":false,"target_va":5369333056,"va":5373767769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4426342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e89a75bcff","size":5,"soft_fail":false,"target_va":5369338848,"va":5373765185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=2
//     target_aliases=26 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4337055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e861d2bdff","size":5,"soft_fail":false,"target_va":5369431360,"va":5373768410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b47d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4320724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e82c12beff","size":5,"soft_fail":false,"target_va":5369448400,"va":5373769119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ca080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4230356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e82c73bfff","size":5,"soft_fail":false,"target_va":5369536640,"va":5373766991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d5e60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3133654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e82a2fd0ff","size":5,"soft_fail":false,"target_va":5370633824,"va":5373767473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403024e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1900302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8f200e3ff","size":5,"soft_fail":false,"target_va":5371864288,"va":5373764585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040fae0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-798371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e85dd1f3ff","size":5,"soft_fail":false,"target_va":5372967648,"va":5373766014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-283858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e82eabfbff","size":5,"soft_fail":false,"target_va":5373480896,"va":5373764749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140549560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t484550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8c6640700","size":5,"soft_fail":false,"target_va":5374252384,"va":5373767829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e1960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1110470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8c6f11000","size":5,"soft_fail":false,"target_va":5374876000,"va":5373765525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14069b420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1870916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8448c1c00","size":5,"soft_fail":false,"target_va":5375636512,"va":5373765591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d71560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42591758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e80ee68902","size":5,"soft_fail":false,"target_va":5416359264,"va":5373767501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43292143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8ef959402","size":5,"soft_fail":false,"target_va":5417057040,"va":5373764892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43292646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8e6979402","size":5,"soft_fail":false,"target_va":5417057056,"va":5373764405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42856446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8feef8d02","size":5,"soft_fail":false,"target_va":5416620794,"va":5373764343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42853806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8aee58d02","size":5,"soft_fail":false,"target_va":5416621600,"va":5373767789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42854198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e836e78d02","size":5,"soft_fail":false,"target_va":5416621632,"va":5373767429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42852147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e833df8d02","size":5,"soft_fail":false,"target_va":5416619696,"va":5373767544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::monitor::MonitorHandle::video_modes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t6917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e8051b0000","size":5,"soft_fail":false,"target_va":5373771424,"va":5373764502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2554206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e85ef92600","size":5,"soft_fail":false,"target_va":5376320688,"va":5373766477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2556204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e82c012700","size":5,"soft_fail":false,"target_va":5376320352,"va":5373764143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::decode_wide
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2555537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373764064,"raw_hex":"e891fe2600","size":5,"soft_fail":false,"target_va":5376319808,"va":5373764266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*52396195(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5373764064,"raw_hex":"ff15a3801f03","size":6,"soft_fail":false,"va":5373766319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":22780,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":11201,"source_line":596,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":19352,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":11475,"source_line":131,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":170,"record_offset":23884,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":19532,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":25352,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":170,"record_offset":26468,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":21468,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":19268,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":25408,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":9018,"source_line":3350,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":26616,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":22488,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":20264,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":170,"record_offset":24796,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":22376,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":21384,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":170,"record_offset":19560,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":6523,"source_line":1375,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":170,"record_offset":20304,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":170,"record_offset":24372,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":170,"record_offset":19804,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::winit_setup::impl$2::from_winit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":25004,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":10977,"source_line":262,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":16624,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":9750,"source_line":112,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":22216,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":13617,"source_line":125,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::refresh_rate_millihertz
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":24428,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":13013,"source_line":137,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":24676,"parent_record_offset":16392,"ancestor_record_offset":16392,"inlinee":13616,"source_line":118,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":17824,"parent_record_offset":17800,"ancestor_record_offset":16392,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":17848,"parent_record_offset":17824,"ancestor_record_offset":16392,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":18188,"parent_record_offset":18164,"ancestor_record_offset":16392,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":18212,"parent_record_offset":18188,"ancestor_record_offset":16392,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":17872,"parent_record_offset":17848,"ancestor_record_offset":16392,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":18236,"parent_record_offset":18212,"ancestor_record_offset":16392,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":17800,"parent_record_offset":17632,"ancestor_record_offset":16392,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":18164,"parent_record_offset":18140,"ancestor_record_offset":16392,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":170,"record_offset":17232,"parent_record_offset":17188,"ancestor_record_offset":16392,"inlinee":6483,"source_line":349,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":170,"record_offset":26200,"parent_record_offset":26176,"ancestor_record_offset":16392,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":16980,"parent_record_offset":16956,"ancestor_record_offset":16392,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":17544,"parent_record_offset":17504,"ancestor_record_offset":16392,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":18472,"parent_record_offset":18444,"ancestor_record_offset":16392,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":18140,"parent_record_offset":18116,"ancestor_record_offset":16392,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":17028,"parent_record_offset":17004,"ancestor_record_offset":16392,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":16392,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":17632,"parent_record_offset":17584,"ancestor_record_offset":16392,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":17584,"parent_record_offset":17544,"ancestor_record_offset":16392,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":18116,"parent_record_offset":18092,"ancestor_record_offset":16392,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":170,"record_offset":17688,"parent_record_offset":17632,"ancestor_record_offset":16392,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17380,"parent_record_offset":17340,"ancestor_record_offset":16392,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17420,"parent_record_offset":17380,"ancestor_record_offset":16392,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17460,"parent_record_offset":17420,"ancestor_record_offset":16392,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17340,"parent_record_offset":17300,"ancestor_record_offset":16392,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":19388,"parent_record_offset":19352,"ancestor_record_offset":16392,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":170,"record_offset":22804,"parent_record_offset":22780,"ancestor_record_offset":16392,"inlinee":11203,"source_line":75,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":170,"record_offset":17300,"parent_record_offset":17260,"ancestor_record_offset":16392,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$113::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":17188,"parent_record_offset":17148,"ancestor_record_offset":16392,"inlinee":9754,"source_line":3043,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":17148,"parent_record_offset":16680,"ancestor_record_offset":16392,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":17260,"parent_record_offset":17188,"ancestor_record_offset":16392,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":16956,"parent_record_offset":16932,"ancestor_record_offset":16392,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":16932,"parent_record_offset":16908,"ancestor_record_offset":16392,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":21972,"parent_record_offset":21948,"ancestor_record_offset":16392,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":21676,"parent_record_offset":21632,"ancestor_record_offset":16392,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":17504,"parent_record_offset":17460,"ancestor_record_offset":16392,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":19292,"parent_record_offset":19268,"ancestor_record_offset":16392,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":23256,"parent_record_offset":23232,"ancestor_record_offset":16392,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":16908,"parent_record_offset":16884,"ancestor_record_offset":16392,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":170,"record_offset":26676,"parent_record_offset":26652,"ancestor_record_offset":16392,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":170,"record_offset":26980,"parent_record_offset":26956,"ancestor_record_offset":16392,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":170,"record_offset":25632,"parent_record_offset":25600,"ancestor_record_offset":16392,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":170,"record_offset":21948,"parent_record_offset":21632,"ancestor_record_offset":16392,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":170,"record_offset":21596,"parent_record_offset":21560,"ancestor_record_offset":16392,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":170,"record_offset":21632,"parent_record_offset":21596,"ancestor_record_offset":16392,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::Wtf8Buf::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":170,"record_offset":16884,"parent_record_offset":16860,"ancestor_record_offset":16392,"inlinee":6907,"source_line":175,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":170,"record_offset":17712,"parent_record_offset":17688,"ancestor_record_offset":16392,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":24964,"parent_record_offset":24940,"ancestor_record_offset":16392,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":21036,"parent_record_offset":21008,"ancestor_record_offset":16392,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":170,"record_offset":25064,"parent_record_offset":25004,"ancestor_record_offset":16392,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":170,"record_offset":25164,"parent_record_offset":25140,"ancestor_record_offset":16392,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":170,"record_offset":20596,"parent_record_offset":20572,"ancestor_record_offset":16392,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":170,"record_offset":20680,"parent_record_offset":20656,"ancestor_record_offset":16392,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":170,"record_offset":19464,"parent_record_offset":19424,"ancestor_record_offset":16392,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":170,"record_offset":17760,"parent_record_offset":17736,"ancestor_record_offset":16392,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":25468,"parent_record_offset":25444,"ancestor_record_offset":16392,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":26368,"parent_record_offset":25552,"ancestor_record_offset":16392,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":170,"record_offset":25600,"parent_record_offset":25552,"ancestor_record_offset":16392,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":170,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":16392,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":21996,"parent_record_offset":21972,"ancestor_record_offset":16392,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":22020,"parent_record_offset":21996,"ancestor_record_offset":16392,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":25444,"parent_record_offset":25408,"ancestor_record_offset":16392,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":25500,"parent_record_offset":25408,"ancestor_record_offset":16392,"inlinee":9019,"source_line":3384,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by_key::key::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":25848,"parent_record_offset":25812,"ancestor_record_offset":16392,"inlinee":5406,"source_line":3357,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":170,"record_offset":25552,"parent_record_offset":25500,"ancestor_record_offset":16392,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":170,"record_offset":24580,"parent_record_offset":24556,"ancestor_record_offset":16392,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":170,"record_offset":24556,"parent_record_offset":24452,"ancestor_record_offset":16392,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":17736,"parent_record_offset":17712,"ancestor_record_offset":16392,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":22616,"parent_record_offset":22588,"ancestor_record_offset":16392,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$3::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":170,"record_offset":26040,"parent_record_offset":25880,"ancestor_record_offset":16392,"inlinee":5412,"source_line":3604,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":170,"record_offset":20828,"parent_record_offset":20800,"ancestor_record_offset":16392,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":170,"record_offset":20800,"parent_record_offset":20772,"ancestor_record_offset":16392,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":170,"record_offset":25812,"parent_record_offset":25780,"ancestor_record_offset":16392,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":21492,"parent_record_offset":21468,"ancestor_record_offset":16392,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":170,"record_offset":25780,"parent_record_offset":25600,"ancestor_record_offset":16392,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":170,"record_offset":17928,"parent_record_offset":17460,"ancestor_record_offset":16392,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":17952,"parent_record_offset":17928,"ancestor_record_offset":16392,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=45
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":18016,"parent_record_offset":16680,"ancestor_record_offset":16392,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":25376,"parent_record_offset":25352,"ancestor_record_offset":16392,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":20944,"parent_record_offset":20920,"ancestor_record_offset":16392,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":170,"record_offset":24604,"parent_record_offset":24580,"ancestor_record_offset":16392,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":170,"record_offset":25692,"parent_record_offset":25632,"ancestor_record_offset":16392,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":170,"record_offset":25720,"parent_record_offset":25632,"ancestor_record_offset":16392,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":170,"record_offset":24848,"parent_record_offset":24824,"ancestor_record_offset":16392,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":25744,"parent_record_offset":25720,"ancestor_record_offset":16392,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":24628,"parent_record_offset":24604,"ancestor_record_offset":16392,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":22276,"parent_record_offset":22240,"ancestor_record_offset":16392,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":24492,"parent_record_offset":24452,"ancestor_record_offset":16392,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":16768,"parent_record_offset":16680,"ancestor_record_offset":16392,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":170,"record_offset":22312,"parent_record_offset":22240,"ancestor_record_offset":16392,"inlinee":8040,"source_line":1590,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":170,"record_offset":21120,"parent_record_offset":21092,"ancestor_record_offset":16392,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":170,"record_offset":21152,"parent_record_offset":21120,"ancestor_record_offset":16392,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":170,"record_offset":24824,"parent_record_offset":24796,"ancestor_record_offset":16392,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":170,"record_offset":19424,"parent_record_offset":19388,"ancestor_record_offset":16392,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":170,"record_offset":21064,"parent_record_offset":21036,"ancestor_record_offset":16392,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":20184,"parent_record_offset":20156,"ancestor_record_offset":16392,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":27532,"parent_record_offset":27344,"ancestor_record_offset":16392,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":19724,"parent_record_offset":19700,"ancestor_record_offset":16392,"inlinee":4833,"source_line":1700,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":19876,"parent_record_offset":19844,"ancestor_record_offset":16392,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":27624,"parent_record_offset":27592,"ancestor_record_offset":16392,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":20088,"parent_record_offset":20056,"ancestor_record_offset":16392,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":19900,"parent_record_offset":19876,"ancestor_record_offset":16392,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":16392,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":27556,"parent_record_offset":27532,"ancestor_record_offset":16392,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":27648,"parent_record_offset":27624,"ancestor_record_offset":16392,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":170,"record_offset":20740,"parent_record_offset":20712,"ancestor_record_offset":16392,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":170,"record_offset":20772,"parent_record_offset":20740,"ancestor_record_offset":16392,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":170,"record_offset":20712,"parent_record_offset":20460,"ancestor_record_offset":16392,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":170,"record_offset":20572,"parent_record_offset":20460,"ancestor_record_offset":16392,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":170,"record_offset":21236,"parent_record_offset":20460,"ancestor_record_offset":16392,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":170,"record_offset":20656,"parent_record_offset":20460,"ancestor_record_offset":16392,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":170,"record_offset":20516,"parent_record_offset":20460,"ancestor_record_offset":16392,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":170,"record_offset":21008,"parent_record_offset":20980,"ancestor_record_offset":16392,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":170,"record_offset":20352,"parent_record_offset":20328,"ancestor_record_offset":16392,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":170,"record_offset":20980,"parent_record_offset":20872,"ancestor_record_offset":16392,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":170,"record_offset":20384,"parent_record_offset":20352,"ancestor_record_offset":16392,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":170,"record_offset":20920,"parent_record_offset":20896,"ancestor_record_offset":16392,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":170,"record_offset":21328,"parent_record_offset":20460,"ancestor_record_offset":16392,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":170,"record_offset":20896,"parent_record_offset":20872,"ancestor_record_offset":16392,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":170,"record_offset":20436,"parent_record_offset":20412,"ancestor_record_offset":16392,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":170,"record_offset":20412,"parent_record_offset":20352,"ancestor_record_offset":16392,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":170,"record_offset":20460,"parent_record_offset":20436,"ancestor_record_offset":16392,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":170,"record_offset":20544,"parent_record_offset":20460,"ancestor_record_offset":16392,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":170,"record_offset":20872,"parent_record_offset":20460,"ancestor_record_offset":16392,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":170,"record_offset":20328,"parent_record_offset":20304,"ancestor_record_offset":16392,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":170,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":16392,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":22424,"parent_record_offset":22400,"ancestor_record_offset":16392,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":22400,"parent_record_offset":22376,"ancestor_record_offset":16392,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":22448,"parent_record_offset":22424,"ancestor_record_offset":16392,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":170,"record_offset":19700,"parent_record_offset":19676,"ancestor_record_offset":16392,"inlinee":6531,"source_line":49,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":170,"record_offset":19588,"parent_record_offset":19560,"ancestor_record_offset":16392,"inlinee":6525,"source_line":1308,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":170,"record_offset":19676,"parent_record_offset":19644,"ancestor_record_offset":16392,"inlinee":6529,"source_line":963,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":170,"record_offset":19616,"parent_record_offset":19588,"ancestor_record_offset":16392,"inlinee":6527,"source_line":1115,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":170,"record_offset":19644,"parent_record_offset":19616,"ancestor_record_offset":16392,"inlinee":6528,"source_line":1152,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":17092,"parent_record_offset":16680,"ancestor_record_offset":16392,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":16836,"parent_record_offset":16812,"ancestor_record_offset":16392,"inlinee":6903,"source_line":656,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":16812,"parent_record_offset":16680,"ancestor_record_offset":16392,"inlinee":6901,"source_line":674,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":170,"record_offset":20056,"parent_record_offset":20028,"ancestor_record_offset":16392,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":170,"record_offset":20028,"parent_record_offset":20000,"ancestor_record_offset":16392,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":170,"record_offset":27344,"parent_record_offset":27308,"ancestor_record_offset":16392,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":170,"record_offset":20156,"parent_record_offset":19964,"ancestor_record_offset":16392,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":170,"record_offset":19964,"parent_record_offset":19936,"ancestor_record_offset":16392,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":170,"record_offset":19936,"parent_record_offset":19804,"ancestor_record_offset":16392,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":170,"record_offset":27308,"parent_record_offset":27280,"ancestor_record_offset":16392,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Buf::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":170,"record_offset":16860,"parent_record_offset":16836,"ancestor_record_offset":16392,"inlinee":6905,"source_line":157,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":170,"record_offset":17116,"parent_record_offset":17092,"ancestor_record_offset":16392,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":170,"record_offset":19844,"parent_record_offset":19804,"ancestor_record_offset":16392,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":170,"record_offset":27592,"parent_record_offset":27308,"ancestor_record_offset":16392,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":170,"record_offset":20000,"parent_record_offset":19964,"ancestor_record_offset":16392,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":24884,"parent_record_offset":24796,"ancestor_record_offset":16392,"inlinee":15169,"source_line":202,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":170,"record_offset":25880,"parent_record_offset":25848,"ancestor_record_offset":16392,"inlinee":5414,"source_line":219,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":25116,"parent_record_offset":25088,"ancestor_record_offset":16392,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":25196,"parent_record_offset":25116,"ancestor_record_offset":16392,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":25088,"parent_record_offset":25064,"ancestor_record_offset":16392,"inlinee":10979,"source_line":466,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$73::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":24940,"parent_record_offset":24884,"ancestor_record_offset":16392,"inlinee":12955,"source_line":410,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":24912,"parent_record_offset":24884,"ancestor_record_offset":16392,"inlinee":5416,"source_line":51,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":16680,"parent_record_offset":16624,"ancestor_record_offset":16392,"inlinee":9752,"source_line":165,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":22240,"parent_record_offset":22216,"ancestor_record_offset":16392,"inlinee":13354,"source_line":211,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::refresh_rate_millihertz
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":24452,"parent_record_offset":24428,"ancestor_record_offset":16392,"inlinee":13000,"source_line":194,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":170,"record_offset":24712,"parent_record_offset":24676,"ancestor_record_offset":16392,"inlinee":13355,"source_line":185,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B41
// 首条位置: {"module":170,"inlinee":15169}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] winit::dpi::impl$73::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":170,"record_offset":24940,"parent_record_offset":24884,"ancestor_record_offset":24884,"inlinee":12955,"source_line":410,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":170,"record_offset":24912,"parent_record_offset":24884,"ancestor_record_offset":24884,"inlinee":5416,"source_line":51,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":24964,"parent_record_offset":24940,"ancestor_record_offset":24884,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1526
// 首条位置: {"module":8,"inlinee":5414}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$3::abs
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":8,"record_offset":8996,"parent_record_offset":8864,"ancestor_record_offset":8864,"inlinee":5412,"source_line":3604,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":8,"record_offset":8908,"parent_record_offset":8864,"ancestor_record_offset":8864,"inlinee":5416,"source_line":51,"archive":"_pdb/C/modules/00008.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::impl$1::from_winit
// 记录形式: compiled_procedure; instances=1
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":221,"record_offset":58572,"rva":5992592,"va":5374701712}
// 编译实例 RVAs: 0x5b7090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_string_lossy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10090345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e869f79900","size":5,"soft_fail":false,"target_va":5384792224,"va":5374701874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t976734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e85ee70e00","size":5,"soft_fail":false,"target_va":5375678688,"va":5374701949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t976698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e83ae70e00","size":5,"soft_fail":false,"target_va":5375678704,"va":5374702001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5979744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e8a0c1a4ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374701931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5363633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e84f28aeff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374702476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: winit_setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42355076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e884498602","size":5,"soft_fail":false,"target_va":5417057056,"va":5374701975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41918875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e89ba17f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5374701914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41919437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e8cda37f02","size":5,"soft_fail":false,"target_va":5416621632,"va":5374702190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::dpi_scaling::platform::logical_monitor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e89fe6ffff","size":5,"soft_fail":false,"target_va":5374695552,"va":5374702044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::dpi_scaling::platform::logical_monitor_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e8e3e4ffff","size":5,"soft_fail":false,"target_va":5374695120,"va":5374702056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1618588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e89cb21800","size":5,"soft_fail":false,"target_va":5376320352,"va":5374701759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::decode_wide
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1617956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374701712,"raw_hex":"e824b01800","size":5,"soft_fail":false,"target_va":5376319808,"va":5374701847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":221,"record_offset":61164,"parent_record_offset":58572,"ancestor_record_offset":58572,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":221,"record_offset":58704,"parent_record_offset":58572,"ancestor_record_offset":58572,"inlinee":9750,"source_line":112,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":221,"record_offset":59720,"parent_record_offset":59696,"ancestor_record_offset":58572,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":221,"record_offset":59744,"parent_record_offset":59720,"ancestor_record_offset":58572,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":221,"record_offset":60084,"parent_record_offset":60060,"ancestor_record_offset":58572,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":221,"record_offset":60108,"parent_record_offset":60084,"ancestor_record_offset":58572,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":221,"record_offset":59768,"parent_record_offset":59744,"ancestor_record_offset":58572,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":221,"record_offset":60132,"parent_record_offset":60108,"ancestor_record_offset":58572,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":221,"record_offset":59696,"parent_record_offset":59540,"ancestor_record_offset":58572,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":221,"record_offset":60060,"parent_record_offset":60036,"ancestor_record_offset":58572,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":221,"record_offset":59244,"parent_record_offset":59212,"ancestor_record_offset":58572,"inlinee":6483,"source_line":349,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":221,"record_offset":59016,"parent_record_offset":58992,"ancestor_record_offset":58572,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":221,"record_offset":59476,"parent_record_offset":59448,"ancestor_record_offset":58572,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":221,"record_offset":60368,"parent_record_offset":60340,"ancestor_record_offset":58572,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":221,"record_offset":60036,"parent_record_offset":60012,"ancestor_record_offset":58572,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":221,"record_offset":59064,"parent_record_offset":59040,"ancestor_record_offset":58572,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":221,"record_offset":59040,"parent_record_offset":59016,"ancestor_record_offset":58572,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":221,"record_offset":59540,"parent_record_offset":59504,"ancestor_record_offset":58572,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":221,"record_offset":59504,"parent_record_offset":59476,"ancestor_record_offset":58572,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":221,"record_offset":60012,"parent_record_offset":59988,"ancestor_record_offset":58572,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":221,"record_offset":59584,"parent_record_offset":59540,"ancestor_record_offset":58572,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":221,"record_offset":59356,"parent_record_offset":59328,"ancestor_record_offset":58572,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":221,"record_offset":59384,"parent_record_offset":59356,"ancestor_record_offset":58572,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":221,"record_offset":59412,"parent_record_offset":59384,"ancestor_record_offset":58572,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":221,"record_offset":59328,"parent_record_offset":59300,"ancestor_record_offset":58572,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":221,"record_offset":59300,"parent_record_offset":59272,"ancestor_record_offset":58572,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$113::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":221,"record_offset":59212,"parent_record_offset":59184,"ancestor_record_offset":58572,"inlinee":9754,"source_line":3043,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":221,"record_offset":59184,"parent_record_offset":58736,"ancestor_record_offset":58572,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":221,"record_offset":59272,"parent_record_offset":59212,"ancestor_record_offset":58572,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":221,"record_offset":58992,"parent_record_offset":58968,"ancestor_record_offset":58572,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":221,"record_offset":58968,"parent_record_offset":58944,"ancestor_record_offset":58572,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":221,"record_offset":59448,"parent_record_offset":59412,"ancestor_record_offset":58572,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":221,"record_offset":58944,"parent_record_offset":58920,"ancestor_record_offset":58572,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::Wtf8Buf::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":221,"record_offset":58920,"parent_record_offset":58896,"ancestor_record_offset":58572,"inlinee":6907,"source_line":175,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":221,"record_offset":59608,"parent_record_offset":59584,"ancestor_record_offset":58572,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":221,"record_offset":59656,"parent_record_offset":59632,"ancestor_record_offset":58572,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":221,"record_offset":59632,"parent_record_offset":59608,"ancestor_record_offset":58572,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":221,"record_offset":59824,"parent_record_offset":59412,"ancestor_record_offset":58572,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":221,"record_offset":59848,"parent_record_offset":59824,"ancestor_record_offset":58572,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":221,"record_offset":59912,"parent_record_offset":58736,"ancestor_record_offset":58572,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":221,"record_offset":58804,"parent_record_offset":58736,"ancestor_record_offset":58572,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":221,"record_offset":59128,"parent_record_offset":58736,"ancestor_record_offset":58572,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":221,"record_offset":58872,"parent_record_offset":58848,"ancestor_record_offset":58572,"inlinee":6903,"source_line":656,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":221,"record_offset":58848,"parent_record_offset":58736,"ancestor_record_offset":58572,"inlinee":6901,"source_line":674,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Buf::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":221,"record_offset":58896,"parent_record_offset":58872,"ancestor_record_offset":58572,"inlinee":6905,"source_line":157,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":221,"record_offset":59152,"parent_record_offset":59128,"ancestor_record_offset":58572,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":221,"record_offset":58736,"parent_record_offset":58704,"ancestor_record_offset":58572,"inlinee":9752,"source_line":165,"archive":"_pdb/C/modules/00221.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_setup::impl$2::from_winit
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AE1
// 首条位置: {"module":81,"inlinee":10977}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":81,"record_offset":122172,"parent_record_offset":122116,"ancestor_record_offset":122116,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":81,"record_offset":122144,"parent_record_offset":122116,"ancestor_record_offset":122116,"inlinee":5416,"source_line":51,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":81,"record_offset":122248,"parent_record_offset":122224,"ancestor_record_offset":122116,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":81,"record_offset":122272,"parent_record_offset":122248,"ancestor_record_offset":122116,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=12
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":81,"record_offset":122224,"parent_record_offset":122196,"ancestor_record_offset":122116,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=12
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":81,"record_offset":122304,"parent_record_offset":122224,"ancestor_record_offset":122116,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":81,"record_offset":122196,"parent_record_offset":122172,"ancestor_record_offset":122116,"inlinee":10979,"source_line":466,"archive":"_pdb/C/modules/00081.jsonl.gz"}
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
// 完整结构及原始类型记录: winit_setup.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::winit_setup::DesktopModes
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: winit_setup.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: winit_setup.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 19; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >", "ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,usdl::video::DisplayMode> >,ref$<tuple$<i64,usdl::video::DisplayMode> >)
// 候选参数类型: ["ref$<tuple$<i64,usdl::video::DisplayMode> >", "ref$<tuple$<i64,usdl::video::DisplayMode> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::DesktopModes::capture
// 信息 ID: f63f8f0400fd316a7de8dc8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_setup::setup_window; module=170, inline_record=5588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::DesktopModes::ensure
// 信息 ID: a9a2852e8a7a4eae0cb1ed66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::winit_setup::setup_window @ 0x4d0bcb -> 0x4d04d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::winit_event_loop::extract_available_monitors_info @ 0x5222f3 -> 0x4d04d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::current_video_mode
// 信息 ID: 6f69dae3ac971a7858180638
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_setup::DesktopModes::ensure; module=170, inline_record=1144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::current_video_mode::closure$0
// 信息 ID: 1b05cd0c69a0440db8dcb254
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2d6daeaa00c71d16436aa0a2: core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::winit_setup::current_video_mode", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::winit_setup::current_video_mode::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::max_by_key::key::closure_env$0<country_core::render::render_config::VideoMode,tuple$<bool,u32>,tiny_glade::winit_setup::current_video_mode::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::winit_setup::current_video_mode::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::max_by_key::key::closure_env$0<country_core::render::render_config::VideoMode,tuple$<bool,u32>,tiny_glade::winit_setup::current_video_mode::closure_env$2>,tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::traits::iterator::Iterator::max_by::fold::closure_env$0<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::winit_setup::current_video_mode::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<country_core::render::render_config::VideoMode,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::render::render_config::VideoMode,ref_mut$<tiny_glade::winit_setup::current_video_mode::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<country_core::render::render_config::VideoMode,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::max_by_key::key::closure_env$0<country_core::render::render_config::VideoMode,tuple$<bool,u32>,tiny_glade::winit_setup::current_video_mode::closure_env$2>,tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::traits::iterator::Iterator::max_by::fold::closure_env$0<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)> >; module=228, inline_record=58224; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<country_core::render::render_config::VideoMode,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::render::render_config::VideoMode,ref_mut$<tiny_glade::winit_setup::current_video_mode::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<country_core::render::render_config::VideoMode,tuple$<> > > >; module=228, inline_record=61132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::current_video_mode::closure$1
// 信息 ID: 53aa883cfade141674a0234e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2d6daeaa00c71d16436aa0a2: core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::winit_setup::current_video_mode", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::winit_setup::current_video_mode::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::max_by_key::key::closure_env$0<country_core::render::render_config::VideoMode,tuple$<bool,u32>,tiny_glade::winit_setup::current_video_mode::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::winit_setup::current_video_mode::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::max_by_key::key::closure_env$0<country_core::render::render_config::VideoMode,tuple$<bool,u32>,tiny_glade::winit_setup::current_video_mode::closure_env$2>,tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::traits::iterator::Iterator::max_by::fold::closure_env$0<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::winit_setup::current_video_mode::closure_env$1 <- core::iter::adapters::map::impl$2::try_fold<country_core::render::render_config::VideoMode,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::render::render_config::VideoMode,ref_mut$<tiny_glade::winit_setup::current_video_mode::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<country_core::render::render_config::VideoMode,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::max_by_key::key::closure_env$0<country_core::render::render_config::VideoMode,tuple$<bool,u32>,tiny_glade::winit_setup::current_video_mode::closure_env$2>,tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::traits::iterator::Iterator::max_by::fold::closure_env$0<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)> >; module=228, inline_record=58136; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<country_core::render::render_config::VideoMode,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::render::render_config::VideoMode,ref_mut$<tiny_glade::winit_setup::current_video_mode::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<country_core::render::render_config::VideoMode,tuple$<> > > >; module=228, inline_record=62012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::current_video_mode::closure$2
// 信息 ID: 8976d1a441d0771efb5951d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2d6daeaa00c71d16436aa0a2: core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::winit_setup::current_video_mode", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::winit_setup::current_video_mode::closure_env$2 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::max_by_key::key::closure_env$0<country_core::render::render_config::VideoMode,tuple$<bool,u32>,tiny_glade::winit_setup::current_video_mode::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::winit_setup::current_video_mode::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::max_by_key::key::closure_env$0<country_core::render::render_config::VideoMode,tuple$<bool,u32>,tiny_glade::winit_setup::current_video_mode::closure_env$2>,tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::traits::iterator::Iterator::max_by::fold::closure_env$0<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] tiny_glade::winit_setup::DesktopModes::ensure; module=170, inline_record=2240; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::max_by_key::key::closure_env$0<country_core::render::render_config::VideoMode,tuple$<bool,u32>,tiny_glade::winit_setup::current_video_mode::closure_env$2>,tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::traits::iterator::Iterator::max_by::fold::closure_env$0<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)> >; module=228, inline_record=57864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::find_best_fullscreen_video_mode
// 信息 ID: 800cbbbe108b01328ef456e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::winit_setup::setup_window @ 0x4d1316 -> 0x4d21e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure$0
// 信息 ID: 2d45b49974cbdbf9031e786b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::winit_setup::find_best_fullscreen_video_mode", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_setup::find_best_fullscreen_video_mode; module=170, inline_record=24884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure$1
// 信息 ID: b7dd4c69bcdcaf0cbdfd46c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d96c03f858954aa9834a31e1: core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::winit_setup::find_best_fullscreen_video_mode", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<winit::monitor::VideoMode,i64,tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure_env$1 <- alloc::vec::into_iter::impl$6::fold<winit::monitor::VideoMode,alloc::alloc::Global,tuple$<i64,winit::monitor::VideoMode>,core::iter::adapters::map::map_fold::closure_env$0<winit::monitor::VideoMode,tuple$<i64,winit::monitor::VideoMode>,tuple$<i64,winit::monitor::VideoMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<winit::monitor::VideoMode,i64,tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,winit::monitor::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<winit::monitor::VideoMode,alloc::alloc::Global,tuple$<i64,winit::monitor::VideoMode>,core::iter::adapters::map::map_fold::closure_env$0<winit::monitor::VideoMode,tuple$<i64,winit::monitor::VideoMode>,tuple$<i64,winit::monitor::VideoMode>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<winit::monitor::VideoMode,i64,tiny_glade::winit_setup::find_best_fullscreen_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,winit::monitor::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)> > >; module=8, inline_record=8864; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::winit_setup::find_best_fullscreen_video_mode; module=170, inline_record=25880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::impl$0::ensure::closure$0
// 信息 ID: 85e5f3b41f963c9cd658f08e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::winit_setup::impl$0::ensure", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_setup::DesktopModes::ensure; module=170, inline_record=1036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::impl$1::from_winit
// 信息 ID: 4be89f9dad42ec3f46c2fda2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::winit_setup::setup_window @ 0x4d1072 -> 0x5b7090 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::winit_event_loop::extract_available_monitors_info @ 0x521f50 -> 0x5b7090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::winit_setup::setup_window::closure$0::closure_env$0> > > @ 0x5e3295 -> 0x5b7090 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::winit_event_loop::run_event_loop::closure_env$2> > > @ 0x5e377f -> 0x5b7090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::impl$2::from_winit
// 信息 ID: c5c9f8b16a960306b4a586f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 2d6daeaa00c71d16436aa0a2: core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// [泛型宿主/闭包/直接引用方参数候选] d79336355d1adce860522d53: country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::render::render_config::VideoMode,core::iter::adapters::map::Map<core::slice::iter::Iter<winit::monitor::VideoMode>,country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)> >; module=81, inline_record=122116; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::winit_setup::setup_window; module=170, inline_record=11596; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::winit_setup::find_best_fullscreen_video_mode; module=170, inline_record=25004; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::winit_event_loop::run_event_loop; module=180, inline_record=3400; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$1>,core::iter::traits::iterator::Iterator::max_by_key::key::closure_env$0<country_core::render::render_config::VideoMode,tuple$<bool,u32>,tiny_glade::winit_setup::current_video_mode::closure_env$2>,tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::iter::traits::iterator::Iterator::max_by::fold::closure_env$0<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)> >; module=228, inline_record=58252; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<country_core::render::render_config::VideoMode,core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::collections::btree::set::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,winit::platform_impl::platform::monitor::impl$7::video_modes::closure_env$0>,winit::monitor::impl$5::video_modes::closure_env$0>,tiny_glade::winit_setup::current_video_mode::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::render::render_config::VideoMode,ref_mut$<tiny_glade::winit_setup::current_video_mode::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<country_core::render::render_config::VideoMode,tuple$<> > > >; module=228, inline_record=61160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::setup_window
// 信息 ID: 5f09617766b78562d714ce27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42bb8e -> 0x4d0960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::setup_window::closure$0
// 信息 ID: 30bf3c56b86ed2c0524612f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::winit_setup::setup_window", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_setup::setup_window; module=170, inline_record=8552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::setup_window::closure$0::closure$0
// 信息 ID: 277130d922ff98d8aafbe582
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::winit_setup::setup_window", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::winit_setup::setup_window::closure$0::closure_env$0 <- alloc::collections::vec_deque::into_iter::impl$2::try_fold<winit::platform_impl::platform::monitor::MonitorHandle,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::winit_setup::setup_window::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::winit_setup::setup_window::closure$0::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::winit_setup::setup_window::closure$0::closure_env$0> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::winit_setup::setup_window::closure$0::closure_env$0> > >; module=228, inline_record=64940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_setup::setup_window::closure$1
// 信息 ID: 0b8467cac4c64c20f94d1348
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::winit_setup::setup_window", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_setup::setup_window; module=170, inline_record=9688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
