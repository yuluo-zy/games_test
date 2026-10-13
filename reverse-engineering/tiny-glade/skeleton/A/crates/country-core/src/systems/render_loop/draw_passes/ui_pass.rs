// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 50d16ca2839c741b40eeee2fd0e3a985fba393dbdda6851ce913f44178a2d48d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_pass.rs.pdb.jsonl.gz
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
// 函数符号: country_core::systems::render_loop::draw_passes::ui_pass::ui_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":487,"record_offset":25140,"rva":6955616,"va":5375664736}
// 编译实例 RVAs: 0x6a2260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: ui_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3040017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e8ef9cd1ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375665036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ae440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t49099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e8cbbf0000","size":5,"soft_fail":false,"target_va":5375714368,"va":5375665264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t297937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e8d18b0400","size":5,"soft_fail":false,"target_va":5375963408,"va":5375665466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t557948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e87c830800","size":5,"soft_fail":false,"target_va":5376222944,"va":5375664991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27928053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e8f525aa01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375665302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t35447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e8778a0000","size":5,"soft_fail":false,"target_va":5375700688,"va":5375665236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-269351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e8d9e3fbff","size":5,"soft_fail":false,"target_va":5375395536,"va":5375664882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-267142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e87aecfbff","size":5,"soft_fail":false,"target_va":5375398240,"va":5375665377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-745935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e8319ef4ff","size":5,"soft_fail":false,"target_va":5374918976,"va":5375664906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t676965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e865540a00","size":5,"soft_fail":false,"target_va":5376341760,"va":5375664790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1701220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375664736,"raw_hex":"e864f51900","size":5,"soft_fail":false,"target_va":5377366224,"va":5375664999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375664736,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5375665327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":25484,"parent_record_offset":25140,"ancestor_record_offset":25140,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":487,"record_offset":25596,"parent_record_offset":25140,"ancestor_record_offset":25140,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":487,"record_offset":25676,"parent_record_offset":25140,"ancestor_record_offset":25140,"inlinee":21984,"source_line":32,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":25308,"parent_record_offset":25140,"ancestor_record_offset":25140,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":26236,"parent_record_offset":25140,"ancestor_record_offset":25140,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":26060,"parent_record_offset":26032,"ancestor_record_offset":25140,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":26112,"parent_record_offset":26088,"ancestor_record_offset":25140,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":26088,"parent_record_offset":26032,"ancestor_record_offset":25140,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":487,"record_offset":25340,"parent_record_offset":25308,"ancestor_record_offset":25140,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":487,"record_offset":26136,"parent_record_offset":26112,"ancestor_record_offset":25140,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":25988,"parent_record_offset":25964,"ancestor_record_offset":25140,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":487,"record_offset":25556,"parent_record_offset":25532,"ancestor_record_offset":25140,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":487,"record_offset":25640,"parent_record_offset":25596,"ancestor_record_offset":25140,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":26160,"parent_record_offset":26136,"ancestor_record_offset":25140,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":487,"record_offset":25508,"parent_record_offset":25484,"ancestor_record_offset":25140,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":487,"record_offset":25532,"parent_record_offset":25508,"ancestor_record_offset":25140,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":25700,"parent_record_offset":25676,"ancestor_record_offset":25140,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":25740,"parent_record_offset":25700,"ancestor_record_offset":25140,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":25412,"parent_record_offset":25388,"ancestor_record_offset":25140,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":25388,"parent_record_offset":25364,"ancestor_record_offset":25140,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":25364,"parent_record_offset":25340,"ancestor_record_offset":25140,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":26284,"parent_record_offset":26260,"ancestor_record_offset":25140,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":26260,"parent_record_offset":26236,"ancestor_record_offset":25140,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":487,"record_offset":25436,"parent_record_offset":25412,"ancestor_record_offset":25140,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":487,"record_offset":25768,"parent_record_offset":25740,"ancestor_record_offset":25140,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":25824,"parent_record_offset":25796,"ancestor_record_offset":25140,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":25912,"parent_record_offset":25880,"ancestor_record_offset":25140,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":26032,"parent_record_offset":25880,"ancestor_record_offset":25140,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":25796,"parent_record_offset":25768,"ancestor_record_offset":25140,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":25936,"parent_record_offset":25912,"ancestor_record_offset":25140,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":487,"record_offset":25964,"parent_record_offset":25936,"ancestor_record_offset":25140,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":487,"record_offset":25880,"parent_record_offset":25852,"ancestor_record_offset":25140,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":487,"record_offset":25852,"parent_record_offset":25824,"ancestor_record_offset":25140,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":487,"record_offset":26328,"rva":6956368,"va":5375665488}
// 编译实例 RVAs: 0x6a2550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: ui_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3040769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e8ff99d1ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375665788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ae440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t48347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e8dbbc0000","size":5,"soft_fail":false,"target_va":5375714368,"va":5375666016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t297185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e8e1880400","size":5,"soft_fail":false,"target_va":5375963408,"va":5375666218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t557196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e88c800800","size":5,"soft_fail":false,"target_va":5376222944,"va":5375665743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27927301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e80523aa01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375666054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t34695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e887870000","size":5,"soft_fail":false,"target_va":5375700688,"va":5375665988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-270103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e8e9e0fbff","size":5,"soft_fail":false,"target_va":5375395536,"va":5375665634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-267894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e88ae9fbff","size":5,"soft_fail":false,"target_va":5375398240,"va":5375666129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-746687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e8419bf4ff","size":5,"soft_fail":false,"target_va":5374918976,"va":5375665658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t676213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e875510a00","size":5,"soft_fail":false,"target_va":5376341760,"va":5375665542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1700468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375665488,"raw_hex":"e874f21900","size":5,"soft_fail":false,"target_va":5377366224,"va":5375665751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rsi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375665488,"raw_hex":"ffd6","size":2,"soft_fail":false,"va":5375666079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":26684,"parent_record_offset":26328,"ancestor_record_offset":26328,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":487,"record_offset":26796,"parent_record_offset":26328,"ancestor_record_offset":26328,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_list.rs
//     evidence={"module":487,"record_offset":26876,"parent_record_offset":26328,"ancestor_record_offset":26328,"inlinee":21984,"source_line":32,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":26508,"parent_record_offset":26328,"ancestor_record_offset":26328,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":27436,"parent_record_offset":26328,"ancestor_record_offset":26328,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":27260,"parent_record_offset":27232,"ancestor_record_offset":26328,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":27312,"parent_record_offset":27288,"ancestor_record_offset":26328,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":487,"record_offset":27288,"parent_record_offset":27232,"ancestor_record_offset":26328,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":487,"record_offset":26540,"parent_record_offset":26508,"ancestor_record_offset":26328,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":487,"record_offset":27336,"parent_record_offset":27312,"ancestor_record_offset":26328,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":487,"record_offset":27188,"parent_record_offset":27164,"ancestor_record_offset":26328,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":487,"record_offset":26756,"parent_record_offset":26732,"ancestor_record_offset":26328,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":487,"record_offset":26840,"parent_record_offset":26796,"ancestor_record_offset":26328,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":27360,"parent_record_offset":27336,"ancestor_record_offset":26328,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":487,"record_offset":26708,"parent_record_offset":26684,"ancestor_record_offset":26328,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":487,"record_offset":26732,"parent_record_offset":26708,"ancestor_record_offset":26328,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":26900,"parent_record_offset":26876,"ancestor_record_offset":26328,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":487,"record_offset":26940,"parent_record_offset":26900,"ancestor_record_offset":26328,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":26612,"parent_record_offset":26588,"ancestor_record_offset":26328,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":26588,"parent_record_offset":26564,"ancestor_record_offset":26328,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":487,"record_offset":26564,"parent_record_offset":26540,"ancestor_record_offset":26328,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":27484,"parent_record_offset":27460,"ancestor_record_offset":26328,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":27460,"parent_record_offset":27436,"ancestor_record_offset":26328,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":487,"record_offset":26636,"parent_record_offset":26612,"ancestor_record_offset":26328,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":487,"record_offset":26968,"parent_record_offset":26940,"ancestor_record_offset":26328,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":27024,"parent_record_offset":26996,"ancestor_record_offset":26328,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":27112,"parent_record_offset":27080,"ancestor_record_offset":26328,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":27232,"parent_record_offset":27080,"ancestor_record_offset":26328,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":487,"record_offset":26996,"parent_record_offset":26968,"ancestor_record_offset":26328,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":487,"record_offset":27136,"parent_record_offset":27112,"ancestor_record_offset":26328,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":487,"record_offset":27164,"parent_record_offset":27136,"ancestor_record_offset":26328,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":487,"record_offset":27080,"parent_record_offset":27052,"ancestor_record_offset":26328,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":487,"record_offset":27052,"parent_record_offset":27024,"ancestor_record_offset":26328,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00487.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: ui_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3144622,"position_among_mapped_render_targets":64,"mapped_render_target_count":70,"previous_callsite":3144582,"next_callsite":3144655,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/wireframe_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3145032,"position_among_mapped_render_targets":67,"mapped_render_target_count":70,"previous_callsite":3144710,"next_callsite":3145617,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/final_post.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3152112,"caller_aliases":{"alias_group_rva":3152112,"count":1,"display_examples":["tiny_glade::render_loop::render"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3159297,"position_among_mapped_render_targets":8,"mapped_render_target_count":9,"previous_callsite":3159251,"next_callsite":3159954,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass
// 信息 ID: ad3e3139f22bf7afc64c3231
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffd48 -> 0x6a2550 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x303501 -> 0x6a2550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_pass::ui_pass
// 信息 ID: 4ebe6f5a68e35f55e0ae43e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffbae -> 0x6a2260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
