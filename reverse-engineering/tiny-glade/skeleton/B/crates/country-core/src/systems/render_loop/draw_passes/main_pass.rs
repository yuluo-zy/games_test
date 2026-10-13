// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d8a86023a238823074ef69d2f7b47ac0fd4d663ec1249a4444378fde87bec4c8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: main_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 475
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::with_stencil_write
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6586
// 首条位置: {"module":475,"inlinee":25990}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":21820,"parent_record_offset":21708,"ancestor_record_offset":21708,"inlinee":25993,"source_line":632,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":21736,"parent_record_offset":21708,"ancestor_record_offset":21708,"inlinee":25991,"source_line":630,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":475,"record_offset":21784,"parent_record_offset":21760,"ancestor_record_offset":21708,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":21844,"parent_record_offset":21820,"ancestor_record_offset":21708,"inlinee":25994,"source_line":632,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":21708,"inlinee":25992,"source_line":630,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":17132,"rva":8600512,"va":5377309632}
// 编译实例 RVAs: 0x833bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=6
//     target_aliases=35 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7709956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8fc5a8aff","size":5,"soft_fail":false,"target_va":5369600800,"va":5377310751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=7
//     target_aliases=19 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7694399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8c1978aff","size":5,"soft_fail":false,"target_va":5369616112,"va":5377310506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t380282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e87acd0500","size":5,"soft_fail":false,"target_va":5377696704,"va":5377316417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t384787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e813df0500","size":5,"soft_fail":false,"target_va":5377701312,"va":5377316520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090b460
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t881442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e822730d00","size":5,"soft_fail":false,"target_va":5378192480,"va":5377311033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3876668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e83c273b00","size":5,"soft_fail":false,"target_va":5381187408,"va":5377310735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34354300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e87c340c02","size":5,"soft_fail":false,"target_va":5411669968,"va":5377315663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34536162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8e2fa0e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377311001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34225858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8c23e0a02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377315721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t34226014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e85e3f0a02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377315757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8f3ecffff","size":5,"soft_fail":false,"target_va":5377304864,"va":5377309736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34233549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8cd5c0a02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377309806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t858857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8e91a0d00","size":5,"soft_fail":false,"target_va":5378169824,"va":5377310962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t211571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8733a0300","size":5,"soft_fail":false,"target_va":5377522032,"va":5377310456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t213434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8ba410300","size":5,"soft_fail":false,"target_va":5377524592,"va":5377311153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4075273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8092f3e00","size":5,"soft_fail":false,"target_va":5381385536,"va":5377310258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4080462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e84e433e00","size":5,"soft_fail":false,"target_va":5381390640,"va":5377310173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4070861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8cd1d3e00","size":5,"soft_fail":false,"target_va":5381386480,"va":5377315614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4074998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8f62d3e00","size":5,"soft_fail":false,"target_va":5381384704,"va":5377309701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3807518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e81e193a00","size":5,"soft_fail":false,"target_va":5381117840,"va":5377310317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11509031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e8279daf00","size":5,"soft_fail":false,"target_va":5388825296,"va":5377316260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t34221390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377309632,"raw_hex":"e84e2d0a02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377315709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377309632,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5377311100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":20796,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":21640,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":33168,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":21056,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::stencil_ref
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":475,"record_offset":21676,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":25988,"source_line":122,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":475,"record_offset":21132,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":25983,"source_line":32,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":475,"record_offset":19756,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::shader_refactoring_state::ShaderRefactoringState::disabled
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/shader_refactoring_state.rs
//     evidence={"module":475,"record_offset":27028,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":25996,"source_line":37,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":475,"record_offset":19880,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::main_pass::with_stencil_write
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
//     evidence={"module":475,"record_offset":21708,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":25990,"source_line":22,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":475,"record_offset":26192,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":18772,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":17340,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":17796,"parent_record_offset":17132,"ancestor_record_offset":17132,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":20380,"parent_record_offset":20356,"ancestor_record_offset":17132,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":20428,"parent_record_offset":20404,"ancestor_record_offset":17132,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":20404,"parent_record_offset":20380,"ancestor_record_offset":17132,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":20356,"parent_record_offset":20332,"ancestor_record_offset":17132,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":20332,"parent_record_offset":20292,"ancestor_record_offset":17132,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":20292,"parent_record_offset":20268,"ancestor_record_offset":17132,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":20680,"parent_record_offset":20656,"ancestor_record_offset":17132,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":20656,"parent_record_offset":20632,"ancestor_record_offset":17132,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":20604,"parent_record_offset":20572,"ancestor_record_offset":17132,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":20548,"parent_record_offset":20012,"ancestor_record_offset":17132,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":20572,"parent_record_offset":20548,"ancestor_record_offset":17132,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":20040,"parent_record_offset":20012,"ancestor_record_offset":17132,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":20632,"parent_record_offset":20572,"ancestor_record_offset":17132,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":20088,"parent_record_offset":20064,"ancestor_record_offset":17132,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":20064,"parent_record_offset":20040,"ancestor_record_offset":17132,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":20268,"parent_record_offset":20244,"ancestor_record_offset":17132,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":19972,"parent_record_offset":19944,"ancestor_record_offset":17132,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":19944,"parent_record_offset":19912,"ancestor_record_offset":17132,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":20012,"parent_record_offset":19944,"ancestor_record_offset":17132,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":19108,"parent_record_offset":19080,"ancestor_record_offset":17132,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":21464,"parent_record_offset":21436,"ancestor_record_offset":17132,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":17132,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":19136,"parent_record_offset":19108,"ancestor_record_offset":17132,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":19056,"parent_record_offset":19028,"ancestor_record_offset":17132,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":19080,"parent_record_offset":19056,"ancestor_record_offset":17132,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":19252,"parent_record_offset":19080,"ancestor_record_offset":17132,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":19364,"parent_record_offset":19340,"ancestor_record_offset":17132,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":475,"record_offset":19788,"parent_record_offset":19756,"ancestor_record_offset":17132,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":475,"record_offset":19184,"parent_record_offset":19160,"ancestor_record_offset":17132,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":475,"record_offset":17496,"parent_record_offset":17464,"ancestor_record_offset":17132,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":475,"record_offset":18924,"parent_record_offset":18880,"ancestor_record_offset":17132,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":20204,"parent_record_offset":20180,"ancestor_record_offset":17132,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":20244,"parent_record_offset":20064,"ancestor_record_offset":17132,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":20732,"parent_record_offset":19912,"ancestor_record_offset":17132,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":22124,"parent_record_offset":22096,"ancestor_record_offset":17132,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":475,"record_offset":17744,"parent_record_offset":17716,"ancestor_record_offset":17132,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":475,"record_offset":19712,"parent_record_offset":18880,"ancestor_record_offset":17132,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":19316,"parent_record_offset":19028,"ancestor_record_offset":17132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":21016,"parent_record_offset":20992,"ancestor_record_offset":17132,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":475,"record_offset":18208,"parent_record_offset":18184,"ancestor_record_offset":17132,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":475,"record_offset":18184,"parent_record_offset":18160,"ancestor_record_offset":17132,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":475,"record_offset":18392,"parent_record_offset":18364,"ancestor_record_offset":17132,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":19208,"parent_record_offset":19184,"ancestor_record_offset":17132,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":475,"record_offset":20116,"parent_record_offset":20088,"ancestor_record_offset":17132,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":20496,"parent_record_offset":20472,"ancestor_record_offset":17132,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":20152,"parent_record_offset":20116,"ancestor_record_offset":17132,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":20472,"parent_record_offset":20292,"ancestor_record_offset":17132,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":475,"record_offset":20992,"parent_record_offset":20820,"ancestor_record_offset":17132,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":17132,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":17132,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":18464,"parent_record_offset":18436,"ancestor_record_offset":17132,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":18112,"parent_record_offset":17996,"ancestor_record_offset":17132,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":475,"record_offset":17464,"parent_record_offset":17436,"ancestor_record_offset":17132,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":475,"record_offset":17824,"parent_record_offset":17796,"ancestor_record_offset":17132,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":475,"record_offset":17964,"parent_record_offset":17932,"ancestor_record_offset":17132,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":475,"record_offset":17932,"parent_record_offset":17896,"ancestor_record_offset":17132,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":475,"record_offset":17872,"parent_record_offset":17848,"ancestor_record_offset":17132,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":475,"record_offset":17896,"parent_record_offset":17872,"ancestor_record_offset":17132,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":475,"record_offset":17848,"parent_record_offset":17824,"ancestor_record_offset":17132,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":475,"record_offset":18436,"parent_record_offset":17896,"ancestor_record_offset":17132,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":17436,"parent_record_offset":17404,"ancestor_record_offset":17132,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":17404,"parent_record_offset":17376,"ancestor_record_offset":17132,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":17596,"parent_record_offset":17564,"ancestor_record_offset":17132,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":17692,"parent_record_offset":17596,"ancestor_record_offset":17132,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":17564,"parent_record_offset":17540,"ancestor_record_offset":17132,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":17376,"parent_record_offset":17340,"ancestor_record_offset":17132,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":17132,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":475,"record_offset":17996,"parent_record_offset":17964,"ancestor_record_offset":17132,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":475,"record_offset":18280,"parent_record_offset":17996,"ancestor_record_offset":17132,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":475,"record_offset":18252,"parent_record_offset":17996,"ancestor_record_offset":17132,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":475,"record_offset":18056,"parent_record_offset":17996,"ancestor_record_offset":17132,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":475,"record_offset":20764,"parent_record_offset":19880,"ancestor_record_offset":17132,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":475,"record_offset":21164,"parent_record_offset":21132,"ancestor_record_offset":17132,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":475,"record_offset":21212,"parent_record_offset":21164,"ancestor_record_offset":17132,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":475,"record_offset":19836,"parent_record_offset":19812,"ancestor_record_offset":17132,"inlinee":25979,"source_line":132,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":475,"record_offset":19812,"parent_record_offset":19788,"ancestor_record_offset":17132,"inlinee":25978,"source_line":171,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":475,"record_offset":19912,"parent_record_offset":19880,"ancestor_record_offset":17132,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":475,"record_offset":21784,"parent_record_offset":21760,"ancestor_record_offset":17132,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":18808,"parent_record_offset":18772,"ancestor_record_offset":17132,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":33232,"parent_record_offset":33200,"ancestor_record_offset":17132,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":33732,"parent_record_offset":33472,"ancestor_record_offset":17132,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":475,"record_offset":18952,"parent_record_offset":18924,"ancestor_record_offset":17132,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":19028,"parent_record_offset":18880,"ancestor_record_offset":17132,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":19288,"parent_record_offset":19028,"ancestor_record_offset":17132,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":21820,"parent_record_offset":21708,"ancestor_record_offset":17132,"inlinee":25993,"source_line":632,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":21844,"parent_record_offset":21820,"ancestor_record_offset":17132,"inlinee":25994,"source_line":632,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":21736,"parent_record_offset":21708,"ancestor_record_offset":17132,"inlinee":25991,"source_line":630,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":17132,"inlinee":25992,"source_line":630,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":21244,"parent_record_offset":21212,"ancestor_record_offset":17132,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":21308,"parent_record_offset":21276,"ancestor_record_offset":17132,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":21436,"parent_record_offset":21376,"ancestor_record_offset":17132,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":21276,"parent_record_offset":21244,"ancestor_record_offset":17132,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":475,"record_offset":18980,"parent_record_offset":18952,"ancestor_record_offset":17132,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":475,"record_offset":18880,"parent_record_offset":18844,"ancestor_record_offset":17132,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":475,"record_offset":18844,"parent_record_offset":18808,"ancestor_record_offset":17132,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2
// 记录形式: compiled_procedure; instances=1
// 源码行: 248
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":34324,"rva":8608320,"va":5377317440}
// 编译实例 RVAs: 0x835a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=5
//     target_aliases=35 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7717764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e87c3c8aff","size":5,"soft_fail":false,"target_va":5369600800,"va":5377318559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=5
//     target_aliases=19 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7702207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e841798aff","size":5,"soft_fail":false,"target_va":5369616112,"va":5377318314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t373578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e84ab30500","size":5,"soft_fail":false,"target_va":5377696704,"va":5377323121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t378083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e8e3c40500","size":5,"soft_fail":false,"target_va":5377701312,"va":5377323224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090b460
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t873634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e8a2540d00","size":5,"soft_fail":false,"target_va":5378192480,"va":5377318841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3868860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e8bc083b00","size":5,"soft_fail":false,"target_va":5381187408,"va":5377318543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34347516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e8fc190c02","size":5,"soft_fail":false,"target_va":5411669968,"va":5377322447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34528354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e862dc0e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377318809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34219074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e842240a02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377322505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t34219230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e8de240a02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377322541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e873ceffff","size":5,"soft_fail":false,"target_va":5377304864,"va":5377317544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34225741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e84d3e0a02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377317614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t851049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e869fc0c00","size":5,"soft_fail":false,"target_va":5378169824,"va":5377318770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t203763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e8f31b0300","size":5,"soft_fail":false,"target_va":5377522032,"va":5377318264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t205626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e83a230300","size":5,"soft_fail":false,"target_va":5377524592,"va":5377318961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4067465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e889103e00","size":5,"soft_fail":false,"target_va":5381385536,"va":5377318066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4072654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e8ce243e00","size":5,"soft_fail":false,"target_va":5381390640,"va":5377317981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4064077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e84d033e00","size":5,"soft_fail":false,"target_va":5381386480,"va":5377322398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4067190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e8760f3e00","size":5,"soft_fail":false,"target_va":5381384704,"va":5377317509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3799710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e89efa3900","size":5,"soft_fail":false,"target_va":5381117840,"va":5377318125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11502327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e8f782af00","size":5,"soft_fail":false,"target_va":5388825296,"va":5377322964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t34214606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377317440,"raw_hex":"e8ce120a02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377322493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377317440,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5377318908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":37984,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":38828,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":47352,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":38244,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::stencil_ref
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":475,"record_offset":38864,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":25988,"source_line":122,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":475,"record_offset":38320,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":25983,"source_line":32,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":475,"record_offset":36944,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":475,"record_offset":37068,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::main_pass::with_stencil_write
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
//     evidence={"module":475,"record_offset":38896,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":25990,"source_line":22,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":35960,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":34528,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":34984,"parent_record_offset":34324,"ancestor_record_offset":34324,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":37568,"parent_record_offset":37544,"ancestor_record_offset":34324,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":37616,"parent_record_offset":37592,"ancestor_record_offset":34324,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":37592,"parent_record_offset":37568,"ancestor_record_offset":34324,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":37544,"parent_record_offset":37520,"ancestor_record_offset":34324,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":37520,"parent_record_offset":37480,"ancestor_record_offset":34324,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":37480,"parent_record_offset":37456,"ancestor_record_offset":34324,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":37868,"parent_record_offset":37844,"ancestor_record_offset":34324,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":37844,"parent_record_offset":37820,"ancestor_record_offset":34324,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":37792,"parent_record_offset":37760,"ancestor_record_offset":34324,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":37736,"parent_record_offset":37200,"ancestor_record_offset":34324,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":37760,"parent_record_offset":37736,"ancestor_record_offset":34324,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":37228,"parent_record_offset":37200,"ancestor_record_offset":34324,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":37820,"parent_record_offset":37760,"ancestor_record_offset":34324,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":37276,"parent_record_offset":37252,"ancestor_record_offset":34324,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":37252,"parent_record_offset":37228,"ancestor_record_offset":34324,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":37456,"parent_record_offset":37432,"ancestor_record_offset":34324,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":37160,"parent_record_offset":37132,"ancestor_record_offset":34324,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":37132,"parent_record_offset":37100,"ancestor_record_offset":34324,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":37200,"parent_record_offset":37132,"ancestor_record_offset":34324,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":36296,"parent_record_offset":36268,"ancestor_record_offset":34324,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":38652,"parent_record_offset":38624,"ancestor_record_offset":34324,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":36348,"parent_record_offset":36324,"ancestor_record_offset":34324,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":36324,"parent_record_offset":36296,"ancestor_record_offset":34324,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":36244,"parent_record_offset":36216,"ancestor_record_offset":34324,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":36268,"parent_record_offset":36244,"ancestor_record_offset":34324,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":36440,"parent_record_offset":36268,"ancestor_record_offset":34324,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":36552,"parent_record_offset":36528,"ancestor_record_offset":34324,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":475,"record_offset":36976,"parent_record_offset":36944,"ancestor_record_offset":34324,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":475,"record_offset":36372,"parent_record_offset":36348,"ancestor_record_offset":34324,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":475,"record_offset":34684,"parent_record_offset":34652,"ancestor_record_offset":34324,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":475,"record_offset":36112,"parent_record_offset":36068,"ancestor_record_offset":34324,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":37392,"parent_record_offset":37368,"ancestor_record_offset":34324,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":37432,"parent_record_offset":37252,"ancestor_record_offset":34324,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":37920,"parent_record_offset":37100,"ancestor_record_offset":34324,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":39312,"parent_record_offset":39284,"ancestor_record_offset":34324,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":475,"record_offset":34932,"parent_record_offset":34904,"ancestor_record_offset":34324,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":475,"record_offset":36900,"parent_record_offset":36068,"ancestor_record_offset":34324,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":36504,"parent_record_offset":36216,"ancestor_record_offset":34324,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":38204,"parent_record_offset":38180,"ancestor_record_offset":34324,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":475,"record_offset":35396,"parent_record_offset":35372,"ancestor_record_offset":34324,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":475,"record_offset":35372,"parent_record_offset":35348,"ancestor_record_offset":34324,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":475,"record_offset":35580,"parent_record_offset":35552,"ancestor_record_offset":34324,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":36396,"parent_record_offset":36372,"ancestor_record_offset":34324,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":475,"record_offset":37304,"parent_record_offset":37276,"ancestor_record_offset":34324,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":37684,"parent_record_offset":37660,"ancestor_record_offset":34324,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":37340,"parent_record_offset":37304,"ancestor_record_offset":34324,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":37660,"parent_record_offset":37480,"ancestor_record_offset":34324,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":475,"record_offset":38180,"parent_record_offset":38008,"ancestor_record_offset":34324,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":35324,"parent_record_offset":35300,"ancestor_record_offset":34324,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":35348,"parent_record_offset":35324,"ancestor_record_offset":34324,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":35652,"parent_record_offset":35624,"ancestor_record_offset":34324,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":35300,"parent_record_offset":35184,"ancestor_record_offset":34324,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":475,"record_offset":34652,"parent_record_offset":34624,"ancestor_record_offset":34324,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":475,"record_offset":35012,"parent_record_offset":34984,"ancestor_record_offset":34324,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":475,"record_offset":35152,"parent_record_offset":35120,"ancestor_record_offset":34324,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":475,"record_offset":35120,"parent_record_offset":35084,"ancestor_record_offset":34324,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":475,"record_offset":35060,"parent_record_offset":35036,"ancestor_record_offset":34324,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":475,"record_offset":35084,"parent_record_offset":35060,"ancestor_record_offset":34324,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":475,"record_offset":35036,"parent_record_offset":35012,"ancestor_record_offset":34324,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":475,"record_offset":35624,"parent_record_offset":35084,"ancestor_record_offset":34324,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":34624,"parent_record_offset":34592,"ancestor_record_offset":34324,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":34592,"parent_record_offset":34564,"ancestor_record_offset":34324,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":34784,"parent_record_offset":34752,"ancestor_record_offset":34324,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":34880,"parent_record_offset":34784,"ancestor_record_offset":34324,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":34752,"parent_record_offset":34728,"ancestor_record_offset":34324,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":34564,"parent_record_offset":34528,"ancestor_record_offset":34324,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":34904,"parent_record_offset":34880,"ancestor_record_offset":34324,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":475,"record_offset":35184,"parent_record_offset":35152,"ancestor_record_offset":34324,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":475,"record_offset":35468,"parent_record_offset":35184,"ancestor_record_offset":34324,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":475,"record_offset":35440,"parent_record_offset":35184,"ancestor_record_offset":34324,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":475,"record_offset":35244,"parent_record_offset":35184,"ancestor_record_offset":34324,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":475,"record_offset":37952,"parent_record_offset":37068,"ancestor_record_offset":34324,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":475,"record_offset":38352,"parent_record_offset":38320,"ancestor_record_offset":34324,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":475,"record_offset":38400,"parent_record_offset":38352,"ancestor_record_offset":34324,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":475,"record_offset":37024,"parent_record_offset":37000,"ancestor_record_offset":34324,"inlinee":25979,"source_line":132,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":475,"record_offset":37000,"parent_record_offset":36976,"ancestor_record_offset":34324,"inlinee":25978,"source_line":171,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":475,"record_offset":37100,"parent_record_offset":37068,"ancestor_record_offset":34324,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":475,"record_offset":38972,"parent_record_offset":38948,"ancestor_record_offset":34324,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":35996,"parent_record_offset":35960,"ancestor_record_offset":34324,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":47416,"parent_record_offset":47384,"ancestor_record_offset":34324,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":47916,"parent_record_offset":47656,"ancestor_record_offset":34324,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":475,"record_offset":36140,"parent_record_offset":36112,"ancestor_record_offset":34324,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":36216,"parent_record_offset":36068,"ancestor_record_offset":34324,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":475,"record_offset":36476,"parent_record_offset":36216,"ancestor_record_offset":34324,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":39008,"parent_record_offset":38896,"ancestor_record_offset":34324,"inlinee":25993,"source_line":632,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":39032,"parent_record_offset":39008,"ancestor_record_offset":34324,"inlinee":25994,"source_line":632,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":38924,"parent_record_offset":38896,"ancestor_record_offset":34324,"inlinee":25991,"source_line":630,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":38948,"parent_record_offset":38924,"ancestor_record_offset":34324,"inlinee":25992,"source_line":630,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":38432,"parent_record_offset":38400,"ancestor_record_offset":34324,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":38496,"parent_record_offset":38464,"ancestor_record_offset":34324,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":38624,"parent_record_offset":38564,"ancestor_record_offset":34324,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":38464,"parent_record_offset":38432,"ancestor_record_offset":34324,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":475,"record_offset":36168,"parent_record_offset":36140,"ancestor_record_offset":34324,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":475,"record_offset":36068,"parent_record_offset":36032,"ancestor_record_offset":34324,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":475,"record_offset":36032,"parent_record_offset":35996,"ancestor_record_offset":34324,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 458
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":48508,"rva":8614832,"va":5377323952}
// 编译实例 RVAs: 0x8373b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7723652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e87c258aff","size":5,"soft_fail":false,"target_va":5369600800,"va":5377324447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=19 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7708111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e831628aff","size":5,"soft_fail":false,"target_va":5369616112,"va":5377324218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t370721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e821a80500","size":5,"soft_fail":false,"target_va":5377696704,"va":5377325978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t375416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e878ba0500","size":5,"soft_fail":false,"target_va":5377701312,"va":5377325891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14090b460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t867749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e8a53d0d00","size":5,"soft_fail":false,"target_va":5378192480,"va":5377324726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3862972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e8bcf13a00","size":5,"soft_fail":false,"target_va":5381187408,"va":5377324431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: main_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34522469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e865c50e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377324694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34216012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e84c180a02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377325759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t845164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e86ce50c00","size":5,"soft_fail":false,"target_va":5378169824,"va":5377324655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t197845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e8d5040300","size":5,"soft_fail":false,"target_va":5377522032,"va":5377324182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t199744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e8400c0300","size":5,"soft_fail":false,"target_va":5377524592,"va":5377324843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3793778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e872e33900","size":5,"soft_fail":false,"target_va":5381117840,"va":5377324057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34211354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377323952,"raw_hex":"e81a060a02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377325745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5377323952,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5377324793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":50472,"parent_record_offset":48508,"ancestor_record_offset":48508,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":475,"record_offset":48716,"parent_record_offset":48508,"ancestor_record_offset":48508,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":49888,"parent_record_offset":48508,"ancestor_record_offset":48508,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":475,"record_offset":49776,"parent_record_offset":48508,"ancestor_record_offset":48508,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::stencil_ref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":475,"record_offset":50508,"parent_record_offset":48508,"ancestor_record_offset":48508,"inlinee":25988,"source_line":122,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":475,"record_offset":49964,"parent_record_offset":48508,"ancestor_record_offset":48508,"inlinee":25983,"source_line":32,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":475,"record_offset":48744,"parent_record_offset":48508,"ancestor_record_offset":48508,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":475,"record_offset":48864,"parent_record_offset":48508,"ancestor_record_offset":48508,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::main_pass::with_stencil_write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
//     evidence={"module":475,"record_offset":50540,"parent_record_offset":48508,"ancestor_record_offset":48508,"inlinee":25990,"source_line":22,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":49360,"parent_record_offset":49336,"ancestor_record_offset":48508,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":49408,"parent_record_offset":49384,"ancestor_record_offset":48508,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":49384,"parent_record_offset":49360,"ancestor_record_offset":48508,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":49336,"parent_record_offset":49312,"ancestor_record_offset":48508,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":49312,"parent_record_offset":49272,"ancestor_record_offset":48508,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":49272,"parent_record_offset":49248,"ancestor_record_offset":48508,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":49660,"parent_record_offset":49636,"ancestor_record_offset":48508,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":49636,"parent_record_offset":49612,"ancestor_record_offset":48508,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":475,"record_offset":49584,"parent_record_offset":49552,"ancestor_record_offset":48508,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":49528,"parent_record_offset":48992,"ancestor_record_offset":48508,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":49552,"parent_record_offset":49528,"ancestor_record_offset":48508,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":49020,"parent_record_offset":48992,"ancestor_record_offset":48508,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":475,"record_offset":49612,"parent_record_offset":49552,"ancestor_record_offset":48508,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":49068,"parent_record_offset":49044,"ancestor_record_offset":48508,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":49044,"parent_record_offset":49020,"ancestor_record_offset":48508,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":475,"record_offset":49248,"parent_record_offset":49224,"ancestor_record_offset":48508,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":48952,"parent_record_offset":48924,"ancestor_record_offset":48508,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":48924,"parent_record_offset":48896,"ancestor_record_offset":48508,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":475,"record_offset":48992,"parent_record_offset":48924,"ancestor_record_offset":48508,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":50296,"parent_record_offset":50268,"ancestor_record_offset":48508,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":50348,"parent_record_offset":50324,"ancestor_record_offset":48508,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":50324,"parent_record_offset":50268,"ancestor_record_offset":48508,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":475,"record_offset":48776,"parent_record_offset":48744,"ancestor_record_offset":48508,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":475,"record_offset":50372,"parent_record_offset":50348,"ancestor_record_offset":48508,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":475,"record_offset":49160,"parent_record_offset":49096,"ancestor_record_offset":48508,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":49184,"parent_record_offset":49160,"ancestor_record_offset":48508,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":49224,"parent_record_offset":49044,"ancestor_record_offset":48508,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":475,"record_offset":49712,"parent_record_offset":48896,"ancestor_record_offset":48508,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":475,"record_offset":50240,"parent_record_offset":50208,"ancestor_record_offset":48508,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":49848,"parent_record_offset":49824,"ancestor_record_offset":48508,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":475,"record_offset":49928,"parent_record_offset":49888,"ancestor_record_offset":48508,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":50396,"parent_record_offset":50372,"ancestor_record_offset":48508,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":475,"record_offset":49096,"parent_record_offset":49068,"ancestor_record_offset":48508,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":475,"record_offset":49800,"parent_record_offset":49776,"ancestor_record_offset":48508,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":49476,"parent_record_offset":49452,"ancestor_record_offset":48508,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":49132,"parent_record_offset":49096,"ancestor_record_offset":48508,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":475,"record_offset":49452,"parent_record_offset":49272,"ancestor_record_offset":48508,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":475,"record_offset":49824,"parent_record_offset":49800,"ancestor_record_offset":48508,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":475,"record_offset":49744,"parent_record_offset":48864,"ancestor_record_offset":48508,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":475,"record_offset":49996,"parent_record_offset":49964,"ancestor_record_offset":48508,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":475,"record_offset":50044,"parent_record_offset":49996,"ancestor_record_offset":48508,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":475,"record_offset":48824,"parent_record_offset":48800,"ancestor_record_offset":48508,"inlinee":25979,"source_line":132,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":475,"record_offset":48800,"parent_record_offset":48776,"ancestor_record_offset":48508,"inlinee":25978,"source_line":171,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":475,"record_offset":48896,"parent_record_offset":48864,"ancestor_record_offset":48508,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":475,"record_offset":50616,"parent_record_offset":50592,"ancestor_record_offset":48508,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":50652,"parent_record_offset":50540,"ancestor_record_offset":48508,"inlinee":25993,"source_line":632,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":50676,"parent_record_offset":50652,"ancestor_record_offset":48508,"inlinee":25994,"source_line":632,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":50568,"parent_record_offset":50540,"ancestor_record_offset":48508,"inlinee":25991,"source_line":630,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":475,"record_offset":50592,"parent_record_offset":50568,"ancestor_record_offset":48508,"inlinee":25992,"source_line":630,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":50076,"parent_record_offset":50044,"ancestor_record_offset":48508,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":50140,"parent_record_offset":50108,"ancestor_record_offset":48508,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":50268,"parent_record_offset":50208,"ancestor_record_offset":48508,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":475,"record_offset":50108,"parent_record_offset":50076,"ancestor_record_offset":48508,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":475,"record_offset":50208,"parent_record_offset":50172,"ancestor_record_offset":48508,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":475,"record_offset":50172,"parent_record_offset":50140,"ancestor_record_offset":48508,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00475.jsonl.gz"}
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
// 完整结构及原始类型记录: main_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: main_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/main_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: main_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5162373,"position_among_mapped_render_targets":30,"mapped_render_target_count":59,"previous_callsite":5161973,"next_callsite":5162412,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/generate_hzb.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5162500,"position_among_mapped_render_targets":35,"mapped_render_target_count":59,"previous_callsite":5162471,"next_callsite":5162521,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/grass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/visibility_queries.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5162571,"position_among_mapped_render_targets":39,"mapped_render_target_count":59,"previous_callsite":5162549,"next_callsite":5162592,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/mouse_depth_value.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/half_res.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: main_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass
// 信息 ID: 280b8bb043ca644b547a8aee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec64b -> 0x8373b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass
// 信息 ID: d443c6d16143621594f4719c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec585 -> 0x833bc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2
// 信息 ID: 0bee9d9b8e3e496ea151c452
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec604 -> 0x835a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::main_pass::with_stencil_write
// 信息 ID: 142f24071df50622e56bd128
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=21708; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=23848; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=25988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=28992; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=475, inline_record=31132; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=38896; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=41036; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=475, inline_record=43176; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
