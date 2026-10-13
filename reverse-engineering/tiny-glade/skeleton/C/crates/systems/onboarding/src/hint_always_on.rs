// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/onboarding/src/hint_always_on.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c98f61a9baf253da19d83229d8af1f31138f93f2d3401a3b9519a30ecbdce195
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: hint_always_on.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2683, 3944, 3973, 3981
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use system_onboarding; // namespace candidate
//   package identities: [{"name":"system-onboarding","source":"local_or_path","version":"0.1.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_always_on::hint_draw_always_on
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3973,"record_offset":272,"rva":42445216,"va":5411154336}
// 编译实例 RVAs: 0x287a9a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-35476412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e844ace2fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411155111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140725200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34953543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e8b9a6eafd","size":5,"soft_fail":false,"target_va":5376201216,"va":5411154754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34914953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e8773debfd","size":5,"soft_fail":false,"target_va":5376240016,"va":5411154964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141088f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25108175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e831e180fe","size":5,"soft_fail":false,"target_va":5386047296,"va":5411155466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108a8c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25101258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e836fc80fe","size":5,"soft_fail":false,"target_va":5386053824,"va":5411155077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420bc9a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8118616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e8a81e84ff","size":5,"soft_fail":false,"target_va":5403036064,"va":5411154675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142881d50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e8f4700000","size":5,"soft_fail":false,"target_va":5411183952,"va":5411155031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5196578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e8224b4f00","size":5,"soft_fail":false,"target_va":5416351392,"va":5411154809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2670716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e87cc02800","size":5,"soft_fail":false,"target_va":5413825888,"va":5411155167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2672710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e846c82800","size":5,"soft_fail":false,"target_va":5413827904,"va":5411155189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::ActiveInputModeTracker::mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31504986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e8a6451ffe","size":5,"soft_fail":false,"target_va":5379649632,"va":5411154613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29546967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e829263dfe","size":5,"soft_fail":false,"target_va":5381607568,"va":5411154530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-29398515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e80d6a3ffe","size":5,"soft_fail":false,"target_va":5381756080,"va":5411154590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::config::TemplateView::node_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34900819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e8ad74ebfd","size":5,"soft_fail":false,"target_va":5376254160,"va":5411154974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34948419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e8bdbaeafd","size":5,"soft_fail":false,"target_va":5376206368,"va":5411154782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::ignore_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34947307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411154336,"raw_hex":"e815bfeafd","size":5,"soft_fail":false,"target_va":5376207632,"va":5411154934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3973,"record_offset":1408,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8431,"source_line":247,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3973,"record_offset":1380,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3973,"record_offset":1568,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3973,"record_offset":1012,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":3973,"record_offset":448,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3973,"record_offset":1296,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3973,"record_offset":1948,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3973,"record_offset":1200,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_display_system::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_display_system.rs
//     evidence={"module":3973,"record_offset":1920,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":134678,"source_line":27,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3973,"record_offset":1628,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":21728,"source_line":488,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3973,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3973,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3973,"record_offset":2304,"parent_record_offset":2280,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3973,"record_offset":2232,"parent_record_offset":2144,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3973,"record_offset":1788,"parent_record_offset":1756,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3973,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3973,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3973,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3973,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3973,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3973,"record_offset":1436,"parent_record_offset":1408,"ancestor_record_offset":272,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3973,"record_offset":1756,"parent_record_offset":1716,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3973,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3973,"record_offset":1692,"parent_record_offset":1652,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3973,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3973,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":272,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3973,"record_offset":1976,"parent_record_offset":1948,"ancestor_record_offset":272,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3973,"record_offset":660,"parent_record_offset":560,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3973,"record_offset":1596,"parent_record_offset":1568,"ancestor_record_offset":272,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3973,"record_offset":560,"parent_record_offset":528,"ancestor_record_offset":272,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":3973,"record_offset":1044,"parent_record_offset":1012,"ancestor_record_offset":272,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":3973,"record_offset":528,"parent_record_offset":480,"ancestor_record_offset":272,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":3973,"record_offset":480,"parent_record_offset":448,"ancestor_record_offset":272,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3973,"record_offset":1500,"parent_record_offset":1464,"ancestor_record_offset":272,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":3973,"record_offset":1076,"parent_record_offset":1044,"ancestor_record_offset":272,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":3973,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3973,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":272,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3973,"record_offset":604,"parent_record_offset":560,"ancestor_record_offset":272,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3973,"record_offset":2200,"parent_record_offset":2172,"ancestor_record_offset":272,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3973,"record_offset":1104,"parent_record_offset":1076,"ancestor_record_offset":272,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3973,"record_offset":1164,"parent_record_offset":1044,"ancestor_record_offset":272,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3973,"record_offset":2004,"parent_record_offset":1976,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3973,"record_offset":1256,"parent_record_offset":1228,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3973,"record_offset":1876,"parent_record_offset":1716,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3973,"record_offset":772,"parent_record_offset":560,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3973,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":272,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3973,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":272,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3973,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":3973,"record_offset":1464,"parent_record_offset":1436,"ancestor_record_offset":272,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3973,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":272,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/03973.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_always_on::hint_draw_always_on::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20D71
// 首条位置: {"module":3944,"inlinee":134513}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::prepare_for_display
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":3944,"record_offset":6304,"parent_record_offset":4752,"ancestor_record_offset":4752,"inlinee":134514,"source_line":232,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3944,"record_offset":4792,"parent_record_offset":4752,"ancestor_record_offset":4752,"inlinee":108016,"source_line":81,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3944,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":4752,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3944,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":4752,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3944,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":4752,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3944,"record_offset":5464,"parent_record_offset":5316,"ancestor_record_offset":4752,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$2::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":3944,"record_offset":5040,"parent_record_offset":4988,"ancestor_record_offset":4752,"inlinee":18893,"source_line":197,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3944,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":4752,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3944,"record_offset":5316,"parent_record_offset":5284,"ancestor_record_offset":4752,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3944,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":4752,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3944,"record_offset":5352,"parent_record_offset":5316,"ancestor_record_offset":4752,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3944,"record_offset":5132,"parent_record_offset":5104,"ancestor_record_offset":4752,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3944,"record_offset":5160,"parent_record_offset":5132,"ancestor_record_offset":4752,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3944,"record_offset":5188,"parent_record_offset":5160,"ancestor_record_offset":4752,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":3944,"record_offset":5104,"parent_record_offset":5076,"ancestor_record_offset":4752,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":3944,"record_offset":5076,"parent_record_offset":5040,"ancestor_record_offset":4752,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3944,"record_offset":5752,"parent_record_offset":5708,"ancestor_record_offset":4752,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3944,"record_offset":5228,"parent_record_offset":5188,"ancestor_record_offset":4752,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3944,"record_offset":5672,"parent_record_offset":4988,"ancestor_record_offset":4752,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3944,"record_offset":5708,"parent_record_offset":5672,"ancestor_record_offset":4752,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3944,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":4752,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":3944,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":4752,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3944,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":4752,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3944,"record_offset":6028,"parent_record_offset":6000,"ancestor_record_offset":4752,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3944,"record_offset":6412,"parent_record_offset":6380,"ancestor_record_offset":4752,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3944,"record_offset":5592,"parent_record_offset":5188,"ancestor_record_offset":4752,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3944,"record_offset":5616,"parent_record_offset":5592,"ancestor_record_offset":4752,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":3944,"record_offset":4920,"parent_record_offset":4792,"ancestor_record_offset":4752,"inlinee":108020,"source_line":213,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::BindingHintDisplay::cleanup
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":3944,"record_offset":6444,"parent_record_offset":6304,"ancestor_record_offset":4752,"inlinee":134517,"source_line":322,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::BindingHintDisplay::filter_to_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3944,"record_offset":6344,"parent_record_offset":6304,"ancestor_record_offset":4752,"inlinee":134515,"source_line":197,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::BindingHintDisplay::resolve_steam
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding_renderer.rs
//     evidence={"module":3944,"record_offset":6380,"parent_record_offset":6304,"ancestor_record_offset":4752,"inlinee":134516,"source_line":244,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::impl$10::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3944,"record_offset":4988,"parent_record_offset":4948,"ancestor_record_offset":4752,"inlinee":108024,"source_line":21,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3944,"record_offset":4888,"parent_record_offset":4792,"ancestor_record_offset":4752,"inlinee":108018,"source_line":31,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3944,"record_offset":4948,"parent_record_offset":4792,"ancestor_record_offset":4752,"inlinee":108022,"source_line":14,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_always_on::hint_draw_always_on::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A68C
// 首条位置: {"module":2683,"inlinee":108172}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::is_some
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3944,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":6544,"inlinee":108173,"source_line":193,"archive":"_pdb/C/modules/03944.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_always_on::hint_draw_always_on::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3981,"record_offset":4720,"rva":42466032,"va":5411175152}
// 编译实例 RVAs: 0x287faf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-26566374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e81aa16afe","size":5,"soft_fail":false,"target_va":5384609408,"va":5411175777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=3
//     target_aliases=17 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41633331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e8cdb984fd","size":5,"soft_fail":false,"target_va":5369542000,"va":5411175326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140725200
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34974251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e8d555eafd","size":5,"soft_fail":false,"target_va":5376201216,"va":5411175462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140727070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34966254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e81275eafd","size":5,"soft_fail":false,"target_va":5376209008,"va":5411175257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140727090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34966414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e87274eafd","size":5,"soft_fail":false,"target_va":5376209040,"va":5411175449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34935401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e897edeafd","size":5,"soft_fail":false,"target_va":5376240016,"va":5411175412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b13d60
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2703783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e8a7412900","size":5,"soft_fail":false,"target_va":5413879136,"va":5411175348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b13ed0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2704133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e805432900","size":5,"soft_fail":false,"target_va":5413879504,"va":5411175366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_onboarding::hint_always_on::hint_draw_always_on::closure$3::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e8c7010000","size":5,"soft_fail":false,"target_va":5411176336,"va":5411175876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_onboarding::hint_binding_renderer::render_hints
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2698887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e8872e2900","size":5,"soft_fail":false,"target_va":5413874176,"va":5411175284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34967020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e81472eafd","size":5,"soft_fail":false,"target_va":5376208368,"va":5411175383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::has_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34967386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411175152,"raw_hex":"e8a670eafd","size":5,"soft_fail":false,"target_va":5376208144,"va":5411175525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3981,"record_offset":4880,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::TemplateView::with_slot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":3981,"record_offset":4988,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":11220,"source_line":233,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":5560,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3981,"record_offset":5244,"parent_record_offset":4988,"ancestor_record_offset":4720,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3981,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":4720,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3981,"record_offset":4944,"parent_record_offset":4912,"ancestor_record_offset":4720,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3981,"record_offset":4912,"parent_record_offset":4880,"ancestor_record_offset":4720,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3981,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":4720,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":3981,"record_offset":5208,"parent_record_offset":5184,"ancestor_record_offset":4720,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3981,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":4720,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3981,"record_offset":5300,"parent_record_offset":4988,"ancestor_record_offset":4720,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3981,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":4720,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3981,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":4720,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3981,"record_offset":5160,"parent_record_offset":4988,"ancestor_record_offset":4720,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_always_on::hint_draw_always_on::closure$3::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_always_on.rs
//     evidence={"module":3981,"record_offset":5044,"parent_record_offset":4988,"ancestor_record_offset":4720,"inlinee":134710,"source_line":112,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":5412,"parent_record_offset":4988,"ancestor_record_offset":4720,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":4720,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":5128,"parent_record_offset":5072,"ancestor_record_offset":4720,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3981,"record_offset":5072,"parent_record_offset":4988,"ancestor_record_offset":4720,"inlinee":8060,"source_line":154,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_always_on::hint_draw_always_on::closure$3::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3981,"record_offset":6184,"rva":42467216,"va":5411176336}
// 编译实例 RVAs: 0x287ff90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-35497852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176336,"raw_hex":"e88458e2fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5411176551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34936873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176336,"raw_hex":"e8d7e7eafd","size":5,"soft_fail":false,"target_va":5376239520,"va":5411176388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e14d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7989985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176336,"raw_hex":"e81f1586ff","size":5,"soft_fail":false,"target_va":5403186384,"va":5411176364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142b14040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_always_on.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2703432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176336,"raw_hex":"e848402900","size":5,"soft_fail":false,"target_va":5413879872,"va":5411176435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::align
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34991344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176336,"raw_hex":"e81013eafd","size":5,"soft_fail":false,"target_va":5376185168,"va":5411176507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::color
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34991181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176336,"raw_hex":"e8b313eafd","size":5,"soft_fail":false,"target_va":5376185232,"va":5411176408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::content_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34991271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176336,"raw_hex":"e85913eafd","size":5,"soft_fail":false,"target_va":5376185152,"va":5411176418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::family
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34991199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176336,"raw_hex":"e8a113eafd","size":5,"soft_fail":false,"target_va":5376185296,"va":5411176490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34991054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176336,"raw_hex":"e83214eafd","size":5,"soft_fail":false,"target_va":5376185424,"va":5411176473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::weight
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34991171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411176336,"raw_hex":"e8bd13eafd","size":5,"soft_fail":false,"target_va":5376185360,"va":5411176526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3981,"record_offset":6412,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_state::hint_text_always_on
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_state.rs
//     evidence={"module":3981,"record_offset":6352,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":108046,"source_line":194,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3981,"record_offset":6380,"parent_record_offset":6184,"ancestor_record_offset":6184,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3981,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":6184,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3981,"record_offset":6604,"parent_record_offset":6580,"ancestor_record_offset":6184,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3981,"record_offset":6628,"parent_record_offset":6604,"ancestor_record_offset":6184,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3981,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":6184,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3981,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":6184,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3981,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":6184,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3981,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":6184,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03981.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_always_on::hint_draw_always_on::closure$3::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E36
// 首条位置: {"module":3981,"inlinee":134710}
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
// 完整结构及原始类型记录: hint_always_on.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: hint_always_on.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: hint_always_on.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_always_on::hint_draw_always_on
// 信息 ID: 2aa5dbf598f00f46caf1cb05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] df373b330135a75a20b802fd: void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>)> > @ 0x20f0a44 -> 0x287a9a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<system_onboarding::hint_state::HintDisplayState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::toolbar::ToolbarUiIsInFocus>)> @ 0x21140af -> 0x287a9a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_always_on::hint_draw_always_on::closure$0
// 信息 ID: e14c2acee2a15ec58b32908c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_onboarding::hint_always_on::hint_draw_always_on", "captures": "unknown"}
// [闭包环境类型引用] system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,ref_mut$<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,ref_mut$<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,tuple$<> > > >; module=3944, inline_record=4752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_always_on::hint_draw_always_on::closure$1
// 信息 ID: c662dbaec91a3d380e3b64e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_onboarding::hint_always_on::hint_draw_always_on", "captures": "unknown"}
// [闭包环境类型引用] system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1 <- core::iter::adapters::map::impl$2::try_fold<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,ref_mut$<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,core::slice::iter::Iter<system_onboarding::hint_state::AlwaysOnHint>,system_onboarding::hint_always_on::hint_draw_always_on::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,ref_mut$<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$1> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<system_onboarding::hint_state::HintType,enum2$<system_onboarding::hint_binding::BindingHintDisplay> >,tuple$<> > > >; module=3944, inline_record=6544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_always_on::hint_draw_always_on::closure$3
// 信息 ID: 1d13db73debdb2b500905b13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_onboarding::hint_always_on::hint_draw_always_on", "captures": "unknown"}
// [闭包环境类型引用] system_onboarding::hint_always_on::hint_draw_always_on::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_always_on::hint_draw_always_on::closure_env$3,ref$<str$> > @ 0x2881f4d -> 0x287faf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_always_on::hint_draw_always_on::closure$3::closure$0
// 信息 ID: d2b4d426822c241822678b7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_onboarding::hint_always_on::hint_draw_always_on", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_onboarding::hint_always_on::hint_draw_always_on::closure$3 @ 0x287fdc4 -> 0x287ff90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_always_on::hint_draw_always_on::closure$3::closure$1
// 信息 ID: 615a37cc6e44c3d8bab97723
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_onboarding::hint_always_on::hint_draw_always_on", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_always_on::hint_draw_always_on::closure$3; module=3981, inline_record=5044; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_always_on::hint_draw_always_on::closure$3; module=3981, inline_record=5528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
