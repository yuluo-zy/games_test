// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/lut_gallery.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3576135e01e79e12719b697efd2228661f4a5036cfcf615591b21ed77e9eba83
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lut_gallery.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 402, 1325, 1363
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"PDB_registry_path_only","version":"0.11.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::lut_gallery::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":402,"record_offset":39400,"rva":5591152,"va":5374300272}
// 编译实例 RVAs: 0x555070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1550624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300272,"raw_hex":"e8e056e8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374300315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28677724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300272,"raw_hex":"e85c96b501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374300415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8758888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300272,"raw_hex":"e868a68500","size":5,"soft_fail":false,"target_va":5383059280,"va":5374300387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t8738267","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374300272,"raw_hex":"e9db558500","size":5,"soft_fail":false,"target_va":5383038672,"va":5374300400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":402,"record_offset":39864,"parent_record_offset":39400,"ancestor_record_offset":39400,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":402,"record_offset":39552,"parent_record_offset":39400,"ancestor_record_offset":39400,"inlinee":23120,"source_line":202,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":402,"record_offset":39756,"parent_record_offset":39732,"ancestor_record_offset":39400,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":402,"record_offset":39780,"parent_record_offset":39756,"ancestor_record_offset":39400,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":402,"record_offset":39704,"parent_record_offset":39676,"ancestor_record_offset":39400,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":402,"record_offset":39732,"parent_record_offset":39704,"ancestor_record_offset":39400,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":402,"record_offset":39676,"parent_record_offset":39648,"ancestor_record_offset":39400,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":402,"record_offset":39648,"parent_record_offset":39576,"ancestor_record_offset":39400,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":402,"record_offset":39576,"parent_record_offset":39552,"ancestor_record_offset":39400,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":402,"record_offset":39804,"parent_record_offset":39780,"ancestor_record_offset":39400,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$27::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":402,"record_offset":39620,"parent_record_offset":39600,"ancestor_record_offset":39400,"inlinee":12031,"source_line":167,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":402,"record_offset":39600,"parent_record_offset":39576,"ancestor_record_offset":39400,"inlinee":9750,"source_line":422,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::lut_gallery::startup
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A70
// 首条位置: {"module":1325,"inlinee":23152}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":1325,"record_offset":1681516,"parent_record_offset":1681488,"ancestor_record_offset":1681488,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/01325.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1325,"record_offset":1681540,"parent_record_offset":1681516,"ancestor_record_offset":1681488,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01325.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":402,"record_offset":39896,"rva":5591312,"va":5374300432}
// 编译实例 RVAs: 0x555110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140361a60
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2051325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e803b3e0ff","size":5,"soft_fail":false,"target_va":5372254816,"va":5374306136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1678122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e8d664e6ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374303141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1554219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e8d548e8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374303926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-86688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e860adfeff","size":5,"soft_fail":false,"target_va":5374216800,"va":5374303483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140644950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t978089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e8a9ec0e00","size":5,"soft_fail":false,"target_va":5375281488,"va":5374303394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e49f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1632204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e8cce71800","size":5,"soft_fail":false,"target_va":5375937008,"va":5374304799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1657455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e86f4a1900","size":5,"soft_fail":false,"target_va":5375963408,"va":5374305948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1662476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e80c5e1900","size":5,"soft_fail":false,"target_va":5375967552,"va":5374305071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1919835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e85b4b1d00","size":5,"soft_fail":false,"target_va":5376222944,"va":5374303104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d84b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8581658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e81af28200","size":5,"soft_fail":false,"target_va":5382884176,"va":5374302513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddb570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8935383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e8d7578800","size":5,"soft_fail":false,"target_va":5383239024,"va":5374303636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9045024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e820048a00","size":5,"soft_fail":false,"target_va":5383348608,"va":5374303579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9046023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e807088a00","size":5,"soft_fail":false,"target_va":5383349824,"va":5374303796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9047069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e81d0c8a00","size":5,"soft_fail":false,"target_va":5383350624,"va":5374303550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29290319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e84fefbe01","size":5,"soft_fail":false,"target_va":5403593344,"va":5374303020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lut_gallery.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29289944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e8d8edbe01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374303411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3227783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e887403100","size":5,"soft_fail":false,"target_va":5377529888,"va":5374302100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28674685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e87d8ab501","size":5,"soft_fail":false,"target_va":5402978880,"va":5374304190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1390760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e8a8381500","size":5,"soft_fail":false,"target_va":5375692416,"va":5374301651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1400509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e8bd5e1500","size":5,"soft_fail":false,"target_va":5375704400,"va":5374303886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1397346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e862521500","size":5,"soft_fail":false,"target_va":5375700688,"va":5374303337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1093233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e871ae1000","size":5,"soft_fail":false,"target_va":5375395536,"va":5374302298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1094172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e81cb21000","size":5,"soft_fail":false,"target_va":5375398240,"va":5374304063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t28667019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e88b6cb501","size":5,"soft_fail":false,"target_va":5402968992,"va":5374301968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t28666675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e8336bb501","size":5,"soft_fail":false,"target_va":5402970800,"va":5374304120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2039565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e80d1f1f00","size":5,"soft_fail":false,"target_va":5376341760,"va":5374302190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3063108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e844bd2e00","size":5,"soft_fail":false,"target_va":5377366224,"va":5374303111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2642978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374300432,"raw_hex":"e822542800","size":5,"soft_fail":false,"target_va":5376946416,"va":5374303433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":402,"record_offset":50616,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":402,"record_offset":48432,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":402,"record_offset":45320,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":402,"record_offset":49548,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":402,"record_offset":48828,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::enumerate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":402,"record_offset":49492,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":16076,"source_line":976,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":402,"record_offset":49408,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":43280,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":46828,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":51528,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":402,"record_offset":43576,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":402,"record_offset":48744,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":402,"record_offset":51296,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":402,"record_offset":51384,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":23127,"source_line":461,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":402,"record_offset":50764,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":402,"record_offset":48884,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::GradingLut::lut_name_to_asset_name
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":402,"record_offset":48632,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":22121,"source_line":81,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::LoadMenuThumbnailMetrics::positions
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":402,"record_offset":49204,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":23124,"source_line":191,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::image_formation::ImageFormationConfigs::current
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":402,"record_offset":49064,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":22349,"source_line":76,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":402,"record_offset":40152,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":402,"record_offset":43308,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":402,"record_offset":43492,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":10631,"source_line":639,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":402,"record_offset":51668,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":402,"record_offset":51584,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":402,"record_offset":51932,"parent_record_offset":51908,"ancestor_record_offset":39896,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":402,"record_offset":51908,"parent_record_offset":51852,"ancestor_record_offset":39896,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":402,"record_offset":48656,"parent_record_offset":48632,"ancestor_record_offset":39896,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":402,"record_offset":48704,"parent_record_offset":48680,"ancestor_record_offset":39896,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":402,"record_offset":51880,"parent_record_offset":51852,"ancestor_record_offset":39896,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":402,"record_offset":51852,"parent_record_offset":51828,"ancestor_record_offset":39896,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":402,"record_offset":49256,"parent_record_offset":49232,"ancestor_record_offset":39896,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":402,"record_offset":49316,"parent_record_offset":49204,"ancestor_record_offset":39896,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":402,"record_offset":52056,"parent_record_offset":52032,"ancestor_record_offset":39896,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":402,"record_offset":52104,"parent_record_offset":52080,"ancestor_record_offset":39896,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":402,"record_offset":49648,"parent_record_offset":49616,"ancestor_record_offset":39896,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":402,"record_offset":50648,"parent_record_offset":50616,"ancestor_record_offset":39896,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":402,"record_offset":40212,"parent_record_offset":40184,"ancestor_record_offset":39896,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":402,"record_offset":41644,"parent_record_offset":41612,"ancestor_record_offset":39896,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":402,"record_offset":41588,"parent_record_offset":41200,"ancestor_record_offset":39896,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":402,"record_offset":41612,"parent_record_offset":41588,"ancestor_record_offset":39896,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":402,"record_offset":41228,"parent_record_offset":41200,"ancestor_record_offset":39896,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":402,"record_offset":41276,"parent_record_offset":41252,"ancestor_record_offset":39896,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":402,"record_offset":41252,"parent_record_offset":41228,"ancestor_record_offset":39896,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":402,"record_offset":41368,"parent_record_offset":41304,"ancestor_record_offset":39896,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":402,"record_offset":41456,"parent_record_offset":41432,"ancestor_record_offset":39896,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":402,"record_offset":40184,"parent_record_offset":40152,"ancestor_record_offset":39896,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":402,"record_offset":41200,"parent_record_offset":40184,"ancestor_record_offset":39896,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":402,"record_offset":40932,"parent_record_offset":40908,"ancestor_record_offset":39896,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":402,"record_offset":51124,"parent_record_offset":51096,"ancestor_record_offset":39896,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":402,"record_offset":51176,"parent_record_offset":51152,"ancestor_record_offset":39896,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":402,"record_offset":51152,"parent_record_offset":51096,"ancestor_record_offset":39896,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":402,"record_offset":43516,"parent_record_offset":43492,"ancestor_record_offset":39896,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":402,"record_offset":40980,"parent_record_offset":40956,"ancestor_record_offset":39896,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":402,"record_offset":40908,"parent_record_offset":40884,"ancestor_record_offset":39896,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":402,"record_offset":48916,"parent_record_offset":48884,"ancestor_record_offset":39896,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":402,"record_offset":40400,"parent_record_offset":40376,"ancestor_record_offset":39896,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":402,"record_offset":40484,"parent_record_offset":40460,"ancestor_record_offset":39896,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":402,"record_offset":40424,"parent_record_offset":40400,"ancestor_record_offset":39896,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":402,"record_offset":48852,"parent_record_offset":48828,"ancestor_record_offset":39896,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":402,"record_offset":49824,"parent_record_offset":49796,"ancestor_record_offset":39896,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":402,"record_offset":49796,"parent_record_offset":49616,"ancestor_record_offset":39896,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":402,"record_offset":50172,"parent_record_offset":50140,"ancestor_record_offset":39896,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::Enumerate::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":402,"record_offset":49516,"parent_record_offset":49492,"ancestor_record_offset":39896,"inlinee":16078,"source_line":23,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":402,"record_offset":50224,"parent_record_offset":50172,"ancestor_record_offset":39896,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":402,"record_offset":49432,"parent_record_offset":49408,"ancestor_record_offset":39896,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":402,"record_offset":49588,"parent_record_offset":49548,"ancestor_record_offset":39896,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":402,"record_offset":49456,"parent_record_offset":49432,"ancestor_record_offset":39896,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":402,"record_offset":49616,"parent_record_offset":49588,"ancestor_record_offset":39896,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::once::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\sources\once.rs
//     evidence={"module":402,"record_offset":49908,"parent_record_offset":49884,"ancestor_record_offset":39896,"inlinee":8637,"source_line":73,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":402,"record_offset":49280,"parent_record_offset":49256,"ancestor_record_offset":39896,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":402,"record_offset":49232,"parent_record_offset":49204,"ancestor_record_offset":39896,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":402,"record_offset":49344,"parent_record_offset":49316,"ancestor_record_offset":39896,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":402,"record_offset":50004,"parent_record_offset":49980,"ancestor_record_offset":39896,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":402,"record_offset":52208,"parent_record_offset":52180,"ancestor_record_offset":39896,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":402,"record_offset":40596,"parent_record_offset":40572,"ancestor_record_offset":39896,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":402,"record_offset":49884,"parent_record_offset":49824,"ancestor_record_offset":39896,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":402,"record_offset":50356,"parent_record_offset":50332,"ancestor_record_offset":39896,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":49856,"parent_record_offset":49824,"ancestor_record_offset":39896,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":41392,"parent_record_offset":41368,"ancestor_record_offset":39896,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":41168,"parent_record_offset":40212,"ancestor_record_offset":39896,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":50080,"parent_record_offset":49824,"ancestor_record_offset":39896,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":50104,"parent_record_offset":50080,"ancestor_record_offset":39896,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":41432,"parent_record_offset":41252,"ancestor_record_offset":39896,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":48680,"parent_record_offset":48656,"ancestor_record_offset":39896,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":50140,"parent_record_offset":49796,"ancestor_record_offset":39896,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":49980,"parent_record_offset":49956,"ancestor_record_offset":39896,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":41688,"parent_record_offset":40152,"ancestor_record_offset":39896,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":51552,"parent_record_offset":51528,"ancestor_record_offset":39896,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$18::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":49956,"parent_record_offset":49932,"ancestor_record_offset":39896,"inlinee":8605,"source_line":2242,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$34::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":49932,"parent_record_offset":49908,"ancestor_record_offset":39896,"inlinee":8604,"source_line":2373,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":50576,"parent_record_offset":49616,"ancestor_record_offset":39896,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":402,"record_offset":41536,"parent_record_offset":41512,"ancestor_record_offset":39896,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":402,"record_offset":51780,"parent_record_offset":51756,"ancestor_record_offset":39896,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":402,"record_offset":48792,"parent_record_offset":48768,"ancestor_record_offset":39896,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":402,"record_offset":40820,"parent_record_offset":40796,"ancestor_record_offset":39896,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":402,"record_offset":40740,"parent_record_offset":40716,"ancestor_record_offset":39896,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":402,"record_offset":49708,"parent_record_offset":49648,"ancestor_record_offset":39896,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":402,"record_offset":49736,"parent_record_offset":49648,"ancestor_record_offset":39896,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":402,"record_offset":49680,"parent_record_offset":49648,"ancestor_record_offset":39896,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":402,"record_offset":49760,"parent_record_offset":49736,"ancestor_record_offset":39896,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":402,"record_offset":50028,"parent_record_offset":50004,"ancestor_record_offset":39896,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":402,"record_offset":43428,"parent_record_offset":43336,"ancestor_record_offset":39896,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":402,"record_offset":43452,"parent_record_offset":43428,"ancestor_record_offset":39896,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":402,"record_offset":50680,"parent_record_offset":50648,"ancestor_record_offset":39896,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":402,"record_offset":50712,"parent_record_offset":50680,"ancestor_record_offset":39896,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":402,"record_offset":41304,"parent_record_offset":41276,"ancestor_record_offset":39896,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":402,"record_offset":41340,"parent_record_offset":41304,"ancestor_record_offset":39896,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":402,"record_offset":41512,"parent_record_offset":41480,"ancestor_record_offset":39896,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":402,"record_offset":48768,"parent_record_offset":48744,"ancestor_record_offset":39896,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":402,"record_offset":50272,"parent_record_offset":50248,"ancestor_record_offset":39896,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":402,"record_offset":50248,"parent_record_offset":50224,"ancestor_record_offset":39896,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":402,"record_offset":43656,"parent_record_offset":43624,"ancestor_record_offset":39896,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":402,"record_offset":43372,"parent_record_offset":43336,"ancestor_record_offset":39896,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":402,"record_offset":43680,"parent_record_offset":43656,"ancestor_record_offset":39896,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":402,"record_offset":43396,"parent_record_offset":43372,"ancestor_record_offset":39896,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":402,"record_offset":40956,"parent_record_offset":40932,"ancestor_record_offset":39896,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":402,"record_offset":50804,"parent_record_offset":50764,"ancestor_record_offset":39896,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":402,"record_offset":48992,"parent_record_offset":48964,"ancestor_record_offset":39896,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":402,"record_offset":48964,"parent_record_offset":48940,"ancestor_record_offset":39896,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":402,"record_offset":48940,"parent_record_offset":48916,"ancestor_record_offset":39896,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::GradingLut::asset_name
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":402,"record_offset":50408,"parent_record_offset":50380,"ancestor_record_offset":39896,"inlinee":22173,"source_line":77,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::GradingLut::lut_name_to_asset_name
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":402,"record_offset":50432,"parent_record_offset":50408,"ancestor_record_offset":39896,"inlinee":22121,"source_line":81,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/lut_gallery.rs
//     evidence={"module":402,"record_offset":50380,"parent_record_offset":50356,"ancestor_record_offset":39896,"inlinee":23126,"source_line":98,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":402,"record_offset":40884,"parent_record_offset":40860,"ancestor_record_offset":39896,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":402,"record_offset":40236,"parent_record_offset":40212,"ancestor_record_offset":39896,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":402,"record_offset":40260,"parent_record_offset":40236,"ancestor_record_offset":39896,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":402,"record_offset":40860,"parent_record_offset":40664,"ancestor_record_offset":39896,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":40772,"parent_record_offset":40688,"ancestor_record_offset":39896,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":40716,"parent_record_offset":40688,"ancestor_record_offset":39896,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":41116,"parent_record_offset":40336,"ancestor_record_offset":39896,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":40688,"parent_record_offset":40664,"ancestor_record_offset":39896,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":40312,"parent_record_offset":40288,"ancestor_record_offset":39896,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":40288,"parent_record_offset":40260,"ancestor_record_offset":39896,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":40336,"parent_record_offset":40312,"ancestor_record_offset":39896,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":402,"record_offset":40544,"parent_record_offset":40516,"ancestor_record_offset":39896,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":402,"record_offset":40572,"parent_record_offset":40544,"ancestor_record_offset":39896,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":402,"record_offset":40632,"parent_record_offset":40516,"ancestor_record_offset":39896,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":402,"record_offset":40516,"parent_record_offset":40336,"ancestor_record_offset":39896,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":402,"record_offset":40664,"parent_record_offset":40336,"ancestor_record_offset":39896,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":402,"record_offset":40376,"parent_record_offset":40336,"ancestor_record_offset":39896,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":402,"record_offset":40460,"parent_record_offset":40336,"ancestor_record_offset":39896,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":402,"record_offset":41032,"parent_record_offset":40336,"ancestor_record_offset":39896,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":402,"record_offset":43600,"parent_record_offset":43576,"ancestor_record_offset":39896,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":402,"record_offset":51632,"parent_record_offset":51608,"ancestor_record_offset":39896,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":402,"record_offset":41480,"parent_record_offset":41456,"ancestor_record_offset":39896,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":402,"record_offset":43336,"parent_record_offset":43308,"ancestor_record_offset":39896,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":402,"record_offset":43624,"parent_record_offset":43600,"ancestor_record_offset":39896,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":402,"record_offset":51692,"parent_record_offset":51668,"ancestor_record_offset":39896,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":402,"record_offset":51608,"parent_record_offset":51584,"ancestor_record_offset":39896,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":402,"record_offset":49016,"parent_record_offset":48992,"ancestor_record_offset":39896,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":402,"record_offset":50832,"parent_record_offset":50804,"ancestor_record_offset":39896,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":402,"record_offset":50888,"parent_record_offset":50860,"ancestor_record_offset":39896,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":402,"record_offset":50976,"parent_record_offset":50944,"ancestor_record_offset":39896,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":402,"record_offset":51096,"parent_record_offset":50944,"ancestor_record_offset":39896,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":402,"record_offset":50860,"parent_record_offset":50832,"ancestor_record_offset":39896,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":402,"record_offset":51000,"parent_record_offset":50976,"ancestor_record_offset":39896,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":402,"record_offset":51028,"parent_record_offset":51000,"ancestor_record_offset":39896,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":402,"record_offset":50944,"parent_record_offset":50916,"ancestor_record_offset":39896,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":402,"record_offset":50916,"parent_record_offset":50888,"ancestor_record_offset":39896,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A56
// 首条位置: {"module":402,"inlinee":23126}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::grading_lut::GradingLut::asset_name
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":402,"record_offset":50408,"parent_record_offset":50380,"ancestor_record_offset":50380,"inlinee":22173,"source_line":77,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":402,"record_offset":50456,"parent_record_offset":50432,"ancestor_record_offset":50380,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":402,"record_offset":50504,"parent_record_offset":50480,"ancestor_record_offset":50380,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":402,"record_offset":50480,"parent_record_offset":50456,"ancestor_record_offset":50380,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::GradingLut::lut_name_to_asset_name
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":402,"record_offset":50432,"parent_record_offset":50408,"ancestor_record_offset":50380,"inlinee":22121,"source_line":81,"archive":"_pdb/A/modules/00402.jsonl.gz"}
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
// 完整结构及原始类型记录: lut_gallery.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::lut_gallery::LutGalleryPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<country_core::systems::render_loop::lut_gallery::LutGalleryPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["country_core::systems::render_loop::lut_gallery::LutGalleryPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$14::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$14::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>", "constructor": "core::iter::sources::once::Once", "arguments": ["country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>", "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > > >", "constructor": "core::iter::adapters::enumerate::Enumerate", "arguments": ["core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lut_gallery.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/lut_gallery.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: lut_gallery.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":402,"record_offset":51296,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00402.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":402,"record_offset":51384,"parent_record_offset":39896,"ancestor_record_offset":39896,"inlinee":23127,"source_line":461,"archive":"_pdb/A/modules/00402.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"76285f6faf29f8540e44c159","relative_path":"_lut_gallery_thumbnail.raster.hlsl_b903e43ffb3da915.bin","sha256":"eb95fc78cda168a8688e1103964eefc58934cfe8da8797e0d76fe8d58d83a6e7","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"lut_gallery","prefix_candidates":["lut_gallery"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"e094b77fa6d642ab414acb55","relative_path":"_lut_gallery_thumbnail.raster.hlsl_f0adff76827e73ff.bin","sha256":"086a8e833a1f0e688cfed3f69b0c8507409becfbce2416086cd2533d36981cfa","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"lut_gallery","prefix_candidates":["lut_gallery"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _lut_gallery_thumbnail.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"$Globals","storage_class":"Uniform","type":{"id":36,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5]},"decorations":[],"pointee":{"id":5,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[35,35,35,11,11,12,12,10]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"input_tex_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"grading_lut_tex_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"framebuffer_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"thumbnail_center","type_id":11,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"thumbnail_size","type_id":11,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"is_hovered","type_id":12,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"is_active","type_id":12,"decorations":[{"decoration":"Offset","operands":[44]}]},{"index":7,"name":"lut_amount","type_id":10,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,12]},"decorations":[],"pointee":{"id":12,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":42,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[42]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":38,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[10,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C0","storage_class":"Output","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,11]},"decorations":[],"pointee":{"id":11,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[10,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"sampler_nnc","storage_class":"UniformConstant","type":{"id":47,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,5]},"decorations":[],"pointee":{"id":5,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"$Globals","storage_class":"Uniform","type":{"id":50,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,9]},"decorations":[],"pointee":{"id":9,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[49,49,49,19,19,20,20,18]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"input_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"grading_lut_tex_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"framebuffer_dims","type_id":49,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"thumbnail_center","type_id":19,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"thumbnail_size","type_id":19,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"is_hovered","type_id":20,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"is_active","type_id":20,"decorations":[{"decoration":"Offset","operands":[44]}]},{"index":7,"name":"lut_amount","type_id":18,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":11,"name":"input_tex","storage_class":"UniformConstant","type":{"id":51,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,8]},"decorations":[],"pointee":{"id":8,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[18,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":13,"name":"grading_lut_tex","storage_class":"UniformConstant","type":{"id":52,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,12]},"decorations":[],"pointee":{"id":12,"name":"type.3d.image","record":{"opcode":"OpTypeImage","operands":[18,2,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,19]},"decorations":[],"pointee":{"id":19,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[18,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.SV_TARGET","storage_class":"Output","type":{"id":54,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,48]},"decorations":[],"pointee":{"id":48,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[18,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _lut_gallery_thumbnail.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":1,"entry_name":"vs_main","interface_ids":[2,3,4]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"$Globals","storage_class":"Uniform","type":{"id":36,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,5]},"decorations":[],"pointee":{"id":5,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[35,35,35,11,11,12,12,10]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"input_tex_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"grading_lut_tex_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"framebuffer_dims","type_id":35,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"thumbnail_center","type_id":11,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"thumbnail_size","type_id":11,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"is_hovered","type_id":12,"decorations":[{"decoration":"Offset","operands":[40]}]},{"index":6,"name":"is_active","type_id":12,"decorations":[{"decoration":"Offset","operands":[44]}]},{"index":7,"name":"lut_amount","type_id":10,"decorations":[{"decoration":"Offset","operands":[48]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":2,"name":"unknown","storage_class":"Input","type":{"id":37,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,12]},"decorations":[],"pointee":{"id":12,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":42,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[42]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Output","type":{"id":38,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,25]},"decorations":[],"pointee":{"id":25,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[10,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"out.var.C0","storage_class":"Output","type":{"id":39,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,11]},"decorations":[],"pointee":{"id":11,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[10,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3144500,"position_among_mapped_render_targets":61,"mapped_render_target_count":70,"previous_callsite":3144467,"next_callsite":3144533,"previous_target_files":["crates/country-core/src/systems/render_loop/session_selection/mod.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/feedback_report_bg.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lut_gallery.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 207; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>", "bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(ref_mut$<bevy_ecs::world::World>)
// 候选参数类型: ["ref_mut$<bevy_ecs::world::World>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::lut_gallery::impl$0::build
// 信息 ID: c6a63a7cc77309a88918c577
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery
// 信息 ID: 0815281309fcb929e6402331
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffb34 -> 0x555110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure$0
// 信息 ID: 93bba1a88b7892795c3f4227
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::PreviewParams>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::grading_lut::GradingLut>,country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery; module=402, inline_record=50380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::lut_gallery::startup
// 信息 ID: 94cd494612b45c477d2c9922
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f6e723bf3a293de264db1c5c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> >; module=1325, inline_record=1681488; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)>; module=1363, inline_record=26892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
