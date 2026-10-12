// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/wireframe_pass.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a6bfd1ef51405973d682a03421f3e3666b2b7eb1e421826f1049d28c4ccd3179
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wireframe_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 487
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"PDB_registry_path_only","version":"0.11.2"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":487,"record_offset":27528,"rva":6957120,"va":5375666240}
// 编译实例 RVAs: 0x6a2840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3041521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e80f97d1ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375666540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ae440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t47595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e8ebb90000","size":5,"soft_fail":false,"target_va":5375714368,"va":5375666768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t296417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e8e1850400","size":5,"soft_fail":false,"target_va":5375963408,"va":5375666986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t556444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e89c7d0800","size":5,"soft_fail":false,"target_va":5376222944,"va":5375666495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wireframe_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27926542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e80e20aa01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375666813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t33943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e897840000","size":5,"soft_fail":false,"target_va":5375700688,"va":5375666740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-270855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e8f9ddfbff","size":5,"soft_fail":false,"target_va":5375395536,"va":5375666386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-268653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e893e6fbff","size":5,"soft_fail":false,"target_va":5375398240,"va":5375666888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-747439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e85198f4ff","size":5,"soft_fail":false,"target_va":5374918976,"va":5375666410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t675461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e8854e0a00","size":5,"soft_fail":false,"target_va":5376341760,"va":5375666294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1699716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375666240,"raw_hex":"e884ef1900","size":5,"soft_fail":false,"target_va":5377366224,"va":5375666503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375666240,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5375666838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":27888,"parent_record_offset":27528,"ancestor_record_offset":27528,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":487,"record_offset":28000,"parent_record_offset":27528,"ancestor_record_offset":27528,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":487,"record_offset":28080,"parent_record_offset":27528,"ancestor_record_offset":27528,"inlinee":21984,"source_line":32,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":27712,"parent_record_offset":27528,"ancestor_record_offset":27528,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":28724,"parent_record_offset":27528,"ancestor_record_offset":27528,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":28640,"parent_record_offset":27528,"ancestor_record_offset":27528,"inlinee":27432,"source_line":627,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":28464,"parent_record_offset":28436,"ancestor_record_offset":27528,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":28516,"parent_record_offset":28492,"ancestor_record_offset":27528,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":28492,"parent_record_offset":28436,"ancestor_record_offset":27528,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":487,"record_offset":27744,"parent_record_offset":27712,"ancestor_record_offset":27528,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":487,"record_offset":28540,"parent_record_offset":28516,"ancestor_record_offset":27528,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":28392,"parent_record_offset":28368,"ancestor_record_offset":27528,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":487,"record_offset":27960,"parent_record_offset":27936,"ancestor_record_offset":27528,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":487,"record_offset":28044,"parent_record_offset":28000,"ancestor_record_offset":27528,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":28564,"parent_record_offset":28540,"ancestor_record_offset":27528,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":487,"record_offset":27912,"parent_record_offset":27888,"ancestor_record_offset":27528,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":487,"record_offset":27936,"parent_record_offset":27912,"ancestor_record_offset":27528,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":28104,"parent_record_offset":28080,"ancestor_record_offset":27528,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":28144,"parent_record_offset":28104,"ancestor_record_offset":27528,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":27816,"parent_record_offset":27792,"ancestor_record_offset":27528,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":27792,"parent_record_offset":27768,"ancestor_record_offset":27528,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":27768,"parent_record_offset":27744,"ancestor_record_offset":27528,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":28688,"parent_record_offset":28664,"ancestor_record_offset":27528,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":28748,"parent_record_offset":28724,"ancestor_record_offset":27528,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":27528,"inlinee":27433,"source_line":627,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":487,"record_offset":27840,"parent_record_offset":27816,"ancestor_record_offset":27528,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":487,"record_offset":28172,"parent_record_offset":28144,"ancestor_record_offset":27528,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":28228,"parent_record_offset":28200,"ancestor_record_offset":27528,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":28316,"parent_record_offset":28284,"ancestor_record_offset":27528,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":28436,"parent_record_offset":28284,"ancestor_record_offset":27528,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":28200,"parent_record_offset":28172,"ancestor_record_offset":27528,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":28340,"parent_record_offset":28316,"ancestor_record_offset":27528,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":487,"record_offset":28368,"parent_record_offset":28340,"ancestor_record_offset":27528,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":487,"record_offset":28284,"parent_record_offset":28256,"ancestor_record_offset":27528,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":487,"record_offset":28256,"parent_record_offset":28228,"ancestor_record_offset":27528,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00487.jsonl.gz"}
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
// 完整结构及原始类型记录: wireframe_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wireframe_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/wireframe_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: wireframe_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3144582,"position_among_mapped_render_targets":63,"mapped_render_target_count":70,"previous_callsite":3144533,"next_callsite":3144622,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/feedback_report_bg.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wireframe_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass
// 信息 ID: 1d6419815575e18df48b75fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffb86 -> 0x6a2840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
