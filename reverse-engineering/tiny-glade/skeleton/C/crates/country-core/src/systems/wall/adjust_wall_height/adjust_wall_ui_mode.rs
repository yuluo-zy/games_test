// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ed085f68eb30aa6612b6e70abd77eac755c83d89eddda06e6e05b1da5d02bc50
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: adjust_wall_ui_mode.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 507, 585, 600, 1643, 1682
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":600,"record_offset":68136,"rva":12895248,"va":5381604368}
// 编译实例 RVAs: 0xc4c410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-5925823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381604368,"raw_hex":"e84194a5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381604506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-5926763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381604368,"raw_hex":"e89590a5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381605462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=5
//     target_aliases=1699 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12882304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381604368,"raw_hex":"e8806e3bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381604491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t285004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381604368,"raw_hex":"e84c590400","size":5,"soft_fail":false,"target_va":5381890784,"va":5381605775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12291568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381604368,"raw_hex":"e8f08dbb00","size":5,"soft_fail":false,"target_va":5393896944,"va":5381605371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35452201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381604368,"raw_hex":"e829f51c02","size":5,"soft_fail":false,"target_va":5417057056,"va":5381604850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35015263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381604368,"raw_hex":"e85f4a1602","size":5,"soft_fail":false,"target_va":5416620769,"va":5381605501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3190381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381604368,"raw_hex":"e86dae3000","size":5,"soft_fail":false,"target_va":5384794864,"va":5381604478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35016208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381604368,"raw_hex":"e8104e1602","size":5,"soft_fail":false,"target_va":5416620794,"va":5381604581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":600,"record_offset":68380,"parent_record_offset":68136,"ancestor_record_offset":68136,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":600,"record_offset":68460,"parent_record_offset":68136,"ancestor_record_offset":68136,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":600,"record_offset":71172,"parent_record_offset":68136,"ancestor_record_offset":68136,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":600,"record_offset":71032,"parent_record_offset":68136,"ancestor_record_offset":68136,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":72548,"parent_record_offset":68136,"ancestor_record_offset":68136,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":600,"record_offset":68352,"parent_record_offset":68136,"ancestor_record_offset":68136,"inlinee":28322,"source_line":306,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":600,"record_offset":70940,"parent_record_offset":68136,"ancestor_record_offset":68136,"inlinee":28321,"source_line":292,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":600,"record_offset":68680,"parent_record_offset":68136,"ancestor_record_offset":68136,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::OwnedSignifierStream::write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":600,"record_offset":72520,"parent_record_offset":68136,"ancestor_record_offset":68136,"inlinee":28613,"source_line":95,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":600,"record_offset":69496,"parent_record_offset":68136,"ancestor_record_offset":68136,"inlinee":28993,"source_line":170,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":68580,"parent_record_offset":68556,"ancestor_record_offset":68136,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":68604,"parent_record_offset":68580,"ancestor_record_offset":68136,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":71624,"parent_record_offset":71600,"ancestor_record_offset":68136,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":71648,"parent_record_offset":71624,"ancestor_record_offset":68136,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":68628,"parent_record_offset":68604,"ancestor_record_offset":68136,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":71672,"parent_record_offset":71648,"ancestor_record_offset":68136,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":68556,"parent_record_offset":68524,"ancestor_record_offset":68136,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":71600,"parent_record_offset":71544,"ancestor_record_offset":68136,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":68524,"parent_record_offset":68488,"ancestor_record_offset":68136,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":68488,"parent_record_offset":68460,"ancestor_record_offset":68136,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":600,"record_offset":68428,"parent_record_offset":68404,"ancestor_record_offset":68136,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":68916,"parent_record_offset":68888,"ancestor_record_offset":68136,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":71572,"parent_record_offset":71544,"ancestor_record_offset":68136,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":71544,"parent_record_offset":71520,"ancestor_record_offset":68136,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":68980,"parent_record_offset":68944,"ancestor_record_offset":68136,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":68944,"parent_record_offset":68916,"ancestor_record_offset":68136,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":71520,"parent_record_offset":71496,"ancestor_record_offset":68136,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":69020,"parent_record_offset":68980,"ancestor_record_offset":68136,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":68800,"parent_record_offset":68772,"ancestor_record_offset":68136,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":68828,"parent_record_offset":68800,"ancestor_record_offset":68136,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":68856,"parent_record_offset":68828,"ancestor_record_offset":68136,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":68772,"parent_record_offset":68744,"ancestor_record_offset":68136,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":600,"record_offset":68744,"parent_record_offset":68716,"ancestor_record_offset":68136,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":600,"record_offset":68716,"parent_record_offset":68680,"ancestor_record_offset":68136,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":600,"record_offset":69364,"parent_record_offset":69340,"ancestor_record_offset":68136,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":70296,"parent_record_offset":70268,"ancestor_record_offset":68136,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":69812,"parent_record_offset":69768,"ancestor_record_offset":68136,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":68888,"parent_record_offset":68856,"ancestor_record_offset":68136,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":69704,"parent_record_offset":69672,"ancestor_record_offset":68136,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":70696,"parent_record_offset":70664,"ancestor_record_offset":68136,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":600,"record_offset":70772,"parent_record_offset":70696,"ancestor_record_offset":68136,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":600,"record_offset":70268,"parent_record_offset":69768,"ancestor_record_offset":68136,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":600,"record_offset":69736,"parent_record_offset":69704,"ancestor_record_offset":68136,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":600,"record_offset":69768,"parent_record_offset":69736,"ancestor_record_offset":68136,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":600,"record_offset":69044,"parent_record_offset":69020,"ancestor_record_offset":68136,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":600,"record_offset":69628,"parent_record_offset":69604,"ancestor_record_offset":68136,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":600,"record_offset":69604,"parent_record_offset":69580,"ancestor_record_offset":68136,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":600,"record_offset":69092,"parent_record_offset":69068,"ancestor_record_offset":68136,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":600,"record_offset":70352,"parent_record_offset":70324,"ancestor_record_offset":68136,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":600,"record_offset":70520,"parent_record_offset":70408,"ancestor_record_offset":68136,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":69556,"parent_record_offset":69532,"ancestor_record_offset":68136,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":70380,"parent_record_offset":70352,"ancestor_record_offset":68136,"inlinee":9710,"source_line":99,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":70408,"parent_record_offset":70380,"ancestor_record_offset":68136,"inlinee":9711,"source_line":275,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":70812,"parent_record_offset":70408,"ancestor_record_offset":68136,"inlinee":10719,"source_line":267,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":69580,"parent_record_offset":69556,"ancestor_record_offset":68136,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":70480,"parent_record_offset":70448,"ancestor_record_offset":68136,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":70448,"parent_record_offset":70408,"ancestor_record_offset":68136,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":69672,"parent_record_offset":69496,"ancestor_record_offset":68136,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":70324,"parent_record_offset":70296,"ancestor_record_offset":68136,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":70664,"parent_record_offset":70520,"ancestor_record_offset":68136,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":69532,"parent_record_offset":69496,"ancestor_record_offset":68136,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":69068,"parent_record_offset":69044,"ancestor_record_offset":68136,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":70068,"parent_record_offset":70040,"ancestor_record_offset":68136,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":600,"record_offset":70620,"parent_record_offset":70588,"ancestor_record_offset":68136,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":600,"record_offset":70588,"parent_record_offset":70556,"ancestor_record_offset":68136,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":600,"record_offset":68404,"parent_record_offset":68380,"ancestor_record_offset":68136,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":600,"record_offset":69260,"parent_record_offset":68856,"ancestor_record_offset":68136,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":69284,"parent_record_offset":69260,"ancestor_record_offset":68136,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":69340,"parent_record_offset":68680,"ancestor_record_offset":68136,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":600,"record_offset":70860,"parent_record_offset":70836,"ancestor_record_offset":68136,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":70736,"parent_record_offset":70696,"ancestor_record_offset":68136,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":600,"record_offset":70836,"parent_record_offset":70812,"ancestor_record_offset":68136,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":600,"record_offset":69400,"parent_record_offset":69364,"ancestor_record_offset":68136,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":600,"record_offset":69424,"parent_record_offset":69400,"ancestor_record_offset":68136,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":600,"record_offset":69456,"parent_record_offset":69364,"ancestor_record_offset":68136,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":600,"record_offset":70964,"parent_record_offset":70940,"ancestor_record_offset":68136,"inlinee":28107,"source_line":183,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":600,"record_offset":70996,"parent_record_offset":70964,"ancestor_record_offset":68136,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":600,"record_offset":70556,"parent_record_offset":70520,"ancestor_record_offset":68136,"inlinee":28995,"source_line":175,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":600,"record_offset":62604,"rva":12893776,"va":5381602896}
// 编译实例 RVAs: 0xc4be50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-5924398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602896,"raw_hex":"e8d299a5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381603081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12880875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602896,"raw_hex":"e815743bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381603062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fa820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3480447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602896,"raw_hex":"e881e4caff","size":5,"soft_fail":false,"target_va":5378123808,"va":5381604250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3190762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602896,"raw_hex":"e8eaaf3000","size":5,"soft_fail":false,"target_va":5384794864,"va":5381604097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35017510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602896,"raw_hex":"e826531602","size":5,"soft_fail":false,"target_va":5416620794,"va":5381603279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::ui::resources::widget_params::WidgetVolumeShape> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-817033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381602896,"raw_hex":"e87788f3ff","size":5,"soft_fail":false,"target_va":5380787328,"va":5381604356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":600,"record_offset":67988,"parent_record_offset":62604,"ancestor_record_offset":62604,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":600,"record_offset":68072,"parent_record_offset":62604,"ancestor_record_offset":62604,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":600,"record_offset":67960,"parent_record_offset":62604,"ancestor_record_offset":62604,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":600,"record_offset":68100,"parent_record_offset":62604,"ancestor_record_offset":62604,"inlinee":28125,"source_line":95,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":600,"record_offset":62796,"parent_record_offset":62604,"ancestor_record_offset":62604,"inlinee":28993,"source_line":170,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":64208,"parent_record_offset":64184,"ancestor_record_offset":62604,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":64232,"parent_record_offset":64208,"ancestor_record_offset":62604,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":64256,"parent_record_offset":64232,"ancestor_record_offset":62604,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":64184,"parent_record_offset":64036,"ancestor_record_offset":62604,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":600,"record_offset":68036,"parent_record_offset":68012,"ancestor_record_offset":62604,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":63192,"parent_record_offset":63160,"ancestor_record_offset":62604,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":63256,"parent_record_offset":63224,"ancestor_record_offset":62604,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":63224,"parent_record_offset":63192,"ancestor_record_offset":62604,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":63288,"parent_record_offset":63256,"ancestor_record_offset":62604,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":63500,"parent_record_offset":63476,"ancestor_record_offset":62604,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":63128,"parent_record_offset":63088,"ancestor_record_offset":62604,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":63160,"parent_record_offset":63128,"ancestor_record_offset":62604,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":63016,"parent_record_offset":62980,"ancestor_record_offset":62604,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":63668,"parent_record_offset":63644,"ancestor_record_offset":62604,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":600,"record_offset":64824,"parent_record_offset":64756,"ancestor_record_offset":62604,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":600,"record_offset":63476,"parent_record_offset":63088,"ancestor_record_offset":62604,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":600,"record_offset":63052,"parent_record_offset":63016,"ancestor_record_offset":62604,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":600,"record_offset":63088,"parent_record_offset":63052,"ancestor_record_offset":62604,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":600,"record_offset":63320,"parent_record_offset":63288,"ancestor_record_offset":62604,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":600,"record_offset":62936,"parent_record_offset":62912,"ancestor_record_offset":62604,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":600,"record_offset":62912,"parent_record_offset":62888,"ancestor_record_offset":62604,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":600,"record_offset":63412,"parent_record_offset":63352,"ancestor_record_offset":62604,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":600,"record_offset":63548,"parent_record_offset":63524,"ancestor_record_offset":62604,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":600,"record_offset":63620,"parent_record_offset":63596,"ancestor_record_offset":62604,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":62864,"parent_record_offset":62840,"ancestor_record_offset":62604,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":63572,"parent_record_offset":63548,"ancestor_record_offset":62604,"inlinee":9710,"source_line":99,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":63596,"parent_record_offset":63572,"ancestor_record_offset":62604,"inlinee":9711,"source_line":275,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":64864,"parent_record_offset":64476,"ancestor_record_offset":62604,"inlinee":10719,"source_line":267,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":600,"record_offset":62888,"parent_record_offset":62864,"ancestor_record_offset":62604,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":64548,"parent_record_offset":64516,"ancestor_record_offset":62604,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":64516,"parent_record_offset":64476,"ancestor_record_offset":62604,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":62980,"parent_record_offset":62796,"ancestor_record_offset":62604,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":63524,"parent_record_offset":63500,"ancestor_record_offset":62604,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":63644,"parent_record_offset":63620,"ancestor_record_offset":62604,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":62840,"parent_record_offset":62796,"ancestor_record_offset":62604,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":63352,"parent_record_offset":63320,"ancestor_record_offset":62604,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":63384,"parent_record_offset":63352,"ancestor_record_offset":62604,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":600,"record_offset":64684,"parent_record_offset":64652,"ancestor_record_offset":62604,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":600,"record_offset":64652,"parent_record_offset":64620,"ancestor_record_offset":62604,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":600,"record_offset":68012,"parent_record_offset":67988,"ancestor_record_offset":62604,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":600,"record_offset":64912,"parent_record_offset":64888,"ancestor_record_offset":62604,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":63692,"parent_record_offset":63668,"ancestor_record_offset":62604,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":600,"record_offset":64888,"parent_record_offset":64864,"ancestor_record_offset":62604,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":600,"record_offset":64620,"parent_record_offset":64588,"ancestor_record_offset":62604,"inlinee":28995,"source_line":175,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x91C0
// 首条位置: {"module":585,"inlinee":37312}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":600,"record_offset":74224,"rva":12896944,"va":5381606064}
// 编译实例 RVAs: 0xc4cab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5927443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606064,"raw_hex":"e8ed8da5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381606126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5927888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606064,"raw_hex":"e8308ca5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381606587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12883924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606064,"raw_hex":"e82c683bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381606111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35450786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606064,"raw_hex":"e8a2ef1c02","size":5,"soft_fail":false,"target_va":5417057056,"va":5381606265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35014238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606064,"raw_hex":"e85e461602","size":5,"soft_fail":false,"target_va":5416620769,"va":5381606526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35014605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606064,"raw_hex":"e8cd471602","size":5,"soft_fail":false,"target_va":5416620794,"va":5381606184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2667696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606064,"raw_hex":"e8504bd7ff","size":5,"soft_fail":false,"target_va":5378938736,"va":5381606427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":75252,"parent_record_offset":74224,"ancestor_record_offset":74224,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":74428,"parent_record_offset":74224,"ancestor_record_offset":74224,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":74608,"parent_record_offset":74224,"ancestor_record_offset":74224,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":75336,"parent_record_offset":74224,"ancestor_record_offset":74224,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":600,"record_offset":75224,"parent_record_offset":74224,"ancestor_record_offset":74224,"inlinee":28623,"source_line":284,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":600,"record_offset":75196,"parent_record_offset":74224,"ancestor_record_offset":74224,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":74512,"parent_record_offset":74488,"ancestor_record_offset":74224,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":74536,"parent_record_offset":74512,"ancestor_record_offset":74224,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":75408,"parent_record_offset":75384,"ancestor_record_offset":74224,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":75432,"parent_record_offset":75408,"ancestor_record_offset":74224,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":74560,"parent_record_offset":74536,"ancestor_record_offset":74224,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":75456,"parent_record_offset":75432,"ancestor_record_offset":74224,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":74488,"parent_record_offset":74456,"ancestor_record_offset":74224,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":75384,"parent_record_offset":75360,"ancestor_record_offset":74224,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":74456,"parent_record_offset":74428,"ancestor_record_offset":74224,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":75360,"parent_record_offset":75336,"ancestor_record_offset":74224,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":74764,"parent_record_offset":74740,"ancestor_record_offset":74224,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":74820,"parent_record_offset":74788,"ancestor_record_offset":74224,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":74788,"parent_record_offset":74764,"ancestor_record_offset":74224,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":74856,"parent_record_offset":74820,"ancestor_record_offset":74224,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":75276,"parent_record_offset":75252,"ancestor_record_offset":74224,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":74700,"parent_record_offset":74636,"ancestor_record_offset":74224,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":74672,"parent_record_offset":74636,"ancestor_record_offset":74224,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":75300,"parent_record_offset":75276,"ancestor_record_offset":74224,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":74740,"parent_record_offset":74700,"ancestor_record_offset":74224,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":74636,"parent_record_offset":74608,"ancestor_record_offset":74224,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":600,"record_offset":74880,"parent_record_offset":74856,"ancestor_record_offset":74224,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":600,"record_offset":74960,"parent_record_offset":74904,"ancestor_record_offset":74224,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":74904,"parent_record_offset":74880,"ancestor_record_offset":74224,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":74932,"parent_record_offset":74904,"ancestor_record_offset":74224,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":600,"record_offset":75128,"parent_record_offset":74700,"ancestor_record_offset":74224,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":75152,"parent_record_offset":75128,"ancestor_record_offset":74224,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":600,"record_offset":75508,"rva":12897504,"va":5381606624}
// 编译实例 RVAs: 0xc4cce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5927997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606624,"raw_hex":"e8c38ba5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381606680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5928330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606624,"raw_hex":"e8768aa5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381607029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12884478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606624,"raw_hex":"e802663bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381606665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416289b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10337197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606624,"raw_hex":"e8adbb9d00","size":5,"soft_fail":false,"target_va":5391944112,"va":5381606910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35013791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606624,"raw_hex":"e89f441602","size":5,"soft_fail":false,"target_va":5416620769,"va":5381606973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2668116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381606624,"raw_hex":"e8ac49d7ff","size":5,"soft_fail":false,"target_va":5378938736,"va":5381606847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":75976,"parent_record_offset":75508,"ancestor_record_offset":75508,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":75712,"parent_record_offset":75508,"ancestor_record_offset":75508,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":76060,"parent_record_offset":75508,"ancestor_record_offset":75508,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":600,"record_offset":75920,"parent_record_offset":75508,"ancestor_record_offset":75508,"inlinee":28623,"source_line":284,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":600,"record_offset":75892,"parent_record_offset":75508,"ancestor_record_offset":75508,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::signifier_animation::WallHeightSignifier::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/signifier_animation.rs
//     evidence={"module":600,"record_offset":75948,"parent_record_offset":75508,"ancestor_record_offset":75508,"inlinee":29047,"source_line":27,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":75796,"parent_record_offset":75772,"ancestor_record_offset":75508,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":75820,"parent_record_offset":75796,"ancestor_record_offset":75508,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":76132,"parent_record_offset":76108,"ancestor_record_offset":75508,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":76156,"parent_record_offset":76132,"ancestor_record_offset":75508,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":75844,"parent_record_offset":75820,"ancestor_record_offset":75508,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":76180,"parent_record_offset":76156,"ancestor_record_offset":75508,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":75772,"parent_record_offset":75740,"ancestor_record_offset":75508,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":76108,"parent_record_offset":76084,"ancestor_record_offset":75508,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":75740,"parent_record_offset":75712,"ancestor_record_offset":75508,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":600,"record_offset":76084,"parent_record_offset":76060,"ancestor_record_offset":75508,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":600,"record_offset":76000,"parent_record_offset":75976,"ancestor_record_offset":75508,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":76024,"parent_record_offset":76000,"ancestor_record_offset":75508,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7141
// 首条位置: {"module":507,"inlinee":28993}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":507,"record_offset":43960,"parent_record_offset":43784,"ancestor_record_offset":43784,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":507,"record_offset":43820,"parent_record_offset":43784,"ancestor_record_offset":43784,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":44444,"parent_record_offset":44420,"ancestor_record_offset":43784,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":44468,"parent_record_offset":44444,"ancestor_record_offset":43784,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":44492,"parent_record_offset":44468,"ancestor_record_offset":43784,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":44420,"parent_record_offset":44232,"ancestor_record_offset":43784,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":44164,"parent_record_offset":44132,"ancestor_record_offset":43784,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":44232,"parent_record_offset":44196,"ancestor_record_offset":43784,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":44196,"parent_record_offset":44164,"ancestor_record_offset":43784,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":44276,"parent_record_offset":44232,"ancestor_record_offset":43784,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":44580,"parent_record_offset":44552,"ancestor_record_offset":43784,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":44100,"parent_record_offset":44056,"ancestor_record_offset":43784,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":44132,"parent_record_offset":44100,"ancestor_record_offset":43784,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":43992,"parent_record_offset":43960,"ancestor_record_offset":43784,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":44980,"parent_record_offset":44948,"ancestor_record_offset":43784,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":507,"record_offset":45056,"parent_record_offset":44980,"ancestor_record_offset":43784,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":507,"record_offset":44552,"parent_record_offset":44056,"ancestor_record_offset":43784,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":507,"record_offset":44024,"parent_record_offset":43992,"ancestor_record_offset":43784,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":507,"record_offset":44056,"parent_record_offset":44024,"ancestor_record_offset":43784,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":507,"record_offset":44300,"parent_record_offset":44276,"ancestor_record_offset":43784,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":43916,"parent_record_offset":43892,"ancestor_record_offset":43784,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":43892,"parent_record_offset":43868,"ancestor_record_offset":43784,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":507,"record_offset":44380,"parent_record_offset":44324,"ancestor_record_offset":43784,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":507,"record_offset":44636,"parent_record_offset":44608,"ancestor_record_offset":43784,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":507,"record_offset":44804,"parent_record_offset":44692,"ancestor_record_offset":43784,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":507,"record_offset":43844,"parent_record_offset":43820,"ancestor_record_offset":43784,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":507,"record_offset":44664,"parent_record_offset":44636,"ancestor_record_offset":43784,"inlinee":9710,"source_line":99,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":507,"record_offset":44692,"parent_record_offset":44664,"ancestor_record_offset":43784,"inlinee":9711,"source_line":275,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":507,"record_offset":45096,"parent_record_offset":44692,"ancestor_record_offset":43784,"inlinee":10719,"source_line":267,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":507,"record_offset":43868,"parent_record_offset":43844,"ancestor_record_offset":43784,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":507,"record_offset":44764,"parent_record_offset":44732,"ancestor_record_offset":43784,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":507,"record_offset":44732,"parent_record_offset":44692,"ancestor_record_offset":43784,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":507,"record_offset":44608,"parent_record_offset":44580,"ancestor_record_offset":43784,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":507,"record_offset":44948,"parent_record_offset":44804,"ancestor_record_offset":43784,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":44324,"parent_record_offset":44300,"ancestor_record_offset":43784,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":44352,"parent_record_offset":44324,"ancestor_record_offset":43784,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":44904,"parent_record_offset":44872,"ancestor_record_offset":43784,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":507,"record_offset":44872,"parent_record_offset":44840,"ancestor_record_offset":43784,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":507,"record_offset":45144,"parent_record_offset":45120,"ancestor_record_offset":43784,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":45020,"parent_record_offset":44980,"ancestor_record_offset":43784,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":507,"record_offset":45120,"parent_record_offset":45096,"ancestor_record_offset":43784,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":507,"record_offset":44840,"parent_record_offset":44804,"ancestor_record_offset":43784,"inlinee":28995,"source_line":175,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7143
// 首条位置: {"module":507,"inlinee":28995}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":507,"record_offset":44872,"parent_record_offset":44840,"ancestor_record_offset":44840,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":44904,"parent_record_offset":44872,"ancestor_record_offset":44840,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00507.jsonl.gz"}
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
// 完整结构及原始类型记录: adjust_wall_ui_mode.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: adjust_wall_ui_mode.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: adjust_wall_ui_mode.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 27; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] relative_path::relative_path_buf::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::new
// 信息 ID: 33c519c82d210e094b096772
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height; module=585, inline_record=16176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier
// 信息 ID: 795e7714346e1710454cfe2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height @ 0xbeaaa8 -> 0xc4c410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget
// 信息 ID: 619af19f5b5f619d7de341c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height @ 0xbeaa59 -> 0xc4be50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
// 信息 ID: 7f26c05da2a0efcb1baab458
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation; module=507, inline_record=43784; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=600, inline_record=62796; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=600, inline_record=63776; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=600, inline_record=64992; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=600, inline_record=66408; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=600, inline_record=67708; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier; module=600, inline_record=69496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
// 信息 ID: b04d06066b900dfb70a5f67a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation; module=507, inline_record=44840; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=600, inline_record=64620; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=600, inline_record=66024; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=600, inline_record=67336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier; module=600, inline_record=70556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$0
// 信息 ID: 4ed7c4cecd118ad9755f1e80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xc4d523 -> 0xc4cab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$1
// 信息 ID: bf321033bc1e3c9e1f332689
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xc4d66d -> 0xc4cce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
