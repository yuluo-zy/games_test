// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/glade-border/src/display_glade_border.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ee111517270dabc4017d8aacce027614d04aec8382b4c2c00c87d6ccf1d292eb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: display_glade_border.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2616, 2624
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
// use system_glade_border; // namespace candidate
//   package identities: [{"name":"system-glade-border","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2616,"record_offset":276,"rva":34240272,"va":5402949392}
// 编译实例 RVAs: 0x20a7710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-27271221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e8cbdf5ffe","size":5,"soft_fail":false,"target_va":5375678688,"va":5402949904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-27270845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e843e15ffe","size":5,"soft_fail":false,"target_va":5375678704,"va":5402949544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34227702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e80abaf5fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5402949889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33709946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e886a0fdfd","size":5,"soft_fail":false,"target_va":5369240528,"va":5402950469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ab9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e82f420000","size":5,"soft_fail":false,"target_va":5402966464,"va":5402949516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13400390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e84679cc00","size":5,"soft_fail":false,"target_va":5416350720,"va":5402950325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14106930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e83241d700","size":5,"soft_fail":false,"target_va":5417057056,"va":5402950121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13670368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e8e097d000","size":5,"soft_fail":false,"target_va":5416620769,"va":5402950396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18154996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e80cfaeafe","size":5,"soft_fail":false,"target_va":5384794864,"va":5402949855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13671012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e8649ad000","size":5,"soft_fail":false,"target_va":5416620794,"va":5402949777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24014240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e8609291fe","size":5,"soft_fail":false,"target_va":5378935744,"va":5402949979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22738053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e87b0ba5fe","size":5,"soft_fail":false,"target_va":5380211968,"va":5402950016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23352092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e8e4ac9bfe","size":5,"soft_fail":false,"target_va":5379597392,"va":5402949479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23352264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402949392,"raw_hex":"e838ac9bfe","size":5,"soft_fail":false,"target_va":5379597536,"va":5402949795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2616,"record_offset":2916,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":2616,"record_offset":3028,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":2292,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2616,"record_offset":1800,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":2616,"record_offset":1236,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":2616,"record_offset":4472,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":2616,"record_offset":3000,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2616,"record_offset":544,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2616,"record_offset":628,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2616,"record_offset":460,"parent_record_offset":276,"ancestor_record_offset":276,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":276,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":276,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":836,"parent_record_offset":812,"ancestor_record_offset":276,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":276,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":276,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":276,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":2720,"parent_record_offset":2536,"ancestor_record_offset":276,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":812,"parent_record_offset":724,"ancestor_record_offset":276,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2616,"record_offset":3108,"parent_record_offset":3064,"ancestor_record_offset":276,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2616,"record_offset":3064,"parent_record_offset":3028,"ancestor_record_offset":276,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2616,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":276,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":276,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":2472,"parent_record_offset":2444,"ancestor_record_offset":276,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":752,"parent_record_offset":724,"ancestor_record_offset":276,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":276,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":4732,"parent_record_offset":4708,"ancestor_record_offset":276,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":276,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":2536,"parent_record_offset":2500,"ancestor_record_offset":276,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":2500,"parent_record_offset":2472,"ancestor_record_offset":276,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":276,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":2576,"parent_record_offset":2536,"ancestor_record_offset":276,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2616,"record_offset":2400,"parent_record_offset":2328,"ancestor_record_offset":276,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":4660,"parent_record_offset":4624,"ancestor_record_offset":276,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":276,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":4596,"parent_record_offset":4560,"ancestor_record_offset":276,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":4624,"parent_record_offset":4596,"ancestor_record_offset":276,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":2444,"parent_record_offset":2400,"ancestor_record_offset":276,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":568,"parent_record_offset":544,"ancestor_record_offset":276,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":1448,"parent_record_offset":1348,"ancestor_record_offset":276,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2616,"record_offset":4532,"parent_record_offset":4496,"ancestor_record_offset":276,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2616,"record_offset":4496,"parent_record_offset":4472,"ancestor_record_offset":276,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2616,"record_offset":1348,"parent_record_offset":1316,"ancestor_record_offset":276,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":2616,"record_offset":1836,"parent_record_offset":1800,"ancestor_record_offset":276,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":2616,"record_offset":1316,"parent_record_offset":1268,"ancestor_record_offset":276,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":2616,"record_offset":4560,"parent_record_offset":4496,"ancestor_record_offset":276,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":2616,"record_offset":1268,"parent_record_offset":1236,"ancestor_record_offset":276,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2616,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":276,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2616,"record_offset":2680,"parent_record_offset":2624,"ancestor_record_offset":276,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":2616,"record_offset":1876,"parent_record_offset":1836,"ancestor_record_offset":276,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2616,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":276,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2616,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":276,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2616,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":276,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2616,"record_offset":2652,"parent_record_offset":2624,"ancestor_record_offset":276,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2616,"record_offset":1392,"parent_record_offset":1348,"ancestor_record_offset":276,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2616,"record_offset":780,"parent_record_offset":752,"ancestor_record_offset":276,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2616,"record_offset":1904,"parent_record_offset":1876,"ancestor_record_offset":276,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2616,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":276,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2616,"record_offset":1964,"parent_record_offset":1836,"ancestor_record_offset":276,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2616,"record_offset":2848,"parent_record_offset":2400,"ancestor_record_offset":276,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2616,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":276,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2616,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":276,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2616,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":276,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2616,"record_offset":4772,"parent_record_offset":4624,"ancestor_record_offset":276,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2616,"record_offset":1504,"parent_record_offset":1348,"ancestor_record_offset":276,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2616,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":276,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2616,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":276,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2616,"record_offset":484,"parent_record_offset":460,"ancestor_record_offset":276,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A5BC
// 首条位置: {"module":2624,"inlinee":107964}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2616,"record_offset":5400,"rva":34241760,"va":5402950880}
// 编译实例 RVAs: 0x20a7ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-27272280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e8a8db5ffe","size":5,"soft_fail":false,"target_va":5375678688,"va":5402950963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-27272886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e84ad95ffe","size":5,"soft_fail":false,"target_va":5375678704,"va":5402951585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34228763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e8e5b5f5fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5402950950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11213387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e8b5e554ff","size":5,"soft_fail":false,"target_va":5391737856,"va":5402951238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13669270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e89693d000","size":5,"soft_fail":false,"target_va":5416620769,"va":5402951494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13669279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e89f93d000","size":5,"soft_fail":false,"target_va":5416620794,"va":5402951510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24015082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e8168f91fe","size":5,"soft_fail":false,"target_va":5378936272,"va":5402951349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24012686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e8729891fe","size":5,"soft_fail":false,"target_va":5378938736,"va":5402951417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24014954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e8968f91fe","size":5,"soft_fail":false,"target_va":5378936352,"va":5402951301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24015298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e83e8e91fe","size":5,"soft_fail":false,"target_va":5378936032,"va":5402951325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::thick_line
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24015399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402950880,"raw_hex":"e8d98d91fe","size":5,"soft_fail":false,"target_va":5378935984,"va":5402951378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2616,"record_offset":6752,"parent_record_offset":5400,"ancestor_record_offset":5400,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2616,"record_offset":6568,"parent_record_offset":5400,"ancestor_record_offset":5400,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2616,"record_offset":5572,"parent_record_offset":5400,"ancestor_record_offset":5400,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2616,"record_offset":6836,"parent_record_offset":5400,"ancestor_record_offset":5400,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":6032,"parent_record_offset":6008,"ancestor_record_offset":5400,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":5400,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":5400,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":6980,"parent_record_offset":6956,"ancestor_record_offset":5400,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":5400,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":5400,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":6008,"parent_record_offset":5972,"ancestor_record_offset":5400,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":5400,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2616,"record_offset":6596,"parent_record_offset":6568,"ancestor_record_offset":5400,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2616,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":5400,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":5904,"parent_record_offset":5872,"ancestor_record_offset":5400,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":5400,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":5972,"parent_record_offset":5936,"ancestor_record_offset":5400,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":5936,"parent_record_offset":5904,"ancestor_record_offset":5400,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":6884,"parent_record_offset":6860,"ancestor_record_offset":5400,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2616,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":5400,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":6168,"parent_record_offset":6140,"ancestor_record_offset":5400,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":5400,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":5840,"parent_record_offset":5656,"ancestor_record_offset":5400,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":5872,"parent_record_offset":5840,"ancestor_record_offset":5400,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":5600,"parent_record_offset":5572,"ancestor_record_offset":5400,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":6388,"parent_record_offset":6356,"ancestor_record_offset":5400,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":2616,"record_offset":6464,"parent_record_offset":6388,"ancestor_record_offset":5400,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2616,"record_offset":6140,"parent_record_offset":5656,"ancestor_record_offset":5400,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":2616,"record_offset":5628,"parent_record_offset":5600,"ancestor_record_offset":5400,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":2616,"record_offset":5656,"parent_record_offset":5628,"ancestor_record_offset":5400,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":2616,"record_offset":6224,"parent_record_offset":6196,"ancestor_record_offset":5400,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":2616,"record_offset":5700,"parent_record_offset":5656,"ancestor_record_offset":5400,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":2616,"record_offset":6292,"parent_record_offset":6252,"ancestor_record_offset":5400,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2616,"record_offset":6196,"parent_record_offset":6168,"ancestor_record_offset":5400,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2616,"record_offset":6356,"parent_record_offset":6292,"ancestor_record_offset":5400,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2616,"record_offset":5796,"parent_record_offset":5772,"ancestor_record_offset":5400,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2616,"record_offset":6860,"parent_record_offset":6836,"ancestor_record_offset":5400,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2616,"record_offset":5772,"parent_record_offset":5748,"ancestor_record_offset":5400,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2616,"record_offset":6504,"parent_record_offset":6252,"ancestor_record_offset":5400,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2616,"record_offset":5748,"parent_record_offset":5724,"ancestor_record_offset":5400,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2616,"record_offset":6428,"parent_record_offset":6388,"ancestor_record_offset":5400,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2616,"record_offset":6252,"parent_record_offset":6224,"ancestor_record_offset":5400,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2616,"record_offset":5724,"parent_record_offset":5700,"ancestor_record_offset":5400,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] system_glade_border::display_glade_border::display_glade_border::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-glade-border","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/glade-border/src/display_glade_border.rs
//     evidence={"module":2616,"record_offset":6320,"parent_record_offset":6292,"ancestor_record_offset":5400,"inlinee":107940,"source_line":55,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A5A4
// 首条位置: {"module":2616,"inlinee":107940}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_glade_border::display_glade_border::display_glade_border::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2616,"record_offset":7232,"rva":34242576,"va":5402951696}
// 编译实例 RVAs: 0x20a8010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-27273078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e88ad85ffe","size":5,"soft_fail":false,"target_va":5375678688,"va":5402951761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-27273523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e8cdd65ffe","size":5,"soft_fail":false,"target_va":5375678704,"va":5402952222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34229559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e8c9b2f5fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5402951746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: display_glade_border.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14105147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e83b3ad700","size":5,"soft_fail":false,"target_va":5417057056,"va":5402951904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13668603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e8fb90d000","size":5,"soft_fail":false,"target_va":5416620769,"va":5402952161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13668968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e86892d000","size":5,"soft_fail":false,"target_va":5416620794,"va":5402951821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24015710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e8a28c91fe","size":5,"soft_fail":false,"target_va":5378936272,"va":5402951977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24013336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e8e89591fe","size":5,"soft_fail":false,"target_va":5378938736,"va":5402952067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::dashed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24005306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e846b591fe","size":5,"soft_fail":false,"target_va":5378946704,"va":5402952005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24015596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e8148d91fe","size":5,"soft_fail":false,"target_va":5378936352,"va":5402951943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24015933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e8c38b91fe","size":5,"soft_fail":false,"target_va":5378936032,"va":5402951960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::thick_line
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24016048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402951696,"raw_hex":"e8508b91fe","size":5,"soft_fail":false,"target_va":5378935984,"va":5402952027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2616,"record_offset":8168,"parent_record_offset":7232,"ancestor_record_offset":7232,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2616,"record_offset":7404,"parent_record_offset":7232,"ancestor_record_offset":7232,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":7584,"parent_record_offset":7232,"ancestor_record_offset":7232,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2616,"record_offset":8252,"parent_record_offset":7232,"ancestor_record_offset":7232,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":7488,"parent_record_offset":7464,"ancestor_record_offset":7232,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":7512,"parent_record_offset":7488,"ancestor_record_offset":7232,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":7232,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":7232,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":7232,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":7232,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":7464,"parent_record_offset":7432,"ancestor_record_offset":7232,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2616,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":7232,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2616,"record_offset":7432,"parent_record_offset":7404,"ancestor_record_offset":7232,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2616,"record_offset":8276,"parent_record_offset":8252,"ancestor_record_offset":7232,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":7232,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":7792,"parent_record_offset":7760,"ancestor_record_offset":7232,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":7232,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2616,"record_offset":7828,"parent_record_offset":7792,"ancestor_record_offset":7232,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2616,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":7232,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":2616,"record_offset":7676,"parent_record_offset":7612,"ancestor_record_offset":7232,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":7648,"parent_record_offset":7612,"ancestor_record_offset":7232,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":7232,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":7712,"parent_record_offset":7676,"ancestor_record_offset":7232,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2616,"record_offset":7612,"parent_record_offset":7584,"ancestor_record_offset":7232,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":2616,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":7232,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2616,"record_offset":7932,"parent_record_offset":7876,"ancestor_record_offset":7232,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2616,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":7232,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2616,"record_offset":7904,"parent_record_offset":7876,"ancestor_record_offset":7232,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2616,"record_offset":8100,"parent_record_offset":7676,"ancestor_record_offset":7232,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/02616.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2616,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":7232,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/02616.jsonl.gz"}
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
// 完整结构及原始类型记录: display_glade_border.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: display_glade_border.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: display_glade_border.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::DisplayOutOfBorder>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border
// 信息 ID: 9dafe6f111cf51cb2d252fce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f19c562820910d365be4d9a8: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> @ 0x10846a3 -> 0x20a7710 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::ShowDemoBorderOverlay>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > @ 0x20aacc9 -> 0x20a7710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border::closure$0
// 信息 ID: 03e96a6ecb9f343fffcfc837
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_glade_border::display_glade_border::display_glade_border", "captures": "unknown"}
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_glade_border::display_glade_border::display_glade_border::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_glade_border::display_glade_border::display_glade_border::closure_env$0> >; module=2624, inline_record=3784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border::closure$1
// 信息 ID: 29b966dcb779e0dea9311bf7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_glade_border::display_glade_border::display_glade_border", "captures": "unknown"}
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$1 <- core::ops::function::FnOnce::call_once<system_glade_border::display_glade_border::display_glade_border::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$1 <- core::ptr::drop_in_place<system_glade_border::display_glade_border::display_glade_border::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_glade_border::display_glade_border::display_glade_border::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x20a8303 -> 0x20a7ce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border::closure$1::closure$0
// 信息 ID: 00a8f443424fbd15136c412d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_glade_border::display_glade_border::display_glade_border", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_glade_border::display_glade_border::display_glade_border::closure$1; module=2616, inline_record=6320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_glade_border::display_glade_border::display_glade_border::closure$2
// 信息 ID: ea507332b6f72fe67d692b88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_glade_border::display_glade_border::display_glade_border", "captures": "unknown"}
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$2 <- core::ops::function::FnOnce::call_once<system_glade_border::display_glade_border::display_glade_border::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_glade_border::display_glade_border::display_glade_border::closure_env$2 <- core::ptr::drop_in_place<system_glade_border::display_glade_border::display_glade_border::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_glade_border::display_glade_border::display_glade_border::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x20a8263 -> 0x20a8010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
