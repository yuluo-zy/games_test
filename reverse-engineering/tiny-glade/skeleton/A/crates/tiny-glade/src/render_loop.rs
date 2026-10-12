// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/render_loop.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ab422845297aa1bab75a6ef6932451476e7b0f7995b262a8b0d1368f354a6d25
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: render_loop.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 138, 197
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"PDB_registry_path_only","version":"0.7.4"}]
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"PDB_registry_path_only","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_frame_render_graph
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":197,"record_offset":3636,"rva":3133120,"va":5371842240}
// 编译实例 RVAs: 0x2fcec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400668f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2727191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8e962d6ff","size":5,"soft_fail":false,"target_va":5369129200,"va":5371856386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400db0c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2240651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e875cfddff","size":5,"soft_fail":false,"target_va":5369606336,"va":5371846982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400db690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2240986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e826ceddff","size":5,"soft_fail":false,"target_va":5369607824,"va":5371848805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400db9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2246252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e894b9ddff","size":5,"soft_fail":false,"target_va":5369608672,"va":5371854919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dc470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2238888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e858d6ddff","size":5,"soft_fail":false,"target_va":5369611376,"va":5371850259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2219167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e86123deff","size":5,"soft_fail":false,"target_va":5369632448,"va":5371851610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ff9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2102965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e84be9dfff","size":5,"soft_fail":false,"target_va":5369756144,"va":5371859104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140265f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-623297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e83f7df6ff","size":5,"soft_fail":false,"target_va":5371223824,"va":5371847116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402664a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-621848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8e882f6ff","size":5,"soft_fail":false,"target_va":5371225248,"va":5371847091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140266a30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-620399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e89188f6ff","size":5,"soft_fail":false,"target_va":5371226672,"va":5371847066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c10a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-259126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8ca0bfcff","size":5,"soft_fail":false,"target_va":5371596960,"va":5371856081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140366a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t414300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e85c520600","size":5,"soft_fail":false,"target_va":5372275296,"va":5371860991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140367730
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t418888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e848640600","size":5,"soft_fail":false,"target_va":5372278576,"va":5371859683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140378770
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t491748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8e4800700","size":5,"soft_fail":false,"target_va":5372348272,"va":5371856519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140385b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t542380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8ac460800","size":5,"soft_fail":false,"target_va":5372402560,"va":5371860175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140386550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t544508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8fc4e0800","size":5,"soft_fail":false,"target_va":5372405072,"va":5371860559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140392800
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t598297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e819210900","size":5,"soft_fail":false,"target_va":5372454912,"va":5371856610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t893559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e877a20d00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371856148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1306017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8a1ed1300","size":5,"soft_fail":false,"target_va":5373149840,"va":5371843818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=9
//     target_aliases=3 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2671848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8e8c42800","size":5,"soft_fail":false,"target_va":5374516480,"va":5371844627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140648fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3448121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8399d3400","size":5,"soft_fail":false,"target_va":5375299488,"va":5371851362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4380375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8d7d64200","size":5,"soft_fail":false,"target_va":5376239920,"va":5371859540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407378d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4433751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e857a74300","size":5,"soft_fail":false,"target_va":5376276688,"va":5371842932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140748250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4489808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e850824400","size":5,"soft_fail":false,"target_va":5376344656,"va":5371854843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31057850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8bae7d901","size":5,"soft_fail":false,"target_va":5402900112,"va":5371842257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31749789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e89d76e401","size":5,"soft_fail":false,"target_va":5403593360,"va":5371843566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-549734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e89a9cf7ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5371854609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t5512103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8a71b5400","size":5,"soft_fail":false,"target_va":5377364336,"va":5371852228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31121313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8a1dfda01","size":5,"soft_fail":false,"target_va":5402978912,"va":5371857594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=20
//     evidence={"assembly":"\tcallq\t31121427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e813e0da01","size":5,"soft_fail":false,"target_va":5402978880,"va":5371857448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31121142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8f6deda01","size":5,"soft_fail":false,"target_va":5402979152,"va":5371858005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31122059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e88be2da01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371858064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31122610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8b2e4da01","size":5,"soft_fail":false,"target_va":5402980464,"va":5371857849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3574663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8878b3600","size":5,"soft_fail":false,"target_va":5375430304,"va":5371855636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2513290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e88a592600","size":5,"soft_fail":false,"target_va":5374356208,"va":5371842913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2500325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8e5262600","size":5,"soft_fail":false,"target_va":5374355680,"va":5371855350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2512527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e88f562600","size":5,"soft_fail":false,"target_va":5374355424,"va":5371842892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::frame_constants::FrameConstants::should_reset_temporal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3308134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8667a3200","size":5,"soft_fail":false,"target_va":5375165200,"va":5371857061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::GpuWorld::export_from_rg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3289442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e862313200","size":5,"soft_fail":false,"target_va":5375145376,"va":5371855929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::gpu_world::GpuWorld::import_to_rg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3301294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8ae5f3200","size":5,"soft_fail":false,"target_va":5375144560,"va":5371843261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gpu_lut::GpuLutTracker::render_luts
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2622296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e858032800","size":5,"soft_fail":false,"target_va":5374466784,"va":5371844483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::grading_lut::ColorGradingState::flattened
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2179795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8d3422100","size":5,"soft_fail":false,"target_va":5374031184,"va":5371851384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2553068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8ecf42600","size":5,"soft_fail":false,"target_va":5374399200,"va":5371846127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2553045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8d5f42600","size":5,"soft_fail":false,"target_va":5374399152,"va":5371846102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::should_show_world
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2553824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8e0f72600","size":5,"soft_fail":false,"target_va":5374399248,"va":5371845419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::GladeType::is_winter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2408282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e85abf2400","size":5,"soft_fail":false,"target_va":5374255120,"va":5371846833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::RenderContext::finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3550718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8fe2d3600","size":5,"soft_fail":false,"target_va":5375405616,"va":5371854893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2440997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8253f2500","size":5,"soft_fail":false,"target_va":5374290208,"va":5371849206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::apply_screen_shadows::compute_shadow_mask_mean
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2442171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8bb432500","size":5,"soft_fail":false,"target_va":5374291968,"va":5371849792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3553547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e80b393600","size":5,"soft_fail":false,"target_va":5375403424,"va":5371849872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4001993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8c9103d00","size":5,"soft_fail":false,"target_va":5375849312,"va":5371847314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4004033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8c1183d00","size":5,"soft_fail":false,"target_va":5375851440,"va":5371847402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3177145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8b97a3000","size":5,"soft_fail":false,"target_va":5375024288,"va":5371847138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3180659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e873883000","size":5,"soft_fail":false,"target_va":5375028000,"va":5371847336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2760857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e899202a00","size":5,"soft_fail":false,"target_va":5374608352,"va":5371847490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::dof::calc_dof
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2410546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e832c82400","size":5,"soft_fail":false,"target_va":5374260864,"va":5371850313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::dof::dof_prepare
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2409569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e861c42400","size":5,"soft_fail":false,"target_va":5374259488,"va":5371849914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2166103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8570d2100","size":5,"soft_fail":false,"target_va":5374014448,"va":5371848340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::draw_path_mask::draw_path_mask_compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3356181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e815363300","size":5,"soft_fail":false,"target_va":5375203456,"va":5371847270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::eval_gi_screen_radiance
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3411384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8b80d3400","size":5,"soft_fail":false,"target_va":5375260544,"va":5371849155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3504948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8347b3500","size":5,"soft_fail":false,"target_va":5375353088,"va":5371848135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3223917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e86d313100","size":5,"soft_fail":false,"target_va":5375071456,"va":5371847534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3225803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8cb383100","size":5,"soft_fail":false,"target_va":5375073984,"va":5371848176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3214196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8740b3100","size":5,"soft_fail":false,"target_va":5375061760,"va":5371847559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::grass::grass_compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3748817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8d1333900","size":5,"soft_fail":false,"target_va":5375597056,"va":5371848234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3862259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8f3ee3a00","size":5,"soft_fail":false,"target_va":5375710720,"va":5371848456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::highlight_active_terrain_stroke
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e831392c00","size":5,"soft_fail":false,"target_va":5374745456,"va":5371847226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::leaf_particles::leaf_particles_compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2146341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e825c02000","size":5,"soft_fail":false,"target_va":5373993792,"va":5371847446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2140170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e80aa82000","size":5,"soft_fail":false,"target_va":5373988544,"va":5371848369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3957847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e857643c00","size":5,"soft_fail":false,"target_va":5375805056,"va":5371847204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2633851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e87b302800","size":5,"soft_fail":false,"target_va":5374481280,"va":5371847424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2598847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8bfa72700","size":5,"soft_fail":false,"target_va":5374445040,"va":5371846188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3230479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e80f4b3100","size":5,"soft_fail":false,"target_va":5375077776,"va":5371847292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3305795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e843713200","size":5,"soft_fail":false,"target_va":5375152960,"va":5371847160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3063309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e80dbe2e00","size":5,"soft_fail":false,"target_va":5374912400,"va":5371849086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2974407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8c7622d00","size":5,"soft_fail":false,"target_va":5374821792,"va":5371847380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::read_terrain_data::read_terrain_data
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4028925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8fd793d00","size":5,"soft_fail":false,"target_va":5375876112,"va":5371847182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::reprojection::calculate_reprojection_map
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3160943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e86f3b3000","size":5,"soft_fail":false,"target_va":5375009536,"va":5371848588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2447970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8625a2500","size":5,"soft_fail":false,"target_va":5374292480,"va":5371844505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3757279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8df543900","size":5,"soft_fail":false,"target_va":5375604752,"va":5371847468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2595365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8259a2700","size":5,"soft_fail":false,"target_va":5374446208,"va":5371850838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2326957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8ad812300","size":5,"soft_fail":false,"target_va":5374174320,"va":5371847358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3390696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8e8bc3300","size":5,"soft_fail":false,"target_va":5375239360,"va":5371848659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::stairs::stairs
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3211427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8a3003100","size":5,"soft_fail":false,"target_va":5375058944,"va":5371847512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::taa::calculate_taa
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3232078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e84e513100","size":5,"soft_fail":false,"target_va":5375082800,"va":5371850717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::calc_terrain_lighting_texture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2447889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8115a2500","size":5,"soft_fail":false,"target_va":5374295504,"va":5371847610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::visibility_queries::run_visibility_queries
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3885060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e804483b00","size":5,"soft_fail":false,"target_va":5375733376,"va":5371848311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3070021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e845d82e00","size":5,"soft_fail":false,"target_va":5374919328,"va":5371849302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2594582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e816972700","size":5,"soft_fail":false,"target_va":5374448240,"va":5371853653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::final_post::final_post
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2445133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e84d4f2500","size":5,"soft_fail":false,"target_va":5374297024,"va":5371851886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2444714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8aa4d2500","size":5,"soft_fail":false,"target_va":5374299456,"va":5371854737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2596652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e82c9f2700","size":5,"soft_fail":false,"target_va":5374449920,"va":5371853263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2156234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8cae62000","size":5,"soft_fail":false,"target_va":5374004656,"va":5371848417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::main_pass::main_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2149501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e87dcc2000","size":5,"soft_fail":false,"target_va":5373997584,"va":5371848078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2152833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e881d92000","size":5,"soft_fail":false,"target_va":5374001120,"va":5371848282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2640059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8bb482800","size":5,"soft_fail":false,"target_va":5374487312,"va":5371847248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3185671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8079c3000","size":5,"soft_fail":false,"target_va":5375033264,"va":5371847588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3202754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8c2de3000","size":5,"soft_fail":false,"target_va":5375053568,"va":5371850809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3202401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e861dd3000","size":5,"soft_fail":false,"target_va":5375052176,"va":5371849770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3185681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8119c3000","size":5,"soft_fail":false,"target_va":5375035264,"va":5371849578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3703877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e845843800","size":5,"soft_fail":false,"target_va":5375557712,"va":5371853830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3697980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e83c6d3800","size":5,"soft_fail":false,"target_va":5375551760,"va":5371853775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3811331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e803283a00","size":5,"soft_fail":false,"target_va":5375665488,"va":5371854152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_pass::ui_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3810989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8ad263a00","size":5,"soft_fail":false,"target_va":5375664736,"va":5371853742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3812533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8b52c3a00","size":5,"soft_fail":false,"target_va":5375666240,"va":5371853702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2446807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8d7552500","size":5,"soft_fail":false,"target_va":5374300432,"va":5371853620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::menu_background::render_menu_bg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2598884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8e4a72700","size":5,"soft_fail":false,"target_va":5374451440,"va":5371852551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::painterly::PainterlyState::has_target
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3070638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8aeda2e00","size":5,"soft_fail":false,"target_va":5374927408,"va":5371856765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::painterly::painterly_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3074854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e826eb2e00","size":5,"soft_fail":false,"target_va":5374927472,"va":5371852613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::session_selection::render_session_selection_previews
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2931384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8b8ba2c00","size":5,"soft_fail":false,"target_va":5374784976,"va":5371853587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::vermeer_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3395784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8c8d03300","size":5,"soft_fail":false,"target_va":5375248944,"va":5371853155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t5501558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e876f25300","size":5,"soft_fail":false,"target_va":5377353776,"va":5371852213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31117547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8ebd0da01","size":5,"soft_fail":false,"target_va":5402968992,"va":5371851440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31119234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e882d7da01","size":5,"soft_fail":false,"target_va":5402970800,"va":5371851561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4418509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8cd6b4300","size":5,"soft_fail":false,"target_va":5376262400,"va":5371843886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4417361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e851674300","size":5,"soft_fail":false,"target_va":5376261408,"va":5371844042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4493077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8158f4400","size":5,"soft_fail":false,"target_va":5376341760,"va":5371848678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::format
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4493972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e894924400","size":5,"soft_fail":false,"target_va":5376340880,"va":5371846903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::mip_levels
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4496361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8e99b4400","size":5,"soft_fail":false,"target_va":5376340928,"va":5371844562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t4495974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8669a4400","size":5,"soft_fail":false,"target_va":5376340512,"va":5371844533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4491983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8cf8a4400","size":5,"soft_fail":false,"target_va":5376340720,"va":5371848732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4491986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8d28a4400","size":5,"soft_fail":false,"target_va":5376340752,"va":5371848761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::usage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4493729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8a1914400","size":5,"soft_fail":false,"target_va":5376340672,"va":5371846938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t4374873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e859c14200","size":5,"soft_fail":false,"target_va":5376219552,"va":5371844674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_resource_handle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4364726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8b6994200","size":5,"soft_fail":false,"target_va":5376219312,"va":5371854581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::on_retire
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4362609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e871914200","size":5,"soft_fail":false,"target_va":5376219056,"va":5371856442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::egui_backend::EguiBackend::draw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2767695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8b1c4d5ff","size":5,"soft_fail":false,"target_va":5369086512,"va":5371854202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_screenshot::capture_image
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t795001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e879210c00","size":5,"soft_fail":false,"target_va":5372649776,"va":5371854770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::handle_screenshot_opportunity
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-1721159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371842240,"raw_hex":"e8b9bce5ff","size":5,"soft_fail":false,"target_va":5370130848,"va":5371852002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%r10","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371842240,"raw_hex":"41ffd2","size":3,"soft_fail":false,"va":5371849696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":197,"record_offset":5920,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":7892,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":4176,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":13596,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":11624,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":197,"record_offset":26876,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":3876,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":40536,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":26844,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":38024,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":3996,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":3904,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":18208,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":37108,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":197,"record_offset":16620,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":197,"record_offset":6032,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":12651,"source_line":11,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":197,"record_offset":11228,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":7522,"source_line":144,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::RenderGraph::record
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":197,"record_offset":3940,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":7460,"source_line":107,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":13396,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":31736,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":8356,"source_line":69,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::finish
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":37080,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":9082,"source_line":96,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import_lazy_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":31568,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":9080,"source_line":77,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::TemporalRenderGraphRecorder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":197,"record_offset":4148,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":8384,"source_line":117,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::compile_frame_render_graph::explain
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":38424,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":12660,"source_line":126,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::handle_screenshot_error
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":27084,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":12659,"source_line":816,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::render_world
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":20240,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":12656,"source_line":853,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::next_requested_screenshot_stage
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":197,"record_offset":26012,"parent_record_offset":3636,"ancestor_record_offset":3636,"inlinee":9754,"source_line":495,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":197,"record_offset":37676,"parent_record_offset":37648,"ancestor_record_offset":3636,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":197,"record_offset":37648,"parent_record_offset":37432,"ancestor_record_offset":3636,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":38316,"parent_record_offset":38284,"ancestor_record_offset":3636,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":31668,"parent_record_offset":31636,"ancestor_record_offset":3636,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":38284,"parent_record_offset":38252,"ancestor_record_offset":3636,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":31636,"parent_record_offset":31600,"ancestor_record_offset":3636,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":31600,"parent_record_offset":31568,"ancestor_record_offset":3636,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":26232,"parent_record_offset":26012,"ancestor_record_offset":3636,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":197,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":3636,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":197,"record_offset":4228,"parent_record_offset":4200,"ancestor_record_offset":3636,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":5752,"parent_record_offset":5728,"ancestor_record_offset":3636,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":3636,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":5676,"parent_record_offset":5644,"ancestor_record_offset":3636,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":9332,"parent_record_offset":8944,"ancestor_record_offset":3636,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":5620,"parent_record_offset":5228,"ancestor_record_offset":3636,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":3636,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":5260,"parent_record_offset":5228,"ancestor_record_offset":3636,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":5704,"parent_record_offset":5644,"ancestor_record_offset":3636,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":5308,"parent_record_offset":5284,"ancestor_record_offset":3636,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":3636,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":5400,"parent_record_offset":5336,"ancestor_record_offset":3636,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":3636,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":3636,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":5796,"parent_record_offset":5228,"ancestor_record_offset":3636,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":3636,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":8944,"parent_record_offset":7916,"ancestor_record_offset":3636,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3636,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":5228,"parent_record_offset":4200,"ancestor_record_offset":3636,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":197,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":3636,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":197,"record_offset":26964,"parent_record_offset":26900,"ancestor_record_offset":3636,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":27204,"parent_record_offset":27084,"ancestor_record_offset":3636,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":3636,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":3636,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":197,"record_offset":13464,"parent_record_offset":13396,"ancestor_record_offset":3636,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":197,"record_offset":4424,"parent_record_offset":4400,"ancestor_record_offset":3636,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":197,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":3636,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":197,"record_offset":38568,"parent_record_offset":38424,"ancestor_record_offset":3636,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":197,"record_offset":27232,"parent_record_offset":27084,"ancestor_record_offset":3636,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":197,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":3636,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":197,"record_offset":11460,"parent_record_offset":11428,"ancestor_record_offset":3636,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":197,"record_offset":26988,"parent_record_offset":26964,"ancestor_record_offset":3636,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":197,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3636,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":37524,"parent_record_offset":37496,"ancestor_record_offset":3636,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":37548,"parent_record_offset":37524,"ancestor_record_offset":3636,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":197,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":3636,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":197,"record_offset":26936,"parent_record_offset":26900,"ancestor_record_offset":3636,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":197,"record_offset":26900,"parent_record_offset":26876,"ancestor_record_offset":3636,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":3636,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":5196,"parent_record_offset":4228,"ancestor_record_offset":3636,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":26384,"parent_record_offset":26012,"ancestor_record_offset":3636,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":5464,"parent_record_offset":5284,"ancestor_record_offset":3636,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":18176,"parent_record_offset":16620,"ancestor_record_offset":3636,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":197,"record_offset":5568,"parent_record_offset":5544,"ancestor_record_offset":3636,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":197,"record_offset":27012,"parent_record_offset":26988,"ancestor_record_offset":3636,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":11380,"parent_record_offset":11228,"ancestor_record_offset":3636,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":3636,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":3636,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":3636,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":26296,"parent_record_offset":26260,"ancestor_record_offset":3636,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":3636,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":4076,"parent_record_offset":4020,"ancestor_record_offset":3636,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":26168,"parent_record_offset":26080,"ancestor_record_offset":3636,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":26192,"parent_record_offset":26168,"ancestor_record_offset":3636,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":22244,"parent_record_offset":20240,"ancestor_record_offset":3636,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":11308,"parent_record_offset":11276,"ancestor_record_offset":3636,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":5336,"parent_record_offset":5308,"ancestor_record_offset":3636,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":5372,"parent_record_offset":5336,"ancestor_record_offset":3636,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":5544,"parent_record_offset":5512,"ancestor_record_offset":3636,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":26324,"parent_record_offset":26260,"ancestor_record_offset":3636,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":26260,"parent_record_offset":26012,"ancestor_record_offset":3636,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":3636,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":26488,"parent_record_offset":26460,"ancestor_record_offset":3636,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":26112,"parent_record_offset":26080,"ancestor_record_offset":3636,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":27144,"parent_record_offset":27120,"ancestor_record_offset":3636,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":26512,"parent_record_offset":26488,"ancestor_record_offset":3636,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":26136,"parent_record_offset":26112,"ancestor_record_offset":3636,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":5868,"parent_record_offset":5844,"ancestor_record_offset":3636,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":197,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":3636,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":197,"record_offset":22216,"parent_record_offset":20240,"ancestor_record_offset":3636,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":197,"record_offset":20648,"parent_record_offset":20240,"ancestor_record_offset":3636,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":197,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":3636,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":3636,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":4276,"parent_record_offset":4252,"ancestor_record_offset":3636,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":4888,"parent_record_offset":4688,"ancestor_record_offset":3636,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":37464,"parent_record_offset":37432,"ancestor_record_offset":3636,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":37376,"parent_record_offset":37352,"ancestor_record_offset":3636,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":37432,"parent_record_offset":37376,"ancestor_record_offset":3636,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":37404,"parent_record_offset":37376,"ancestor_record_offset":3636,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":37496,"parent_record_offset":37464,"ancestor_record_offset":3636,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":37236,"parent_record_offset":37212,"ancestor_record_offset":3636,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":4800,"parent_record_offset":4716,"ancestor_record_offset":3636,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":4744,"parent_record_offset":4716,"ancestor_record_offset":3636,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":5144,"parent_record_offset":4352,"ancestor_record_offset":3636,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":4716,"parent_record_offset":4688,"ancestor_record_offset":3636,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":3636,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":4304,"parent_record_offset":4276,"ancestor_record_offset":3636,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":4352,"parent_record_offset":4328,"ancestor_record_offset":3636,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":4568,"parent_record_offset":4540,"ancestor_record_offset":3636,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":4596,"parent_record_offset":4568,"ancestor_record_offset":3636,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":4656,"parent_record_offset":4540,"ancestor_record_offset":3636,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":4540,"parent_record_offset":4352,"ancestor_record_offset":3636,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":4688,"parent_record_offset":4352,"ancestor_record_offset":3636,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":4400,"parent_record_offset":4352,"ancestor_record_offset":3636,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=36
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":4484,"parent_record_offset":4352,"ancestor_record_offset":3636,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":5060,"parent_record_offset":4352,"ancestor_record_offset":3636,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":197,"record_offset":26056,"parent_record_offset":26012,"ancestor_record_offset":3636,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":197,"record_offset":26436,"parent_record_offset":26412,"ancestor_record_offset":3636,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":197,"record_offset":27260,"parent_record_offset":27084,"ancestor_record_offset":3636,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":197,"record_offset":27120,"parent_record_offset":27084,"ancestor_record_offset":3636,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":197,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":3636,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":197,"record_offset":26080,"parent_record_offset":26056,"ancestor_record_offset":3636,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":197,"record_offset":26460,"parent_record_offset":26436,"ancestor_record_offset":3636,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":197,"record_offset":11276,"parent_record_offset":11228,"ancestor_record_offset":3636,"inlinee":7507,"source_line":26,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":197,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":3636,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":197,"record_offset":11428,"parent_record_offset":11404,"ancestor_record_offset":3636,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":197,"record_offset":13436,"parent_record_offset":13396,"ancestor_record_offset":3636,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":3636,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3636,"inlinee":7446,"source_line":100,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::temporal::Temporal::as_ref
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":197,"record_offset":22612,"parent_record_offset":20240,"ancestor_record_offset":3636,"inlinee":8380,"source_line":146,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::temporal::Temporal::split_pair
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":197,"record_offset":22584,"parent_record_offset":20240,"ancestor_record_offset":3636,"inlinee":8381,"source_line":158,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_frame_render_graph::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":18244,"parent_record_offset":18208,"ancestor_record_offset":3636,"inlinee":12653,"source_line":178,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_frame_render_graph::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":18416,"parent_record_offset":18380,"ancestor_record_offset":3636,"inlinee":12654,"source_line":186,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_frame_render_graph::explain
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3174
// 首条位置: {"module":197,"inlinee":12660}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":38540,"parent_record_offset":38424,"ancestor_record_offset":38424,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":197,"record_offset":38568,"parent_record_offset":38424,"ancestor_record_offset":38424,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":197,"record_offset":38596,"parent_record_offset":38424,"ancestor_record_offset":38424,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":197,"record_offset":38456,"parent_record_offset":38424,"ancestor_record_offset":38424,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":38480,"parent_record_offset":38456,"ancestor_record_offset":38424,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":38504,"parent_record_offset":38480,"ancestor_record_offset":38424,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_frame_render_graph::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 178
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x316D
// 首条位置: {"module":197,"inlinee":12653}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":18244,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":197,"record_offset":18340,"parent_record_offset":18272,"ancestor_record_offset":18244,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":197,"record_offset":18312,"parent_record_offset":18272,"ancestor_record_offset":18244,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_frame_render_graph::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x316E
// 首条位置: {"module":197,"inlinee":12654}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":18444,"parent_record_offset":18416,"ancestor_record_offset":18416,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":197,"record_offset":18512,"parent_record_offset":18444,"ancestor_record_offset":18416,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":197,"record_offset":18484,"parent_record_offset":18444,"ancestor_record_offset":18416,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_loading_screen_frame_render_graph
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 415
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3183
// 首条位置: {"module":197,"inlinee":12675}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":197,"record_offset":52528,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":52700,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":50784,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":63712,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":197,"record_offset":63684,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":50484,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":65476,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":56420,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":70276,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":50604,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":50512,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":69360,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":197,"record_offset":65716,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":197,"record_offset":65672,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":197,"record_offset":62116,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":197,"record_offset":52640,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":12651,"source_line":11,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":197,"record_offset":56036,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":7522,"source_line":144,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::RenderGraph::record
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":197,"record_offset":50548,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":7460,"source_line":107,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":62016,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":65760,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":8356,"source_line":69,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::finish
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":69332,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":9082,"source_line":96,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import_lazy_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":65504,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":9080,"source_line":77,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::TemporalRenderGraphRecorder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":197,"record_offset":50756,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":8384,"source_line":117,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":197,"record_offset":58624,"parent_record_offset":58600,"ancestor_record_offset":50140,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":197,"record_offset":58600,"parent_record_offset":58544,"ancestor_record_offset":50140,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":58572,"parent_record_offset":58544,"ancestor_record_offset":50140,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":65604,"parent_record_offset":65572,"ancestor_record_offset":50140,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":58544,"parent_record_offset":58516,"ancestor_record_offset":50140,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":65572,"parent_record_offset":65536,"ancestor_record_offset":50140,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":65536,"parent_record_offset":65504,"ancestor_record_offset":50140,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":197,"record_offset":52552,"parent_record_offset":52528,"ancestor_record_offset":50140,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":197,"record_offset":61072,"parent_record_offset":56464,"ancestor_record_offset":50140,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":197,"record_offset":50836,"parent_record_offset":50808,"ancestor_record_offset":50140,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":52360,"parent_record_offset":52336,"ancestor_record_offset":50140,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":52336,"parent_record_offset":52312,"ancestor_record_offset":50140,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":52284,"parent_record_offset":52252,"ancestor_record_offset":50140,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":54140,"parent_record_offset":53752,"ancestor_record_offset":50140,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":52228,"parent_record_offset":51836,"ancestor_record_offset":50140,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":52252,"parent_record_offset":52228,"ancestor_record_offset":50140,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":51868,"parent_record_offset":51836,"ancestor_record_offset":50140,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":52312,"parent_record_offset":52252,"ancestor_record_offset":50140,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":51916,"parent_record_offset":51892,"ancestor_record_offset":50140,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":51892,"parent_record_offset":51868,"ancestor_record_offset":50140,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":52008,"parent_record_offset":51944,"ancestor_record_offset":50140,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":52096,"parent_record_offset":52072,"ancestor_record_offset":50140,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":52428,"parent_record_offset":52404,"ancestor_record_offset":50140,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":52404,"parent_record_offset":51836,"ancestor_record_offset":50140,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":52724,"parent_record_offset":52700,"ancestor_record_offset":50140,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":53752,"parent_record_offset":52724,"ancestor_record_offset":50140,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":50808,"parent_record_offset":50784,"ancestor_record_offset":50140,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":51836,"parent_record_offset":50808,"ancestor_record_offset":50140,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":197,"record_offset":51568,"parent_record_offset":51544,"ancestor_record_offset":50140,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":51616,"parent_record_offset":51592,"ancestor_record_offset":50140,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":51544,"parent_record_offset":51520,"ancestor_record_offset":50140,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":197,"record_offset":62084,"parent_record_offset":62016,"ancestor_record_offset":50140,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":197,"record_offset":51032,"parent_record_offset":51008,"ancestor_record_offset":50140,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":197,"record_offset":51116,"parent_record_offset":51092,"ancestor_record_offset":50140,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":197,"record_offset":51056,"parent_record_offset":51032,"ancestor_record_offset":50140,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":197,"record_offset":56264,"parent_record_offset":56236,"ancestor_record_offset":50140,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":197,"record_offset":50628,"parent_record_offset":50604,"ancestor_record_offset":50140,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":69776,"parent_record_offset":69748,"ancestor_record_offset":50140,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":69800,"parent_record_offset":69776,"ancestor_record_offset":50140,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":197,"record_offset":51228,"parent_record_offset":51204,"ancestor_record_offset":50140,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":52032,"parent_record_offset":52008,"ancestor_record_offset":50140,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":51804,"parent_record_offset":50836,"ancestor_record_offset":50140,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":52072,"parent_record_offset":51892,"ancestor_record_offset":50140,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":61392,"parent_record_offset":56464,"ancestor_record_offset":50140,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":58260,"parent_record_offset":56464,"ancestor_record_offset":50140,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":197,"record_offset":52176,"parent_record_offset":52152,"ancestor_record_offset":50140,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":56188,"parent_record_offset":56036,"ancestor_record_offset":50140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":51456,"parent_record_offset":51432,"ancestor_record_offset":50140,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":51376,"parent_record_offset":51352,"ancestor_record_offset":50140,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":50656,"parent_record_offset":50628,"ancestor_record_offset":50140,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":50684,"parent_record_offset":50628,"ancestor_record_offset":50140,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":58152,"parent_record_offset":56464,"ancestor_record_offset":50140,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":56116,"parent_record_offset":56084,"ancestor_record_offset":50140,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":58124,"parent_record_offset":56464,"ancestor_record_offset":50140,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":51944,"parent_record_offset":51916,"ancestor_record_offset":50140,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":51980,"parent_record_offset":51944,"ancestor_record_offset":50140,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":52152,"parent_record_offset":52120,"ancestor_record_offset":50140,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":52452,"parent_record_offset":52428,"ancestor_record_offset":50140,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":52476,"parent_record_offset":52452,"ancestor_record_offset":50140,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":197,"record_offset":51592,"parent_record_offset":51568,"ancestor_record_offset":50140,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":197,"record_offset":51520,"parent_record_offset":51496,"ancestor_record_offset":50140,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":50860,"parent_record_offset":50836,"ancestor_record_offset":50140,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":50884,"parent_record_offset":50860,"ancestor_record_offset":50140,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":51496,"parent_record_offset":51296,"ancestor_record_offset":50140,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69716,"parent_record_offset":69684,"ancestor_record_offset":50140,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69628,"parent_record_offset":69604,"ancestor_record_offset":50140,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69684,"parent_record_offset":69628,"ancestor_record_offset":50140,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69656,"parent_record_offset":69628,"ancestor_record_offset":50140,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69748,"parent_record_offset":69716,"ancestor_record_offset":50140,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69488,"parent_record_offset":69464,"ancestor_record_offset":50140,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":51408,"parent_record_offset":51324,"ancestor_record_offset":50140,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":51352,"parent_record_offset":51324,"ancestor_record_offset":50140,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":51752,"parent_record_offset":50960,"ancestor_record_offset":50140,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":51324,"parent_record_offset":51296,"ancestor_record_offset":50140,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":50936,"parent_record_offset":50912,"ancestor_record_offset":50140,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":50912,"parent_record_offset":50884,"ancestor_record_offset":50140,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":50960,"parent_record_offset":50936,"ancestor_record_offset":50140,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":51176,"parent_record_offset":51148,"ancestor_record_offset":50140,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":51204,"parent_record_offset":51176,"ancestor_record_offset":50140,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":51264,"parent_record_offset":51148,"ancestor_record_offset":50140,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":51148,"parent_record_offset":50960,"ancestor_record_offset":50140,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":51296,"parent_record_offset":50960,"ancestor_record_offset":50140,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":51008,"parent_record_offset":50960,"ancestor_record_offset":50140,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":51092,"parent_record_offset":50960,"ancestor_record_offset":50140,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":51668,"parent_record_offset":50960,"ancestor_record_offset":50140,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":197,"record_offset":58292,"parent_record_offset":56464,"ancestor_record_offset":50140,"inlinee":8680,"source_line":688,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":197,"record_offset":52120,"parent_record_offset":52096,"ancestor_record_offset":50140,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":197,"record_offset":56084,"parent_record_offset":56036,"ancestor_record_offset":50140,"inlinee":7507,"source_line":26,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":197,"record_offset":52664,"parent_record_offset":52640,"ancestor_record_offset":50140,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":197,"record_offset":56236,"parent_record_offset":56212,"ancestor_record_offset":50140,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":197,"record_offset":61128,"parent_record_offset":56464,"ancestor_record_offset":50140,"inlinee":12247,"source_line":23,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":197,"record_offset":62056,"parent_record_offset":62016,"ancestor_record_offset":50140,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":50572,"parent_record_offset":50548,"ancestor_record_offset":50140,"inlinee":7446,"source_line":100,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_loading_screen_frame_render_graph::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":56464,"parent_record_offset":56420,"ancestor_record_offset":50140,"inlinee":12677,"source_line":453,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_loading_screen_frame_render_graph::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 453
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3185
// 首条位置: {"module":197,"inlinee":12677}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":197,"record_offset":61072,"parent_record_offset":56464,"ancestor_record_offset":56464,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":56592,"parent_record_offset":56464,"ancestor_record_offset":56464,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":59340,"parent_record_offset":56464,"ancestor_record_offset":56464,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":61392,"parent_record_offset":56464,"ancestor_record_offset":56464,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":58260,"parent_record_offset":56464,"ancestor_record_offset":56464,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":59168,"parent_record_offset":56464,"ancestor_record_offset":56464,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":58152,"parent_record_offset":56464,"ancestor_record_offset":56464,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":58124,"parent_record_offset":56464,"ancestor_record_offset":56464,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":197,"record_offset":58292,"parent_record_offset":56464,"ancestor_record_offset":56464,"inlinee":8680,"source_line":688,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":197,"record_offset":61128,"parent_record_offset":56464,"ancestor_record_offset":56464,"inlinee":12247,"source_line":23,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":197,"record_offset":58624,"parent_record_offset":58600,"ancestor_record_offset":56464,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":197,"record_offset":58600,"parent_record_offset":58544,"ancestor_record_offset":56464,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":58572,"parent_record_offset":58544,"ancestor_record_offset":56464,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":58544,"parent_record_offset":58516,"ancestor_record_offset":56464,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":197,"record_offset":61096,"parent_record_offset":61072,"ancestor_record_offset":56464,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":197,"record_offset":56628,"parent_record_offset":56592,"ancestor_record_offset":56464,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":60908,"parent_record_offset":60884,"ancestor_record_offset":56464,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":60884,"parent_record_offset":60860,"ancestor_record_offset":56464,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":58080,"parent_record_offset":58048,"ancestor_record_offset":56464,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":58024,"parent_record_offset":57636,"ancestor_record_offset":56464,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":60776,"parent_record_offset":60384,"ancestor_record_offset":56464,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":58048,"parent_record_offset":58024,"ancestor_record_offset":56464,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":57664,"parent_record_offset":57636,"ancestor_record_offset":56464,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":60860,"parent_record_offset":60800,"ancestor_record_offset":56464,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":57712,"parent_record_offset":57688,"ancestor_record_offset":56464,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":57688,"parent_record_offset":57664,"ancestor_record_offset":56464,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":57804,"parent_record_offset":57740,"ancestor_record_offset":56464,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":57892,"parent_record_offset":57868,"ancestor_record_offset":56464,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":60976,"parent_record_offset":60952,"ancestor_record_offset":56464,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":60952,"parent_record_offset":60384,"ancestor_record_offset":56464,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":57636,"parent_record_offset":56592,"ancestor_record_offset":56464,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":60384,"parent_record_offset":59340,"ancestor_record_offset":56464,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":197,"record_offset":57368,"parent_record_offset":57344,"ancestor_record_offset":56464,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":57416,"parent_record_offset":57392,"ancestor_record_offset":56464,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":57344,"parent_record_offset":57320,"ancestor_record_offset":56464,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":197,"record_offset":56832,"parent_record_offset":56808,"ancestor_record_offset":56464,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":197,"record_offset":56916,"parent_record_offset":56892,"ancestor_record_offset":56464,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":197,"record_offset":56856,"parent_record_offset":56832,"ancestor_record_offset":56464,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":197,"record_offset":57028,"parent_record_offset":57004,"ancestor_record_offset":56464,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":57828,"parent_record_offset":57804,"ancestor_record_offset":56464,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":57604,"parent_record_offset":56628,"ancestor_record_offset":56464,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":57868,"parent_record_offset":57688,"ancestor_record_offset":56464,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":197,"record_offset":57972,"parent_record_offset":57948,"ancestor_record_offset":56464,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":58192,"parent_record_offset":58152,"ancestor_record_offset":56464,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":57256,"parent_record_offset":57232,"ancestor_record_offset":56464,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":57176,"parent_record_offset":57152,"ancestor_record_offset":56464,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":57740,"parent_record_offset":57712,"ancestor_record_offset":56464,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":57776,"parent_record_offset":57740,"ancestor_record_offset":56464,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":57948,"parent_record_offset":57916,"ancestor_record_offset":56464,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":61000,"parent_record_offset":60976,"ancestor_record_offset":56464,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":61024,"parent_record_offset":61000,"ancestor_record_offset":56464,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":197,"record_offset":57392,"parent_record_offset":57368,"ancestor_record_offset":56464,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":197,"record_offset":57320,"parent_record_offset":57296,"ancestor_record_offset":56464,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":56660,"parent_record_offset":56628,"ancestor_record_offset":56464,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":56684,"parent_record_offset":56660,"ancestor_record_offset":56464,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":57296,"parent_record_offset":57096,"ancestor_record_offset":56464,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":57208,"parent_record_offset":57124,"ancestor_record_offset":56464,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":57152,"parent_record_offset":57124,"ancestor_record_offset":56464,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":57552,"parent_record_offset":56760,"ancestor_record_offset":56464,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":57124,"parent_record_offset":57096,"ancestor_record_offset":56464,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":56736,"parent_record_offset":56712,"ancestor_record_offset":56464,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":56712,"parent_record_offset":56684,"ancestor_record_offset":56464,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":56760,"parent_record_offset":56736,"ancestor_record_offset":56464,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":56976,"parent_record_offset":56948,"ancestor_record_offset":56464,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":57004,"parent_record_offset":56976,"ancestor_record_offset":56464,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":57064,"parent_record_offset":56948,"ancestor_record_offset":56464,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":56948,"parent_record_offset":56760,"ancestor_record_offset":56464,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":57096,"parent_record_offset":56760,"ancestor_record_offset":56464,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":56808,"parent_record_offset":56760,"ancestor_record_offset":56464,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":56892,"parent_record_offset":56760,"ancestor_record_offset":56464,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":57468,"parent_record_offset":56760,"ancestor_record_offset":56464,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":197,"record_offset":57916,"parent_record_offset":57892,"ancestor_record_offset":56464,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":197,"record_offset":61156,"parent_record_offset":61128,"ancestor_record_offset":56464,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::impl$3::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 581
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3182
// 首条位置: {"module":197,"inlinee":12674}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 591
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":197,"record_offset":41428,"rva":3152112,"va":5371861232}
// 编译实例 RVAs: 0x3018f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Sub$GT$::sub
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5578080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8601d5500","size":5,"soft_fail":false,"target_va":5377444464,"va":5371866379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3146770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8eefbcfff","size":5,"soft_fail":false,"target_va":5368720672,"va":5371867437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400668f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2741067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8b52cd6ff","size":5,"soft_fail":false,"target_va":5369129200,"va":5371870262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400db9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2260433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e82f82ddff","size":5,"soft_fail":false,"target_va":5369608672,"va":5371869100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2230165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e86bf8ddff","size":5,"soft_fail":false,"target_va":5369632448,"va":5371862608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e1dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2228535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8c9feddff","size":5,"soft_fail":false,"target_va":5369634256,"va":5371862786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e2ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2225307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8650bdeff","size":5,"soft_fail":false,"target_va":5369637552,"va":5371862854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ff9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2118943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8e1aadfff","size":5,"soft_fail":false,"target_va":5369756144,"va":5371875082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a6e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1427414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e82a38eaff","size":5,"soft_fail":false,"target_va":5370441296,"va":5371868705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a85b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1421635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8bd4eeaff","size":5,"soft_fail":false,"target_va":5370447280,"va":5371868910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cc830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1273388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8d491ecff","size":5,"soft_fail":false,"target_va":5370595376,"va":5371868759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1271060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8ec9aecff","size":5,"soft_fail":false,"target_va":5370597792,"va":5371868847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14026aee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-622580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e80c80f6ff","size":5,"soft_fail":false,"target_va":5371244256,"va":5371866831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c10a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-273026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e87ed5fbff","size":5,"soft_fail":false,"target_va":5371596960,"va":5371869981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da640
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-177813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e86b49fdff","size":5,"soft_fail":false,"target_va":5371700800,"va":5371878608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402deb20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-152876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8d4aafdff","size":5,"soft_fail":false,"target_va":5371718432,"va":5371871303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035dea0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t362907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e89b890500","size":5,"soft_fail":false,"target_va":5372239520,"va":5371876608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361a60
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t377899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e82bc40500","size":5,"soft_fail":false,"target_va":5372254816,"va":5371876912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140365550
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t393157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8c5ff0500","size":5,"soft_fail":false,"target_va":5372269904,"va":5371876742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140366a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t397170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8720f0600","size":5,"soft_fail":false,"target_va":5372275296,"va":5371878121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140367730
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t402814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e87e250600","size":5,"soft_fail":false,"target_va":5372278576,"va":5371875757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t446798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e84ed10600","size":5,"soft_fail":false,"target_va":5372314208,"va":5371867405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140373640
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t460212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8b4050700","size":5,"soft_fail":false,"target_va":5372327488,"va":5371867271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140374a30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t465203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e833190700","size":5,"soft_fail":false,"target_va":5372332592,"va":5371867384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140376450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t471933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e87d330700","size":5,"soft_fail":false,"target_va":5372339280,"va":5371867342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403779e0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t471938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e882330700","size":5,"soft_fail":false,"target_va":5372344800,"va":5371872857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140378770
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t477927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8e74a0700","size":5,"soft_fail":false,"target_va":5372348272,"va":5371870340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140385b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t525234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8b2030800","size":5,"soft_fail":false,"target_va":5372402560,"va":5371877321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140386550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t527538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8b20c0800","size":5,"soft_fail":false,"target_va":5372405072,"va":5371877529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387490
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t536460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e88c2f0800","size":5,"soft_fail":false,"target_va":5372408976,"va":5371872511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t558830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8ee860800","size":5,"soft_fail":false,"target_va":5372426160,"va":5371867325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391980
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t575892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e894c90800","size":5,"soft_fail":false,"target_va":5372451200,"va":5371875303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140392800
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t584525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e84deb0800","size":5,"soft_fail":false,"target_va":5372454912,"va":5371870382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t882671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8ef770d00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371867036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1284688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8509a1300","size":5,"soft_fail":false,"target_va":5373149840,"va":5371865147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2648827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8fb6a2800","size":5,"soft_fail":false,"target_va":5374516480,"va":5371867648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4372466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8f2b74200","size":5,"soft_fail":false,"target_va":5376239920,"va":5371867449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407378d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4412123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8db524300","size":5,"soft_fail":false,"target_va":5376276688,"va":5371864560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140748250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4475627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8eb4a4400","size":5,"soft_fail":false,"target_va":5376344656,"va":5371869024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31038858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e88a9dd901","size":5,"soft_fail":false,"target_va":5402900112,"va":5371861249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a6510
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31070966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8f61ada01","size":5,"soft_fail":false,"target_va":5402944784,"va":5371873813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31730153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8e929e401","size":5,"soft_fail":false,"target_va":5403593360,"va":5371863202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-563678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e82266f7ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5371868553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-220226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8bea3fcff","size":5,"soft_fail":false,"target_va":5371652368,"va":5371872589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31105258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8eaa0da01","size":5,"soft_fail":false,"target_va":5402978688,"va":5371873425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31105705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8a9a2da01","size":5,"soft_fail":false,"target_va":5402978912,"va":5371873202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t31105702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8a6a2da01","size":5,"soft_fail":false,"target_va":5402978880,"va":5371873173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31105378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e862a1da01","size":5,"soft_fail":false,"target_va":5402979152,"va":5371873769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31105258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8eaa0da01","size":5,"soft_fail":false,"target_va":5402978944,"va":5371873681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-30573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e89388ffff","size":5,"soft_fail":false,"target_va":5371840832,"va":5371871400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-25977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8879affff","size":5,"soft_fail":false,"target_va":5371840720,"va":5371866692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31118370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e822d4da01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371861753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31106959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e88fa7da01","size":5,"soft_fail":false,"target_va":5402980464,"va":5371873500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e899b0ffff","size":5,"soft_fail":false,"target_va":5371841024,"va":5371861346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31119228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e87cd7da01","size":5,"soft_fail":false,"target_va":5402980656,"va":5371861423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3560491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e82b543600","size":5,"soft_fail":false,"target_va":5375430304,"va":5371869808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2491662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e80e052600","size":5,"soft_fail":false,"target_va":5374356208,"va":5371864541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2486153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e889ef2500","size":5,"soft_fail":false,"target_va":5374355680,"va":5371869522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2490899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e813022600","size":5,"soft_fail":false,"target_va":5374355424,"va":5371864520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2344660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8d4c62300","size":5,"soft_fail":false,"target_va":5374213616,"va":5371868951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::pass_resources::PassResources::new_from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2571362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8623c2700","size":5,"soft_fail":false,"target_va":5374442560,"va":5371871193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SavedSession::loading_background_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2390606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e84e7a2400","size":5,"soft_fail":false,"target_va":5374257120,"va":5371866509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::RenderContext::finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3536537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e899f63500","size":5,"soft_fail":false,"target_va":5375405616,"va":5371869074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2271984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8f0aa2200","size":5,"soft_fail":false,"target_va":5374135296,"va":5371863307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::sort
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2272210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8d2ab2200","size":5,"soft_fail":false,"target_va":5374135584,"va":5371863369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2272675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8a3ad2200","size":5,"soft_fail":false,"target_va":5374136016,"va":5371863336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3689336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8784b3800","size":5,"soft_fail":false,"target_va":5375557712,"va":5371868371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3797066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e84af03900","size":5,"soft_fail":false,"target_va":5375665488,"va":5371868417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::vermeer_render_from_target
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3383650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e862a13300","size":5,"soft_fail":false,"target_va":5375251984,"va":5371868329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba8
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4183098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e83ad43f00","size":5,"soft_fail":false,"target_va":5376049776,"va":5371866673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2697516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8d4d6d6ff","size":5,"soft_fail":false,"target_va":5369169024,"va":5371866535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5490053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e885c55300","size":5,"soft_fail":false,"target_va":5377353776,"va":5371863718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4494987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e88b964400","size":5,"soft_fail":false,"target_va":5376356912,"va":5371861920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4498874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8baa54400","size":5,"soft_fail":false,"target_va":5376360560,"va":5371861681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4494819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8e3954400","size":5,"soft_fail":false,"target_va":5376356144,"va":5371861320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4446493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e81dd94300","size":5,"soft_fail":false,"target_va":5376317936,"va":5371871438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4451762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8b2ed4300","size":5,"soft_fail":false,"target_va":5376317648,"va":5371865881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4397187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e883184300","size":5,"soft_fail":false,"target_va":5376262400,"va":5371865208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4396039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e807144300","size":5,"soft_fail":false,"target_va":5376261408,"va":5371865364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::extent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4414590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e87e5c4300","size":5,"soft_fail":false,"target_va":5376278016,"va":5371863421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::format
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4413958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8065a4300","size":5,"soft_fail":false,"target_va":5376277360,"va":5371863397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4472826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8fa3f4400","size":5,"soft_fail":false,"target_va":5376341760,"va":5371868929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4473783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8b7434400","size":5,"soft_fail":false,"target_va":5376340512,"va":5371866724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sample_count
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4472781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8cd3f4400","size":5,"soft_fail":false,"target_va":5376341264,"va":5371868478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4473962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e86a444400","size":5,"soft_fail":false,"target_va":5376340720,"va":5371866753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::GraphExecutor::last_known_frame_duration_ns
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4401858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8c22a4300","size":5,"soft_fail":false,"target_va":5376273360,"va":5371871497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::GraphExecutor::wait_gpu_queue_trim
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4399921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e831234300","size":5,"soft_fail":false,"target_va":5376272400,"va":5371872474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::GraphExecutor::wait_submits_done
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4400821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8b5264300","size":5,"soft_fail":false,"target_va":5376271312,"va":5371870486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4351852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e86c674200","size":5,"soft_fail":false,"target_va":5376219552,"va":5371867695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_resource_handle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4350782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e83e634200","size":5,"soft_fail":false,"target_va":5376219312,"va":5371868525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3d9350ee38791ca5 (.llvm.8305701337189838128)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t763462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e846a60b00","size":5,"soft_fail":false,"target_va":5372625168,"va":5371861701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5572285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8bd065500","size":5,"soft_fail":false,"target_va":5377443760,"va":5371871470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5576581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e885175500","size":5,"soft_fail":false,"target_va":5377442944,"va":5371866358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::render_loop::compile_frame_render_graph
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e88dacffff","size":5,"soft_fail":false,"target_va":5371842240,"va":5371863598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::finish_screenshot_capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1733952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371861232,"raw_hex":"e8c08ae5ff","size":5,"soft_fail":false,"target_va":5370137472,"va":5371871419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":197,"record_offset":45864,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":71104,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$10::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":197,"record_offset":71068,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":12683,"source_line":886,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":197,"record_offset":70816,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":44320,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":48372,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":48140,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":48288,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::f64::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":73844,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":11656,"source_line":935,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":197,"record_offset":48316,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":197,"record_offset":74540,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":48048,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":74484,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":73228,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":12684,"source_line":1444,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":48080,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":70968,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":73256,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":74456,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":73372,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":70932,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":73536,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_secs_f64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":197,"record_offset":73872,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":12687,"source_line":931,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_config::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":197,"record_offset":47496,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":12662,"source_line":80,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_lists::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs
//     evidence={"module":197,"record_offset":70872,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":12679,"source_line":98,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":197,"record_offset":48344,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":197,"record_offset":48204,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":43252,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":41664,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":42240,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::SmolRingBuffer::push
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":73400,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":11095,"source_line":838,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::compile_loading_screen_frame_render_graph
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":50140,"parent_record_offset":41428,"ancestor_record_offset":41428,"inlinee":12675,"source_line":415,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":197,"record_offset":58624,"parent_record_offset":58600,"ancestor_record_offset":41428,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":197,"record_offset":58600,"parent_record_offset":58544,"ancestor_record_offset":41428,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":58572,"parent_record_offset":58544,"ancestor_record_offset":41428,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":65604,"parent_record_offset":65572,"ancestor_record_offset":41428,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":58544,"parent_record_offset":58516,"ancestor_record_offset":41428,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":197,"record_offset":74300,"parent_record_offset":74260,"ancestor_record_offset":41428,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":197,"record_offset":74260,"parent_record_offset":74232,"ancestor_record_offset":41428,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":65572,"parent_record_offset":65536,"ancestor_record_offset":41428,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":65536,"parent_record_offset":65504,"ancestor_record_offset":41428,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":197,"record_offset":74616,"parent_record_offset":74592,"ancestor_record_offset":41428,"inlinee":12694,"source_line":544,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$13::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":197,"record_offset":74592,"parent_record_offset":74568,"ancestor_record_offset":41428,"inlinee":12693,"source_line":921,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":197,"record_offset":52552,"parent_record_offset":52528,"ancestor_record_offset":41428,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":197,"record_offset":52528,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":197,"record_offset":44372,"parent_record_offset":44344,"ancestor_record_offset":41428,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":49972,"parent_record_offset":49948,"ancestor_record_offset":41428,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":49948,"parent_record_offset":49924,"ancestor_record_offset":41428,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":45816,"parent_record_offset":45784,"ancestor_record_offset":41428,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":45760,"parent_record_offset":45372,"ancestor_record_offset":41428,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":49840,"parent_record_offset":49448,"ancestor_record_offset":41428,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":45784,"parent_record_offset":45760,"ancestor_record_offset":41428,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":45400,"parent_record_offset":45372,"ancestor_record_offset":41428,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":49924,"parent_record_offset":49864,"ancestor_record_offset":41428,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":45448,"parent_record_offset":45424,"ancestor_record_offset":41428,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":45424,"parent_record_offset":45400,"ancestor_record_offset":41428,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":45540,"parent_record_offset":45476,"ancestor_record_offset":41428,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":45628,"parent_record_offset":45604,"ancestor_record_offset":41428,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":52700,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":50784,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":197,"record_offset":50040,"parent_record_offset":50016,"ancestor_record_offset":41428,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":50016,"parent_record_offset":49448,"ancestor_record_offset":41428,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":44344,"parent_record_offset":44320,"ancestor_record_offset":41428,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":45372,"parent_record_offset":44344,"ancestor_record_offset":41428,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":48404,"parent_record_offset":48372,"ancestor_record_offset":41428,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":49448,"parent_record_offset":48404,"ancestor_record_offset":41428,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":197,"record_offset":45104,"parent_record_offset":45080,"ancestor_record_offset":41428,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$45::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":197,"record_offset":70900,"parent_record_offset":70872,"ancestor_record_offset":41428,"inlinee":7537,"source_line":439,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":43672,"parent_record_offset":43644,"ancestor_record_offset":41428,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":43724,"parent_record_offset":43700,"ancestor_record_offset":41428,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":43700,"parent_record_offset":43672,"ancestor_record_offset":41428,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":43612,"parent_record_offset":43580,"ancestor_record_offset":41428,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":43644,"parent_record_offset":43612,"ancestor_record_offset":41428,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":43816,"parent_record_offset":43644,"ancestor_record_offset":41428,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":43928,"parent_record_offset":43904,"ancestor_record_offset":41428,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":45152,"parent_record_offset":45128,"ancestor_record_offset":41428,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":45080,"parent_record_offset":45056,"ancestor_record_offset":41428,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":197,"record_offset":62084,"parent_record_offset":62016,"ancestor_record_offset":41428,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":197,"record_offset":44568,"parent_record_offset":44544,"ancestor_record_offset":41428,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":197,"record_offset":44652,"parent_record_offset":44628,"ancestor_record_offset":41428,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":197,"record_offset":63684,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":73796,"parent_record_offset":73760,"ancestor_record_offset":41428,"inlinee":11656,"source_line":935,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":73624,"parent_record_offset":73588,"ancestor_record_offset":41428,"inlinee":11657,"source_line":955,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":73936,"parent_record_offset":73904,"ancestor_record_offset":41428,"inlinee":12689,"source_line":1121,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits::rt_f64_to_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":73960,"parent_record_offset":73936,"ancestor_record_offset":41428,"inlinee":12691,"source_line":1143,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":197,"record_offset":44592,"parent_record_offset":44568,"ancestor_record_offset":41428,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":197,"record_offset":56264,"parent_record_offset":56236,"ancestor_record_offset":41428,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":197,"record_offset":43748,"parent_record_offset":43724,"ancestor_record_offset":41428,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":69776,"parent_record_offset":69748,"ancestor_record_offset":41428,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":69800,"parent_record_offset":69776,"ancestor_record_offset":41428,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":73436,"parent_record_offset":73400,"ancestor_record_offset":41428,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":197,"record_offset":41876,"parent_record_offset":41780,"ancestor_record_offset":41428,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":197,"record_offset":44764,"parent_record_offset":44740,"ancestor_record_offset":41428,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":50484,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":43504,"parent_record_offset":43476,"ancestor_record_offset":41428,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":45340,"parent_record_offset":44372,"ancestor_record_offset":41428,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":56420,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":42836,"parent_record_offset":42376,"ancestor_record_offset":41428,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":43552,"parent_record_offset":43372,"ancestor_record_offset":41428,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":70276,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":50604,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":50512,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":58260,"parent_record_offset":56464,"ancestor_record_offset":41428,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":47696,"parent_record_offset":47632,"ancestor_record_offset":41428,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":197,"record_offset":42188,"parent_record_offset":42160,"ancestor_record_offset":41428,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":43880,"parent_record_offset":43580,"ancestor_record_offset":41428,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":44992,"parent_record_offset":44968,"ancestor_record_offset":41428,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":44912,"parent_record_offset":44888,"ancestor_record_offset":41428,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":74352,"parent_record_offset":74300,"ancestor_record_offset":41428,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":42612,"parent_record_offset":42588,"ancestor_record_offset":41428,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":42588,"parent_record_offset":42564,"ancestor_record_offset":41428,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":42796,"parent_record_offset":42764,"ancestor_record_offset":41428,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":50656,"parent_record_offset":50628,"ancestor_record_offset":41428,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":43772,"parent_record_offset":43748,"ancestor_record_offset":41428,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":44280,"parent_record_offset":43332,"ancestor_record_offset":41428,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":58152,"parent_record_offset":56464,"ancestor_record_offset":41428,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":56116,"parent_record_offset":56084,"ancestor_record_offset":41428,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":58124,"parent_record_offset":56464,"ancestor_record_offset":41428,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":41812,"parent_record_offset":41780,"ancestor_record_offset":41428,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":41840,"parent_record_offset":41812,"ancestor_record_offset":41428,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":45684,"parent_record_offset":45652,"ancestor_record_offset":41428,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":42564,"parent_record_offset":42540,"ancestor_record_offset":41428,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":42892,"parent_record_offset":42868,"ancestor_record_offset":41428,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":42540,"parent_record_offset":42432,"ancestor_record_offset":41428,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":197,"record_offset":41780,"parent_record_offset":41752,"ancestor_record_offset":41428,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":197,"record_offset":42272,"parent_record_offset":42240,"ancestor_record_offset":41428,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":197,"record_offset":42404,"parent_record_offset":42376,"ancestor_record_offset":41428,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":197,"record_offset":42376,"parent_record_offset":42344,"ancestor_record_offset":41428,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":197,"record_offset":42320,"parent_record_offset":42296,"ancestor_record_offset":41428,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":197,"record_offset":42344,"parent_record_offset":42320,"ancestor_record_offset":41428,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":197,"record_offset":42296,"parent_record_offset":42272,"ancestor_record_offset":41428,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":197,"record_offset":42868,"parent_record_offset":42344,"ancestor_record_offset":41428,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":41976,"parent_record_offset":41944,"ancestor_record_offset":41428,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":42136,"parent_record_offset":41976,"ancestor_record_offset":41428,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":41944,"parent_record_offset":41920,"ancestor_record_offset":41428,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":41696,"parent_record_offset":41664,"ancestor_record_offset":41428,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":42160,"parent_record_offset":42136,"ancestor_record_offset":41428,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":41752,"parent_record_offset":41720,"ancestor_record_offset":41428,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":41720,"parent_record_offset":41696,"ancestor_record_offset":41428,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":197,"record_offset":42432,"parent_record_offset":42404,"ancestor_record_offset":41428,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":197,"record_offset":42680,"parent_record_offset":42432,"ancestor_record_offset":41428,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":197,"record_offset":42652,"parent_record_offset":42432,"ancestor_record_offset":41428,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":197,"record_offset":42484,"parent_record_offset":42432,"ancestor_record_offset":41428,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":50064,"parent_record_offset":50040,"ancestor_record_offset":41428,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":45892,"parent_record_offset":45864,"ancestor_record_offset":41428,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":48228,"parent_record_offset":48204,"ancestor_record_offset":41428,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":45916,"parent_record_offset":45892,"ancestor_record_offset":41428,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":48252,"parent_record_offset":48228,"ancestor_record_offset":41428,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":74384,"parent_record_offset":74300,"ancestor_record_offset":41428,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":74420,"parent_record_offset":74260,"ancestor_record_offset":41428,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::try_from_secs_f64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":197,"record_offset":73904,"parent_record_offset":73872,"ancestor_record_offset":41428,"inlinee":12688,"source_line":1678,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":197,"record_offset":45128,"parent_record_offset":45104,"ancestor_record_offset":41428,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":197,"record_offset":47924,"parent_record_offset":47496,"ancestor_record_offset":41428,"inlinee":12668,"source_line":9,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::impl$6::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":197,"record_offset":47972,"parent_record_offset":47496,"ancestor_record_offset":41428,"inlinee":12670,"source_line":49,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":197,"record_offset":65716,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":197,"record_offset":65672,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":197,"record_offset":47724,"parent_record_offset":47696,"ancestor_record_offset":41428,"inlinee":10713,"source_line":29,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":197,"record_offset":47632,"parent_record_offset":47564,"ancestor_record_offset":41428,"inlinee":10709,"source_line":43,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$24::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":197,"record_offset":47564,"parent_record_offset":47496,"ancestor_record_offset":41428,"inlinee":10707,"source_line":75,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":197,"record_offset":47668,"parent_record_offset":47632,"ancestor_record_offset":41428,"inlinee":10711,"source_line":13,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_settings::impl$0::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_settings.rs
//     evidence={"module":197,"record_offset":47868,"parent_record_offset":47496,"ancestor_record_offset":41428,"inlinee":12664,"source_line":4,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":197,"record_offset":47896,"parent_record_offset":47496,"ancestor_record_offset":41428,"inlinee":12666,"source_line":5,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":197,"record_offset":62116,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":197,"record_offset":45056,"parent_record_offset":45032,"ancestor_record_offset":41428,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":44396,"parent_record_offset":44372,"ancestor_record_offset":41428,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":44420,"parent_record_offset":44396,"ancestor_record_offset":41428,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":45032,"parent_record_offset":44832,"ancestor_record_offset":41428,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69716,"parent_record_offset":69684,"ancestor_record_offset":41428,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69628,"parent_record_offset":69604,"ancestor_record_offset":41428,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69684,"parent_record_offset":69628,"ancestor_record_offset":41428,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69656,"parent_record_offset":69628,"ancestor_record_offset":41428,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69748,"parent_record_offset":69716,"ancestor_record_offset":41428,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":197,"record_offset":69488,"parent_record_offset":69464,"ancestor_record_offset":41428,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":44944,"parent_record_offset":44860,"ancestor_record_offset":41428,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":44888,"parent_record_offset":44860,"ancestor_record_offset":41428,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":45288,"parent_record_offset":44496,"ancestor_record_offset":41428,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":44860,"parent_record_offset":44832,"ancestor_record_offset":41428,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":44472,"parent_record_offset":44448,"ancestor_record_offset":41428,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":44448,"parent_record_offset":44420,"ancestor_record_offset":41428,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":44496,"parent_record_offset":44472,"ancestor_record_offset":41428,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":44712,"parent_record_offset":44684,"ancestor_record_offset":41428,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":44740,"parent_record_offset":44712,"ancestor_record_offset":41428,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":44800,"parent_record_offset":44684,"ancestor_record_offset":41428,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":44684,"parent_record_offset":44496,"ancestor_record_offset":41428,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":44832,"parent_record_offset":44496,"ancestor_record_offset":41428,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":44544,"parent_record_offset":44496,"ancestor_record_offset":41428,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":44628,"parent_record_offset":44496,"ancestor_record_offset":41428,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":45204,"parent_record_offset":44496,"ancestor_record_offset":41428,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":197,"record_offset":58292,"parent_record_offset":56464,"ancestor_record_offset":41428,"inlinee":8680,"source_line":688,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":197,"record_offset":45652,"parent_record_offset":45628,"ancestor_record_offset":41428,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":43292,"parent_record_offset":43252,"ancestor_record_offset":41428,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":197,"record_offset":43416,"parent_record_offset":43372,"ancestor_record_offset":41428,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":43580,"parent_record_offset":43372,"ancestor_record_offset":41428,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":43852,"parent_record_offset":43580,"ancestor_record_offset":41428,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":197,"record_offset":52640,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":12651,"source_line":11,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstants::push
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":197,"record_offset":56036,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":7522,"source_line":144,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":197,"record_offset":56084,"parent_record_offset":56036,"ancestor_record_offset":41428,"inlinee":7507,"source_line":26,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":197,"record_offset":47832,"parent_record_offset":47564,"ancestor_record_offset":41428,"inlinee":10594,"source_line":87,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":197,"record_offset":52664,"parent_record_offset":52640,"ancestor_record_offset":41428,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":197,"record_offset":56236,"parent_record_offset":56212,"ancestor_record_offset":41428,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":197,"record_offset":61128,"parent_record_offset":56464,"ancestor_record_offset":41428,"inlinee":12247,"source_line":23,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":197,"record_offset":62056,"parent_record_offset":62016,"ancestor_record_offset":41428,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::RenderGraph::record
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":197,"record_offset":50548,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":7460,"source_line":107,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":62016,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":65760,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":8356,"source_line":69,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::finish
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":69332,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":9082,"source_line":96,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::import_lazy_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":65504,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":9080,"source_line":77,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":50572,"parent_record_offset":50548,"ancestor_record_offset":41428,"inlinee":7446,"source_line":100,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::temporal::TemporalRenderGraphRecorder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":197,"record_offset":50756,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":8384,"source_line":117,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":197,"record_offset":43444,"parent_record_offset":43416,"ancestor_record_offset":41428,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":197,"record_offset":43476,"parent_record_offset":43444,"ancestor_record_offset":41428,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":197,"record_offset":43372,"parent_record_offset":43332,"ancestor_record_offset":41428,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":197,"record_offset":43332,"parent_record_offset":43292,"ancestor_record_offset":41428,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_loading_screen_frame_render_graph::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":56464,"parent_record_offset":56420,"ancestor_record_offset":41428,"inlinee":12677,"source_line":453,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":48168,"parent_record_offset":48140,"ancestor_record_offset":41428,"inlinee":12674,"source_line":581,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":48108,"parent_record_offset":48080,"ancestor_record_offset":41428,"inlinee":12672,"source_line":626,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":71000,"parent_record_offset":70968,"ancestor_record_offset":41428,"inlinee":12680,"source_line":702,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":73588,"parent_record_offset":73536,"ancestor_record_offset":41428,"inlinee":12685,"source_line":792,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":73760,"parent_record_offset":73672,"ancestor_record_offset":41428,"inlinee":12686,"source_line":797,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 626
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3180
// 首条位置: {"module":197,"inlinee":12672}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 702
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3188
// 首条位置: {"module":197,"inlinee":12680}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 792
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x318D
// 首条位置: {"module":197,"inlinee":12685}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f64::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":73624,"parent_record_offset":73588,"ancestor_record_offset":73588,"inlinee":11657,"source_line":955,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 797
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x318E
// 首条位置: {"module":197,"inlinee":12686}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f64::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":73796,"parent_record_offset":73760,"ancestor_record_offset":73760,"inlinee":11656,"source_line":935,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::handle_screenshot_error
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 816
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3173
// 首条位置: {"module":197,"inlinee":12659}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":27204,"parent_record_offset":27084,"ancestor_record_offset":27084,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":197,"record_offset":27232,"parent_record_offset":27084,"ancestor_record_offset":27084,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":27288,"parent_record_offset":27084,"ancestor_record_offset":27084,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":197,"record_offset":27260,"parent_record_offset":27084,"ancestor_record_offset":27084,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":197,"record_offset":27120,"parent_record_offset":27084,"ancestor_record_offset":27084,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":27144,"parent_record_offset":27120,"ancestor_record_offset":27084,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":27168,"parent_record_offset":27144,"ancestor_record_offset":27084,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::SmolRingBuffer::push
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 838
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B57
// 首条位置: {"module":197,"inlinee":11095}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":73436,"parent_record_offset":73400,"ancestor_record_offset":73400,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 853
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3170
// 首条位置: {"module":197,"inlinee":12656}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":22244,"parent_record_offset":20240,"ancestor_record_offset":20240,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":197,"record_offset":22216,"parent_record_offset":20240,"ancestor_record_offset":20240,"inlinee":12658,"source_line":205,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":197,"record_offset":20648,"parent_record_offset":20240,"ancestor_record_offset":20240,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":22316,"parent_record_offset":20240,"ancestor_record_offset":20240,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::Temporal::as_ref
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":197,"record_offset":22612,"parent_record_offset":20240,"ancestor_record_offset":20240,"inlinee":8380,"source_line":146,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::Temporal::split_pair
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":197,"record_offset":22584,"parent_record_offset":20240,"ancestor_record_offset":20240,"inlinee":8381,"source_line":158,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":197,"record_offset":20708,"parent_record_offset":20680,"ancestor_record_offset":20240,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":22140,"parent_record_offset":22108,"ancestor_record_offset":20240,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":22084,"parent_record_offset":21696,"ancestor_record_offset":20240,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":22108,"parent_record_offset":22084,"ancestor_record_offset":20240,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":197,"record_offset":21724,"parent_record_offset":21696,"ancestor_record_offset":20240,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":21772,"parent_record_offset":21748,"ancestor_record_offset":20240,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":21748,"parent_record_offset":21724,"ancestor_record_offset":20240,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":21864,"parent_record_offset":21800,"ancestor_record_offset":20240,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":21952,"parent_record_offset":21928,"ancestor_record_offset":20240,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":20680,"parent_record_offset":20648,"ancestor_record_offset":20240,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":21696,"parent_record_offset":20680,"ancestor_record_offset":20240,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":197,"record_offset":21428,"parent_record_offset":21404,"ancestor_record_offset":20240,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":21476,"parent_record_offset":21452,"ancestor_record_offset":20240,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":21404,"parent_record_offset":21380,"ancestor_record_offset":20240,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":197,"record_offset":22384,"parent_record_offset":22316,"ancestor_record_offset":20240,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":197,"record_offset":20896,"parent_record_offset":20872,"ancestor_record_offset":20240,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":197,"record_offset":20980,"parent_record_offset":20956,"ancestor_record_offset":20240,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":197,"record_offset":20920,"parent_record_offset":20896,"ancestor_record_offset":20240,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":197,"record_offset":21092,"parent_record_offset":21068,"ancestor_record_offset":20240,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":21888,"parent_record_offset":21864,"ancestor_record_offset":20240,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":21664,"parent_record_offset":20708,"ancestor_record_offset":20240,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":21928,"parent_record_offset":21748,"ancestor_record_offset":20240,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":22184,"parent_record_offset":20648,"ancestor_record_offset":20240,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":197,"record_offset":22032,"parent_record_offset":22008,"ancestor_record_offset":20240,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":22284,"parent_record_offset":22244,"ancestor_record_offset":20240,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":21316,"parent_record_offset":21292,"ancestor_record_offset":20240,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":21236,"parent_record_offset":21212,"ancestor_record_offset":20240,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":21800,"parent_record_offset":21772,"ancestor_record_offset":20240,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":21836,"parent_record_offset":21800,"ancestor_record_offset":20240,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":22008,"parent_record_offset":21976,"ancestor_record_offset":20240,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":197,"record_offset":21452,"parent_record_offset":21428,"ancestor_record_offset":20240,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":197,"record_offset":21380,"parent_record_offset":21356,"ancestor_record_offset":20240,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":20732,"parent_record_offset":20708,"ancestor_record_offset":20240,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":20756,"parent_record_offset":20732,"ancestor_record_offset":20240,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":197,"record_offset":21356,"parent_record_offset":21160,"ancestor_record_offset":20240,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":21268,"parent_record_offset":21184,"ancestor_record_offset":20240,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":21212,"parent_record_offset":21184,"ancestor_record_offset":20240,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":21612,"parent_record_offset":20832,"ancestor_record_offset":20240,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":21184,"parent_record_offset":21160,"ancestor_record_offset":20240,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":20808,"parent_record_offset":20784,"ancestor_record_offset":20240,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":20784,"parent_record_offset":20756,"ancestor_record_offset":20240,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":20832,"parent_record_offset":20808,"ancestor_record_offset":20240,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":21040,"parent_record_offset":21012,"ancestor_record_offset":20240,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":21068,"parent_record_offset":21040,"ancestor_record_offset":20240,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":21128,"parent_record_offset":21012,"ancestor_record_offset":20240,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":197,"record_offset":21012,"parent_record_offset":20832,"ancestor_record_offset":20240,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":197,"record_offset":21160,"parent_record_offset":20832,"ancestor_record_offset":20240,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":20872,"parent_record_offset":20832,"ancestor_record_offset":20240,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":20956,"parent_record_offset":20832,"ancestor_record_offset":20240,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":197,"record_offset":21528,"parent_record_offset":20832,"ancestor_record_offset":20240,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":197,"record_offset":21976,"parent_record_offset":21952,"ancestor_record_offset":20240,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":197,"record_offset":22356,"parent_record_offset":22316,"ancestor_record_offset":20240,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00197.jsonl.gz"}
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
// 完整结构及原始类型记录: render_loop.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::render_loop::SmolRingBuffer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: render_loop.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/tiny-glade/src/render_loop.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: render_loop.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] tiny_glade::render_loop::compile_loading_screen_frame_render_graph
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":197,"record_offset":65672,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00197.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":197,"record_offset":65716,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00197.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] tiny_glade::render_loop::render
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_transitive","evidence":{"module":197,"record_offset":65672,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00197.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_transitive","evidence":{"module":197,"record_offset":65716,"parent_record_offset":50140,"ancestor_record_offset":41428,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00197.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":111488,"caller_aliases":{"alias_group_rva":111488,"count":1,"display_examples":["std::sys_common::backtrace::__rust_begin_short_backtrace"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":113130,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":2386416,"caller_aliases":{"alias_group_rva":2386416,"count":1,"display_examples":["tiny_glade::game_loop::GameLoop::main_loop"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":2408367,"position_among_mapped_render_targets":2,"mapped_render_target_count":2,"previous_callsite":2391460,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/vermeer.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3152112,"caller_aliases":{"alias_group_rva":3152112,"count":1,"display_examples":["tiny_glade::render_loop::render"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3154478,"position_among_mapped_render_targets":4,"mapped_render_target_count":9,"previous_callsite":3154249,"next_callsite":3157166,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs"],"next_target_files":["crates/tiny-glade/src/render_loop.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3152112,"caller_aliases":{"alias_group_rva":3152112,"count":1,"display_examples":["tiny_glade::render_loop::render"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3157166,"position_among_mapped_render_targets":5,"mapped_render_target_count":9,"previous_callsite":3154478,"next_callsite":3159209,"previous_target_files":["crates/tiny-glade/src/render_loop.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/vermeer.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: render_loop.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::SmolRingBuffer::push
// 信息 ID: 38d8d4f87e438afa9b368ed2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=73400; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=73472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_frame_render_graph
// 信息 ID: 19f55b1c619caa8b8e97ff23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x30222e -> 0x2fcec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x302cae -> 0x2fcec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_frame_render_graph::closure$0
// 信息 ID: 0fa304803bb690cd1c5f7201
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_frame_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=18244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_frame_render_graph::closure$1
// 信息 ID: 7689c517c4e86b70fe4838df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::render_loop::compile_frame_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=18416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_frame_render_graph::explain
// 信息 ID: 186de6b8997dcebe987276ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=38424; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=40196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_loading_screen_frame_render_graph
// 信息 ID: 493539c266c2498274a15991
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=50140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_loading_screen_frame_render_graph::closure$0
// 信息 ID: b84309621a3eccc9e71adbcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_loading_screen_frame_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=56464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::handle_screenshot_error
// 信息 ID: 60c1a4302801f2d112a4ba10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=27084; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=27348; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=30764; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=31032; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=31300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::impl$3::eq
// 信息 ID: 55e7552ab579c8a4da0d647b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=48168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render
// 信息 ID: 0433ee29b9d91653e4c657d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> > @ 0x1b9ea -> 0x3018f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24bfaf -> 0x3018f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$0
// 信息 ID: 04229d60568bf0a86e3a4323
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=48108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$1
// 信息 ID: 2620d07610971def264ba08c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=71000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$3
// 信息 ID: 43c91d1f273d31c0f0042225
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=73588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$4
// 信息 ID: a170c674713a17be51c19610
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=73760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world
// 信息 ID: 5eeb9861b7f2ec53da2c0221
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=20240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
