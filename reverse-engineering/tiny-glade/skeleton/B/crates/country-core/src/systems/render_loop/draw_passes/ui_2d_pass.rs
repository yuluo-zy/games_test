// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 7076f06699febfa81a87dad67566ea2840aaac1e846d096c776b1cc3f94f5781
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_2d_pass.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 529, 531, 574, 1592, 1654, 1736, 3208
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
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":49524,"rva":10241136,"va":5378950256}
// 编译实例 RVAs: 0x9c4470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=3
//     target_aliases=35 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9350988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8b45071ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378951783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=5
//     target_aliases=19 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9334813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8e38f71ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378950920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0be0
//     owner=unresolved_namespace; count=4
//     target_aliases=251 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4144518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e87ac2c0ff","size":5,"soft_fail":false,"target_va":5374807008,"va":5378951521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3872375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e889e9c4ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378953874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1260776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e818c3ecff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378957475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=20
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1255627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e835d7ecff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378956934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-772059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e82538f4ff","size":5,"soft_fail":false,"target_va":5378180624,"va":5378952678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-368854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e82a5ffaff","size":5,"soft_fail":false,"target_va":5378585968,"va":5378954817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-368700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8c45ffaff","size":5,"soft_fail":false,"target_va":5378586208,"va":5378954903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-365673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8976bfaff","size":5,"soft_fail":false,"target_va":5378586704,"va":5378952372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b940
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-365113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8c76dfaff","size":5,"soft_fail":false,"target_va":5378586944,"va":5378952052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ba30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-365117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8c36dfaff","size":5,"soft_fail":false,"target_va":5378587184,"va":5378952296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b4be30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1598132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8b4621800","size":5,"soft_fail":false,"target_va":5380554288,"va":5378956151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2235630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8ee1c2200","size":5,"soft_fail":false,"target_va":5381187408,"va":5378951773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141305880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9700560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8d0049400","size":5,"soft_fail":false,"target_va":5388654720,"va":5378954155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141306990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9707588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e844209400","size":5,"soft_fail":false,"target_va":5388659088,"va":5378951495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413386d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9907287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8572c9700","size":5,"soft_fail":false,"target_va":5388863184,"va":5378955892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141657e90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13186544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8f035c900","size":5,"soft_fail":false,"target_va":5392137872,"va":5378951323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32714191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8cf2df301","size":5,"soft_fail":false,"target_va":5411669968,"va":5378955772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32895690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8caf2f501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378951473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14488494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8ae13dd00","size":5,"soft_fail":false,"target_va":5393439888,"va":5378951389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13191302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e88648c900","size":5,"soft_fail":false,"target_va":5392142608,"va":5378951301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::validate_world::panic_mismatched
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t32544697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8b997f001","size":5,"soft_fail":false,"target_va":5411500544,"va":5378955842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32585749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e81538f101","size":5,"soft_fail":false,"target_va":5411541584,"va":5378955830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t32585794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e84238f101","size":5,"soft_fail":false,"target_va":5411541776,"va":5378955977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-24146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8aea1ffff","size":5,"soft_fail":false,"target_va":5378926208,"va":5378950349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32592936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e82854f101","size":5,"soft_fail":false,"target_va":5411543360,"va":5378950419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-576718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e83233f7ff","size":5,"soft_fail":false,"target_va":5378374304,"va":5378951017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-780240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e83018f4ff","size":5,"soft_fail":false,"target_va":5378173584,"va":5378953819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-782170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8a610f4ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378951989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1429650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e86e2feaff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378951677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1431058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8ee29eaff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378955645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2434675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e873262500","size":5,"soft_fail":false,"target_va":5381385536,"va":5378950856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2439864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8b83a2500","size":5,"soft_fail":false,"target_va":5381390640,"va":5378950771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2430759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e827172500","size":5,"soft_fail":false,"target_va":5381386480,"va":5378955716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2434382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e84e252500","size":5,"soft_fail":false,"target_va":5381384704,"va":5378950317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2166260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8f40d2100","size":5,"soft_fail":false,"target_va":5381117840,"va":5378951575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_layer
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2165755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8fb0b2100","size":5,"soft_fail":false,"target_va":5381118304,"va":5378952544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::layer_count
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2165785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8190c2100","size":5,"soft_fail":false,"target_va":5381118368,"va":5378952578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::view_type
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2165393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8910a2100","size":5,"soft_fail":false,"target_va":5381117920,"va":5378952522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9868945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e891969600","size":5,"soft_fail":false,"target_va":5388825296,"va":5378956346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t32581281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e8a126f101","size":5,"soft_fail":false,"target_va":5411537104,"va":5378955818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2982042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378950256,"raw_hex":"e89a802d00","size":5,"soft_fail":false,"target_va":5381934464,"va":5378952417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":529,"record_offset":55308,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":55668,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":529,"record_offset":55724,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::state::QueryState::iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":529,"record_offset":54684,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":30255,"source_line":1185,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::query
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":529,"record_offset":54628,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":30253,"source_line":1535,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":52188,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":529,"record_offset":55224,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":57516,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":57544,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":52976,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":57868,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":57652,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture_view
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":56712,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":30290,"source_line":538,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":56324,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":55852,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":55488,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":529,"record_offset":54600,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":26000,"source_line":45,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::common_draw_params::CommonDrawParams::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs
//     evidence={"module":529,"record_offset":59836,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":25981,"source_line":26,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":51204,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":49772,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":50228,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":57780,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":57696,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":8285,"source_line":628,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$40::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":529,"record_offset":56684,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":26711,"source_line":251,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":58124,"parent_record_offset":58100,"ancestor_record_offset":49524,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":529,"record_offset":58100,"parent_record_offset":58012,"ancestor_record_offset":49524,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":52624,"parent_record_offset":52600,"ancestor_record_offset":49524,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":58040,"parent_record_offset":58012,"ancestor_record_offset":49524,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":58012,"parent_record_offset":57988,"ancestor_record_offset":49524,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":52672,"parent_record_offset":52648,"ancestor_record_offset":49524,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":52648,"parent_record_offset":52624,"ancestor_record_offset":49524,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":57988,"parent_record_offset":57964,"ancestor_record_offset":49524,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":529,"record_offset":55344,"parent_record_offset":55308,"ancestor_record_offset":49524,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":52600,"parent_record_offset":52576,"ancestor_record_offset":49524,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":52576,"parent_record_offset":52536,"ancestor_record_offset":49524,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":55248,"parent_record_offset":55224,"ancestor_record_offset":49524,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":52536,"parent_record_offset":52512,"ancestor_record_offset":49524,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":529,"record_offset":57892,"parent_record_offset":57868,"ancestor_record_offset":49524,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":529,"record_offset":57940,"parent_record_offset":57916,"ancestor_record_offset":49524,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":529,"record_offset":55144,"parent_record_offset":55120,"ancestor_record_offset":49524,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":529,"record_offset":58660,"parent_record_offset":58636,"ancestor_record_offset":49524,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":529,"record_offset":54716,"parent_record_offset":54684,"ancestor_record_offset":49524,"inlinee":30256,"source_line":326,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query_manual
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":529,"record_offset":54776,"parent_record_offset":54716,"ancestor_record_offset":49524,"inlinee":30258,"source_line":342,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query_unchecked_manual
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":529,"record_offset":54912,"parent_record_offset":54776,"ancestor_record_offset":49524,"inlinee":30259,"source_line":392,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::update_archetypes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":529,"record_offset":54748,"parent_record_offset":54716,"ancestor_record_offset":49524,"inlinee":30257,"source_line":512,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":52924,"parent_record_offset":52900,"ancestor_record_offset":49524,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":52900,"parent_record_offset":52876,"ancestor_record_offset":49524,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":52848,"parent_record_offset":52816,"ancestor_record_offset":49524,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":52792,"parent_record_offset":52256,"ancestor_record_offset":49524,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":52816,"parent_record_offset":52792,"ancestor_record_offset":49524,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":52284,"parent_record_offset":52256,"ancestor_record_offset":49524,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":52876,"parent_record_offset":52816,"ancestor_record_offset":49524,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":52332,"parent_record_offset":52308,"ancestor_record_offset":49524,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":52308,"parent_record_offset":52284,"ancestor_record_offset":49524,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":52512,"parent_record_offset":52488,"ancestor_record_offset":49524,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":529,"record_offset":55120,"parent_record_offset":54684,"ancestor_record_offset":49524,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":529,"record_offset":54812,"parent_record_offset":54776,"ancestor_record_offset":49524,"inlinee":4113,"source_line":187,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::last_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":529,"record_offset":54964,"parent_record_offset":54940,"ancestor_record_offset":49524,"inlinee":9251,"source_line":3001,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::query_filtered
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":529,"record_offset":54652,"parent_record_offset":54628,"ancestor_record_offset":49524,"inlinee":30254,"source_line":1559,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":529,"record_offset":55020,"parent_record_offset":54996,"ancestor_record_offset":49524,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::identifier::impl$9::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/identifier.rs
//     evidence={"module":529,"record_offset":54876,"parent_record_offset":54852,"ancestor_record_offset":49524,"inlinee":4117,"source_line":11,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":55184,"parent_record_offset":55144,"ancestor_record_offset":49524,"inlinee":13723,"source_line":263,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":54996,"parent_record_offset":54912,"ancestor_record_offset":49524,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":52216,"parent_record_offset":52188,"ancestor_record_offset":49524,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":59892,"parent_record_offset":59864,"ancestor_record_offset":49524,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":52256,"parent_record_offset":52188,"ancestor_record_offset":49524,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::last_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":54940,"parent_record_offset":54912,"ancestor_record_offset":49524,"inlinee":9250,"source_line":326,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":51540,"parent_record_offset":51512,"ancestor_record_offset":49524,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":56152,"parent_record_offset":56124,"ancestor_record_offset":49524,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":51592,"parent_record_offset":51568,"ancestor_record_offset":49524,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":51568,"parent_record_offset":51540,"ancestor_record_offset":49524,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":51488,"parent_record_offset":51460,"ancestor_record_offset":49524,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":51512,"parent_record_offset":51488,"ancestor_record_offset":49524,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":51684,"parent_record_offset":51512,"ancestor_record_offset":49524,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":51796,"parent_record_offset":51772,"ancestor_record_offset":49524,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":529,"record_offset":54852,"parent_record_offset":54812,"ancestor_record_offset":49524,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":529,"record_offset":55520,"parent_record_offset":55488,"ancestor_record_offset":49524,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":529,"record_offset":55420,"parent_record_offset":55380,"ancestor_record_offset":49524,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":529,"record_offset":55272,"parent_record_offset":55248,"ancestor_record_offset":49524,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":529,"record_offset":51616,"parent_record_offset":51592,"ancestor_record_offset":49524,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":529,"record_offset":58068,"parent_record_offset":58040,"ancestor_record_offset":49524,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":529,"record_offset":49928,"parent_record_offset":49896,"ancestor_record_offset":49524,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":51356,"parent_record_offset":51312,"ancestor_record_offset":49524,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":56364,"parent_record_offset":56324,"ancestor_record_offset":49524,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":52448,"parent_record_offset":52424,"ancestor_record_offset":49524,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":52488,"parent_record_offset":52308,"ancestor_record_offset":49524,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":60520,"parent_record_offset":59864,"ancestor_record_offset":49524,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":53040,"parent_record_offset":53012,"ancestor_record_offset":49524,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":50176,"parent_record_offset":50148,"ancestor_record_offset":49524,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":52144,"parent_record_offset":51312,"ancestor_record_offset":49524,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":51748,"parent_record_offset":51460,"ancestor_record_offset":49524,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":529,"record_offset":55692,"parent_record_offset":55668,"ancestor_record_offset":49524,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":50640,"parent_record_offset":50616,"ancestor_record_offset":49524,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":55792,"parent_record_offset":55724,"ancestor_record_offset":49524,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":50616,"parent_record_offset":50592,"ancestor_record_offset":49524,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":50824,"parent_record_offset":50796,"ancestor_record_offset":49524,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":55816,"parent_record_offset":55792,"ancestor_record_offset":49524,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":51640,"parent_record_offset":51616,"ancestor_record_offset":49524,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":529,"record_offset":52360,"parent_record_offset":52332,"ancestor_record_offset":49524,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":52740,"parent_record_offset":52716,"ancestor_record_offset":49524,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":52396,"parent_record_offset":52360,"ancestor_record_offset":49524,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":52716,"parent_record_offset":52536,"ancestor_record_offset":49524,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":529,"record_offset":58864,"parent_record_offset":58692,"ancestor_record_offset":49524,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":50568,"parent_record_offset":50544,"ancestor_record_offset":49524,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":50592,"parent_record_offset":50568,"ancestor_record_offset":49524,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":50896,"parent_record_offset":50868,"ancestor_record_offset":49524,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":50544,"parent_record_offset":50428,"ancestor_record_offset":49524,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":529,"record_offset":55380,"parent_record_offset":55344,"ancestor_record_offset":49524,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":529,"record_offset":49896,"parent_record_offset":49868,"ancestor_record_offset":49524,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":529,"record_offset":50256,"parent_record_offset":50228,"ancestor_record_offset":49524,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":529,"record_offset":50396,"parent_record_offset":50364,"ancestor_record_offset":49524,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":529,"record_offset":50364,"parent_record_offset":50328,"ancestor_record_offset":49524,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":529,"record_offset":50304,"parent_record_offset":50280,"ancestor_record_offset":49524,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":529,"record_offset":50328,"parent_record_offset":50304,"ancestor_record_offset":49524,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":529,"record_offset":50280,"parent_record_offset":50256,"ancestor_record_offset":49524,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":529,"record_offset":50868,"parent_record_offset":50328,"ancestor_record_offset":49524,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":49868,"parent_record_offset":49836,"ancestor_record_offset":49524,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":49836,"parent_record_offset":49808,"ancestor_record_offset":49524,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":50028,"parent_record_offset":49996,"ancestor_record_offset":49524,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":50124,"parent_record_offset":50028,"ancestor_record_offset":49524,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":49996,"parent_record_offset":49972,"ancestor_record_offset":49524,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":49808,"parent_record_offset":49772,"ancestor_record_offset":49524,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":50148,"parent_record_offset":50124,"ancestor_record_offset":49524,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":529,"record_offset":50428,"parent_record_offset":50396,"ancestor_record_offset":49524,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":529,"record_offset":50712,"parent_record_offset":50428,"ancestor_record_offset":49524,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":529,"record_offset":50684,"parent_record_offset":50428,"ancestor_record_offset":49524,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":529,"record_offset":50488,"parent_record_offset":50428,"ancestor_record_offset":49524,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":55044,"parent_record_offset":55020,"ancestor_record_offset":49524,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":529,"record_offset":55068,"parent_record_offset":55044,"ancestor_record_offset":49524,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":55900,"parent_record_offset":55852,"ancestor_record_offset":49524,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":55596,"parent_record_offset":55568,"ancestor_record_offset":49524,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":55568,"parent_record_offset":55544,"ancestor_record_offset":49524,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":55544,"parent_record_offset":55520,"ancestor_record_offset":49524,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":529,"record_offset":59864,"parent_record_offset":59836,"ancestor_record_offset":49524,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":529,"record_offset":57744,"parent_record_offset":57720,"ancestor_record_offset":49524,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":51240,"parent_record_offset":51204,"ancestor_record_offset":49524,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":62124,"parent_record_offset":62092,"ancestor_record_offset":49524,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":62624,"parent_record_offset":62364,"ancestor_record_offset":49524,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":529,"record_offset":51384,"parent_record_offset":51356,"ancestor_record_offset":49524,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":51460,"parent_record_offset":51312,"ancestor_record_offset":49524,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":51720,"parent_record_offset":51460,"ancestor_record_offset":49524,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":57804,"parent_record_offset":57780,"ancestor_record_offset":49524,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":57720,"parent_record_offset":57696,"ancestor_record_offset":49524,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":529,"record_offset":55620,"parent_record_offset":55596,"ancestor_record_offset":49524,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":529,"record_offset":55932,"parent_record_offset":55900,"ancestor_record_offset":49524,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":55996,"parent_record_offset":55964,"ancestor_record_offset":49524,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":56124,"parent_record_offset":56064,"ancestor_record_offset":49524,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":55964,"parent_record_offset":55932,"ancestor_record_offset":49524,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":529,"record_offset":51412,"parent_record_offset":51384,"ancestor_record_offset":49524,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":51312,"parent_record_offset":51276,"ancestor_record_offset":49524,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":51276,"parent_record_offset":51240,"ancestor_record_offset":49524,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=32
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8033
// 首条位置: {"module":574,"inlinee":32819}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$59::cmp
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1592,"record_offset":23360,"parent_record_offset":23260,"ancestor_record_offset":23260,"inlinee":7153,"source_line":1836,"archive":"_pdb/B/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1592,"record_offset":23296,"parent_record_offset":23260,"ancestor_record_offset":23260,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
// 记录形式: compiled_procedure; instances=1
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":529,"record_offset":64432,"rva":10249984,"va":5378959104}
// 编译实例 RVAs: 0x9c6700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9359715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e89d2e71ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378960510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=4
//     target_aliases=19 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9343663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8516d71ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378959770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408923c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1265374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e822b1ecff","size":5,"soft_fail":false,"target_va":5377696704,"va":5378962073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408935c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1260863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8c1c2ecff","size":5,"soft_fail":false,"target_va":5377701312,"va":5378962170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fd6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-825226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e87668f3ff","size":5,"soft_fail":false,"target_va":5378135744,"va":5378960965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140908610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-780262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e81a18f4ff","size":5,"soft_fail":false,"target_va":5378180624,"va":5378960881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-373851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8a54bfaff","size":5,"soft_fail":false,"target_va":5378586944,"va":5378960790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409cdec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e86a6e0000","size":5,"soft_fail":false,"target_va":5378989760,"va":5378961489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2163085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e88d012100","size":5,"soft_fail":false,"target_va":5381123296,"va":5378960206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be6750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2226910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8defa2100","size":5,"soft_fail":false,"target_va":5381187408,"va":5378960493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141629e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12988390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8e62fc600","size":5,"soft_fail":false,"target_va":5391949456,"va":5378961061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162c590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12998410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e80a57c600","size":5,"soft_fail":false,"target_va":5391959440,"va":5378961025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32708366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e80e17f301","size":5,"soft_fail":false,"target_va":5411669968,"va":5378961597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: ui_2d_pass.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32886342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e846cef501","size":5,"soft_fail":false,"target_va":5411847168,"va":5378960821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32579924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e85421f101","size":5,"soft_fail":false,"target_va":5411541584,"va":5378961655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t32580104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e80822f101","size":5,"soft_fail":false,"target_va":5411541776,"va":5378961667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-32994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e81e7fffff","size":5,"soft_fail":false,"target_va":5378926208,"va":5378959197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32584088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e89831f101","size":5,"soft_fail":false,"target_va":5411543360,"va":5378959267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-585568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8a010f7ff","size":5,"soft_fail":false,"target_va":5378374304,"va":5378959867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-787563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e895fbf3ff","size":5,"soft_fail":false,"target_va":5378173584,"va":5378961142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-790901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e88beef3ff","size":5,"soft_fail":false,"target_va":5378169824,"va":5378960720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1438454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e80a0deaff","size":5,"soft_fail":false,"target_va":5377522032,"va":5378960481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1436588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e85414eaff","size":5,"soft_fail":false,"target_va":5377524592,"va":5378961175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2425829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8e5032500","size":5,"soft_fail":false,"target_va":5381385536,"va":5378959702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2431018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e82a182500","size":5,"soft_fail":false,"target_va":5381390640,"va":5378959617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2424920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e858002500","size":5,"soft_fail":false,"target_va":5381386480,"va":5378961555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2425537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8c1022500","size":5,"soft_fail":false,"target_va":5381384704,"va":5378959162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2157688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e878ec2000","size":5,"soft_fail":false,"target_va":5381117840,"va":5378960147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2156403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e873e72000","size":5,"soft_fail":false,"target_va":5381116576,"va":5378960168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2238801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e851292200","size":5,"soft_fail":false,"target_va":5381199056,"va":5378960250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9863356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8bc809600","size":5,"soft_fail":false,"target_va":5388825296,"va":5378961935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32575456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8e00ff101","size":5,"soft_fail":false,"target_va":5411537104,"va":5378961643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1870711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378959104,"raw_hex":"e8778b1c00","size":5,"soft_fail":false,"target_va":5380830832,"va":5378960116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":67084,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":67872,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":71456,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":70384,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":529,"record_offset":70312,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":69840,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":25985,"source_line":106,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":69640,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":529,"record_offset":68684,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":26000,"source_line":45,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":66100,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":64644,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":65112,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":70556,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":70472,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":8285,"source_line":628,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":529,"record_offset":69516,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":529,"record_offset":69488,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":67520,"parent_record_offset":67496,"ancestor_record_offset":64432,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":67568,"parent_record_offset":67544,"ancestor_record_offset":64432,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":529,"record_offset":67544,"parent_record_offset":67520,"ancestor_record_offset":64432,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":67496,"parent_record_offset":67472,"ancestor_record_offset":64432,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":67472,"parent_record_offset":67432,"ancestor_record_offset":64432,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":67432,"parent_record_offset":67408,"ancestor_record_offset":64432,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":67820,"parent_record_offset":67796,"ancestor_record_offset":64432,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":67796,"parent_record_offset":67772,"ancestor_record_offset":64432,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":529,"record_offset":67744,"parent_record_offset":67712,"ancestor_record_offset":64432,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":67688,"parent_record_offset":67152,"ancestor_record_offset":64432,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":67712,"parent_record_offset":67688,"ancestor_record_offset":64432,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":67180,"parent_record_offset":67152,"ancestor_record_offset":64432,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":529,"record_offset":67772,"parent_record_offset":67712,"ancestor_record_offset":64432,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":67228,"parent_record_offset":67204,"ancestor_record_offset":64432,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":67204,"parent_record_offset":67180,"ancestor_record_offset":64432,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":529,"record_offset":67408,"parent_record_offset":67384,"ancestor_record_offset":64432,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":67112,"parent_record_offset":67084,"ancestor_record_offset":64432,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":529,"record_offset":67152,"parent_record_offset":67084,"ancestor_record_offset":64432,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":66436,"parent_record_offset":66408,"ancestor_record_offset":64432,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":70140,"parent_record_offset":70112,"ancestor_record_offset":64432,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":66488,"parent_record_offset":66464,"ancestor_record_offset":64432,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":66464,"parent_record_offset":66436,"ancestor_record_offset":64432,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":66384,"parent_record_offset":66356,"ancestor_record_offset":64432,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":66408,"parent_record_offset":66384,"ancestor_record_offset":64432,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":66580,"parent_record_offset":66408,"ancestor_record_offset":64432,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":529,"record_offset":66692,"parent_record_offset":66668,"ancestor_record_offset":64432,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":529,"record_offset":69580,"parent_record_offset":69516,"ancestor_record_offset":64432,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":529,"record_offset":66512,"parent_record_offset":66488,"ancestor_record_offset":64432,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":529,"record_offset":64804,"parent_record_offset":64772,"ancestor_record_offset":64432,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":529,"record_offset":66252,"parent_record_offset":66208,"ancestor_record_offset":64432,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":70352,"parent_record_offset":70312,"ancestor_record_offset":64432,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":67344,"parent_record_offset":67320,"ancestor_record_offset":64432,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":67384,"parent_record_offset":67204,"ancestor_record_offset":64432,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":529,"record_offset":67936,"parent_record_offset":67908,"ancestor_record_offset":64432,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":65060,"parent_record_offset":65032,"ancestor_record_offset":64432,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":529,"record_offset":67040,"parent_record_offset":66208,"ancestor_record_offset":64432,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":66644,"parent_record_offset":66356,"ancestor_record_offset":64432,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":65528,"parent_record_offset":65504,"ancestor_record_offset":64432,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":65504,"parent_record_offset":65480,"ancestor_record_offset":64432,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":529,"record_offset":65712,"parent_record_offset":65684,"ancestor_record_offset":64432,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":529,"record_offset":66536,"parent_record_offset":66512,"ancestor_record_offset":64432,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":529,"record_offset":67256,"parent_record_offset":67228,"ancestor_record_offset":64432,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":67636,"parent_record_offset":67612,"ancestor_record_offset":64432,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":67292,"parent_record_offset":67256,"ancestor_record_offset":64432,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":529,"record_offset":67612,"parent_record_offset":67432,"ancestor_record_offset":64432,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":65456,"parent_record_offset":65432,"ancestor_record_offset":64432,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":65480,"parent_record_offset":65456,"ancestor_record_offset":64432,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":65784,"parent_record_offset":65756,"ancestor_record_offset":64432,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":529,"record_offset":65432,"parent_record_offset":65316,"ancestor_record_offset":64432,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":529,"record_offset":64772,"parent_record_offset":64744,"ancestor_record_offset":64432,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":529,"record_offset":65144,"parent_record_offset":65112,"ancestor_record_offset":64432,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":529,"record_offset":65284,"parent_record_offset":65252,"ancestor_record_offset":64432,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":529,"record_offset":65252,"parent_record_offset":65216,"ancestor_record_offset":64432,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":529,"record_offset":65192,"parent_record_offset":65168,"ancestor_record_offset":64432,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":529,"record_offset":65216,"parent_record_offset":65192,"ancestor_record_offset":64432,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":529,"record_offset":65168,"parent_record_offset":65144,"ancestor_record_offset":64432,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":529,"record_offset":65756,"parent_record_offset":65216,"ancestor_record_offset":64432,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":64744,"parent_record_offset":64712,"ancestor_record_offset":64432,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":64712,"parent_record_offset":64684,"ancestor_record_offset":64432,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":64912,"parent_record_offset":64876,"ancestor_record_offset":64432,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":65008,"parent_record_offset":64912,"ancestor_record_offset":64432,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":64876,"parent_record_offset":64848,"ancestor_record_offset":64432,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":64684,"parent_record_offset":64644,"ancestor_record_offset":64432,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":529,"record_offset":65032,"parent_record_offset":65008,"ancestor_record_offset":64432,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":529,"record_offset":65316,"parent_record_offset":65284,"ancestor_record_offset":64432,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":529,"record_offset":65600,"parent_record_offset":65316,"ancestor_record_offset":64432,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":529,"record_offset":65572,"parent_record_offset":65316,"ancestor_record_offset":64432,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":529,"record_offset":65376,"parent_record_offset":65316,"ancestor_record_offset":64432,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":529,"record_offset":69888,"parent_record_offset":69840,"ancestor_record_offset":64432,"inlinee":25987,"source_line":138,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":69764,"parent_record_offset":69732,"ancestor_record_offset":64432,"inlinee":25998,"source_line":89,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":69732,"parent_record_offset":69704,"ancestor_record_offset":64432,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":529,"record_offset":69704,"parent_record_offset":69676,"ancestor_record_offset":64432,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":529,"record_offset":70520,"parent_record_offset":70496,"ancestor_record_offset":64432,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":66136,"parent_record_offset":66100,"ancestor_record_offset":64432,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":71520,"parent_record_offset":71488,"ancestor_record_offset":64432,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":72020,"parent_record_offset":71760,"ancestor_record_offset":64432,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":529,"record_offset":66280,"parent_record_offset":66252,"ancestor_record_offset":64432,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":66356,"parent_record_offset":66208,"ancestor_record_offset":64432,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":529,"record_offset":66616,"parent_record_offset":66356,"ancestor_record_offset":64432,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":70580,"parent_record_offset":70556,"ancestor_record_offset":64432,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":529,"record_offset":70496,"parent_record_offset":70472,"ancestor_record_offset":64432,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":529,"record_offset":69552,"parent_record_offset":69516,"ancestor_record_offset":64432,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":529,"record_offset":69792,"parent_record_offset":69764,"ancestor_record_offset":64432,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":529,"record_offset":69608,"parent_record_offset":69516,"ancestor_record_offset":64432,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":529,"record_offset":69920,"parent_record_offset":69888,"ancestor_record_offset":64432,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":69984,"parent_record_offset":69952,"ancestor_record_offset":64432,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":70112,"parent_record_offset":70052,"ancestor_record_offset":64432,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":529,"record_offset":69952,"parent_record_offset":69920,"ancestor_record_offset":64432,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":529,"record_offset":66308,"parent_record_offset":66280,"ancestor_record_offset":64432,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":66208,"parent_record_offset":66172,"ancestor_record_offset":64432,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":529,"record_offset":66172,"parent_record_offset":66136,"ancestor_record_offset":64432,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 279
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x768F
// 首条位置: {"module":531,"inlinee":30351}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":531,"record_offset":42432,"parent_record_offset":42380,"ancestor_record_offset":42380,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":42608,"parent_record_offset":42380,"ancestor_record_offset":42380,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":42524,"parent_record_offset":42380,"ancestor_record_offset":42380,"inlinee":8285,"source_line":628,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":531,"record_offset":42572,"parent_record_offset":42548,"ancestor_record_offset":42380,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":42632,"parent_record_offset":42608,"ancestor_record_offset":42380,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":531,"record_offset":42548,"parent_record_offset":42524,"ancestor_record_offset":42380,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/00531.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_2d_pass.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::handle", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_2d_pass.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: ui_2d_pass.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":56324,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":57652,"parent_record_offset":49524,"ancestor_record_offset":49524,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":70312,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":26792,"source_line":502,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":529,"record_offset":70384,"parent_record_offset":64432,"ancestor_record_offset":64432,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00529.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":531,"record_offset":42432,"parent_record_offset":42380,"ancestor_record_offset":42380,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00531.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1455112,"position_among_mapped_render_targets":19,"mapped_render_target_count":25,"previous_callsite":1455076,"next_callsite":1455177,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1455177,"position_among_mapped_render_targets":20,"mapped_render_target_count":25,"previous_callsite":1455112,"next_callsite":1455213,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":1460144,"caller_aliases":{"alias_group_rva":1460144,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1463114,"position_among_mapped_render_targets":2,"mapped_render_target_count":4,"previous_callsite":1463082,"next_callsite":1463150,"previous_target_files":["crates/country-core/src/systems/render_loop/vermeer.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_2d_pass.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)
// 候选参数类型: ["country_core::render::pass_builder::draw_builder::DrawBuilder", "ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>", "bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<std::path::PathBuf>,ref$<std::path::PathBuf>)
// 候选参数类型: ["ref$<std::path::PathBuf>", "ref$<std::path::PathBuf>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
// 信息 ID: 001e5a8ac8356beed619356e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163449 -> 0x9c6700 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x16534a -> 0x9c6700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure$0
// 信息 ID: 5c00fe23e1333bdc718dc9bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0 <- country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass::closure_env$0>; module=531, inline_record=42380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
// 信息 ID: 9520a44f0725dc15448d312a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163408 -> 0x9c4470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure$0
// 信息 ID: 808b32f89bdf49029655c122
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::stable::driftsort_main<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0>,alloc::vec::Vec<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::shared::pivot::median3_rec<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::stable::drift::sort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0 <- core::slice::sort::stable::quicksort::quicksort<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; module=1592, inline_record=23260; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; module=1592, inline_record=23424; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; module=1592, inline_record=23724; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; module=1592, inline_record=23908; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; module=1592, inline_record=24160; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; module=1592, inline_record=107376; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; module=1592, inline_record=107656; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::small_sort_general_with_scratch<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2dImage>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIcon> > >,enum2$<core::option::Option<ref$<country_core::components::UiInteractionState> > >,enum2$<core::option::Option<ref$<country_core::components::UiDrawOrder> > >,enum2$<core::option::Option<ref$<country_core::components::UiInactive> > > >,country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass::closure_env$0> >; module=1592, inline_record=179676; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
