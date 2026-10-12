// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/loading_screen.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4e75e7eefb084743d2b43473d27ab1f536a33225656a64ea650127c876b95b6e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: loading_screen.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 448, 1308, 1325, 1361
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
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::loading_screen::update_loading_screen
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64A2
// 首条位置: {"module":1325,"inlinee":25762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1325,"record_offset":1874240,"parent_record_offset":1874208,"ancestor_record_offset":1874208,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01325.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1325,"record_offset":1874268,"parent_record_offset":1874240,"ancestor_record_offset":1874208,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01325.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::loading_screen::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":448,"record_offset":25428,"rva":6309504,"va":5375018624}
// 编译实例 RVAs: 0x604680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140f16410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: loading_screen.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9510255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018624,"raw_hex":"e86f1d9100","size":5,"soft_fail":false,"target_va":5384528912,"va":5375018652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8021260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018624,"raw_hex":"e80c657a00","size":5,"soft_fail":false,"target_va":5383039904,"va":5375018639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t8086189","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375018624,"raw_hex":"e9ad627b00","size":5,"soft_fail":false,"target_va":5383104880,"va":5375018686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":448,"record_offset":25568,"parent_record_offset":25428,"ancestor_record_offset":25428,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::loading_screen::startup_loading_screen
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":448,"record_offset":25628,"rva":6309584,"va":5375018704}
// 编译实例 RVAs: 0x6046d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: loading_screen.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2393829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018704,"raw_hex":"e81b79dbff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375018848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: loading_screen.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2269257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018704,"raw_hex":"e8b75fddff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375018948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: loading_screen.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2269670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018704,"raw_hex":"e81a5eddff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375019377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: loading_screen.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t712298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018704,"raw_hex":"e86ade0a00","size":5,"soft_fail":false,"target_va":5375731504,"va":5375019201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: loading_screen.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t968989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018704,"raw_hex":"e81dc90e00","size":5,"soft_fail":false,"target_va":5375988432,"va":5375019438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d75160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: loading_screen.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7801366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018704,"raw_hex":"e8160a7700","size":5,"soft_fail":false,"target_va":5382820192,"va":5375018821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27958823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018704,"raw_hex":"e8279eaa01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375019316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9363545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018704,"raw_hex":"e859e08e00","size":5,"soft_fail":false,"target_va":5384382832,"va":5375019282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-453073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018704,"raw_hex":"e82f16f9ff","size":5,"soft_fail":false,"target_va":5374566080,"va":5375019148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2347393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375018704,"raw_hex":"e881d12300","size":5,"soft_fail":false,"target_va":5377366224,"va":5375018826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":448,"record_offset":25856,"parent_record_offset":25628,"ancestor_record_offset":25628,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":27100,"parent_record_offset":25628,"ancestor_record_offset":25628,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":448,"record_offset":25796,"parent_record_offset":25628,"ancestor_record_offset":25628,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":448,"record_offset":25940,"parent_record_offset":25628,"ancestor_record_offset":25628,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":448,"record_offset":26396,"parent_record_offset":25628,"ancestor_record_offset":25628,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":448,"record_offset":27072,"parent_record_offset":25628,"ancestor_record_offset":25628,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":26848,"parent_record_offset":26824,"ancestor_record_offset":25628,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":26872,"parent_record_offset":26848,"ancestor_record_offset":25628,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":27276,"parent_record_offset":27252,"ancestor_record_offset":25628,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":26824,"parent_record_offset":26796,"ancestor_record_offset":25628,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":448,"record_offset":27252,"parent_record_offset":27152,"ancestor_record_offset":25628,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":27184,"parent_record_offset":27152,"ancestor_record_offset":25628,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":26796,"parent_record_offset":26768,"ancestor_record_offset":25628,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":26768,"parent_record_offset":26740,"ancestor_record_offset":25628,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":26944,"parent_record_offset":26768,"ancestor_record_offset":25628,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":448,"record_offset":27152,"parent_record_offset":27124,"ancestor_record_offset":25628,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":26704,"parent_record_offset":26672,"ancestor_record_offset":25628,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":26672,"parent_record_offset":26640,"ancestor_record_offset":25628,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":26608,"parent_record_offset":26576,"ancestor_record_offset":25628,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":26640,"parent_record_offset":26608,"ancestor_record_offset":25628,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":448,"record_offset":26576,"parent_record_offset":26544,"ancestor_record_offset":25628,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":448,"record_offset":26544,"parent_record_offset":26512,"ancestor_record_offset":25628,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":448,"record_offset":26512,"parent_record_offset":26464,"ancestor_record_offset":25628,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":448,"record_offset":25824,"parent_record_offset":25796,"ancestor_record_offset":25628,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":26740,"parent_record_offset":26704,"ancestor_record_offset":25628,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":448,"record_offset":25880,"parent_record_offset":25856,"ancestor_record_offset":25628,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":26228,"parent_record_offset":26200,"ancestor_record_offset":25628,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":26280,"parent_record_offset":26256,"ancestor_record_offset":25628,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":448,"record_offset":26256,"parent_record_offset":26200,"ancestor_record_offset":25628,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":448,"record_offset":27004,"parent_record_offset":26980,"ancestor_record_offset":25628,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":448,"record_offset":25904,"parent_record_offset":25880,"ancestor_record_offset":25628,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":448,"record_offset":26304,"parent_record_offset":26280,"ancestor_record_offset":25628,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":448,"record_offset":27216,"parent_record_offset":27184,"ancestor_record_offset":25628,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":448,"record_offset":26160,"parent_record_offset":26136,"ancestor_record_offset":25628,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":448,"record_offset":26980,"parent_record_offset":26704,"ancestor_record_offset":25628,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":27124,"parent_record_offset":27100,"ancestor_record_offset":25628,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":26900,"parent_record_offset":26872,"ancestor_record_offset":25628,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":448,"record_offset":26328,"parent_record_offset":26304,"ancestor_record_offset":25628,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":448,"record_offset":26464,"parent_record_offset":26396,"ancestor_record_offset":25628,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":448,"record_offset":26436,"parent_record_offset":26396,"ancestor_record_offset":25628,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":448,"record_offset":25964,"parent_record_offset":25940,"ancestor_record_offset":25628,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":448,"record_offset":26012,"parent_record_offset":25988,"ancestor_record_offset":25628,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":448,"record_offset":26088,"parent_record_offset":26060,"ancestor_record_offset":25628,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":448,"record_offset":26200,"parent_record_offset":26060,"ancestor_record_offset":25628,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":448,"record_offset":25988,"parent_record_offset":25964,"ancestor_record_offset":25628,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":448,"record_offset":26112,"parent_record_offset":26088,"ancestor_record_offset":25628,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":448,"record_offset":26136,"parent_record_offset":26112,"ancestor_record_offset":25628,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":448,"record_offset":26060,"parent_record_offset":26036,"ancestor_record_offset":25628,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":448,"record_offset":26036,"parent_record_offset":26012,"ancestor_record_offset":25628,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::loading_screen::startup_loading_screen::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF037
// 首条位置: {"module":1308,"inlinee":61495}
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
// 完整结构及原始类型记录: loading_screen.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::loading_screen::LoadingScreen
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$9::add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::startup::MinimalStartupPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::logo_animation::AnimatedLogo,bevy_app::plugin::sealed::PluginMarker,country_core::resources::loading_screen::LoadingScreen,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::sealed::impl$9::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "country_core::startup::MinimalStartupPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::logo_animation::AnimatedLogo", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::loading_screen::LoadingScreen", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::resources::loading_screen::LoadingScreen>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::resources::loading_screen::LoadingScreen"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::resources::loading_screen::LoadingScreen>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::resources::loading_screen::LoadingScreen"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::resources::loading_screen::LoadingScreen>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::resources::loading_screen::LoadingScreen"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::resources::loading_screen::LoadingScreen>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::resources::loading_screen::LoadingScreen"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::resources::loading_screen::LoadingScreen>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::resources::loading_screen::LoadingScreen"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<country_core::resources::loading_screen::LoadingScreen>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["country_core::resources::loading_screen::LoadingScreen"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<country_core::resources::loading_screen::LoadingScreen>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["country_core::resources::loading_screen::LoadingScreen"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::loading_screen::LoadingScreenProgress
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 51
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>", "constructor": "bevy_ecs::change_detection::ResMut", "arguments": ["country_core::resources::loading_screen::LoadingScreenProgress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress>", "constructor": "bevy_ecs::change_detection::Res", "arguments": ["country_core::resources::loading_screen::LoadingScreenProgress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> >", "constructor": "core::option::Option", "arguments": ["bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >", "constructor": "enum2$", "arguments": ["core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$4::run_if<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "constructor": "bevy_ecs::schedule::config::impl$4::run_if", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<country_core::resources::loading_screen::LoadingScreenProgress>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["country_core::resources::loading_screen::LoadingScreenProgress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >),bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if", "arguments": ["void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)>", "bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$4::init_state<country_core::resources::loading_screen::LoadingScreenProgress>", "constructor": "bevy_ecs::system::system_param::impl$4::init_state", "arguments": ["country_core::resources::loading_screen::LoadingScreenProgress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<country_core::resources::loading_screen::LoadingScreenProgress>", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["country_core::resources::loading_screen::LoadingScreenProgress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::impl$6::init_resource::closure_env$0<country_core::resources::loading_screen::LoadingScreenProgress>", "constructor": "bevy_ecs::component::impl$6::init_resource::closure_env$0", "arguments": ["country_core::resources::loading_screen::LoadingScreenProgress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::Components::get_or_insert_resource_with<bevy_ecs::component::impl$6::init_resource::closure_env$0<country_core::resources::loading_screen::LoadingScreenProgress> >", "constructor": "bevy_ecs::component::Components::get_or_insert_resource_with", "arguments": ["bevy_ecs::component::impl$6::init_resource::closure_env$0<country_core::resources::loading_screen::LoadingScreenProgress>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: loading_screen.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: loading_screen.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 213; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>", "bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>", "utils::timer::FilteredWallTime"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::loading_screen::impl$0::build
// 信息 ID: 619173040ccc825686f64107
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::loading_screen::startup_loading_screen
// 信息 ID: 3d264306d22598d861a81304
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cc1c15e21a217d5b987dc287: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > @ 0xe729d0 -> 0x6046d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> @ 0xf14ed1 -> 0x6046d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::loading_screen::startup_loading_screen::closure$0
// 信息 ID: 0f85d8168868ee52163a9503
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::loading_screen::startup_loading_screen", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::loading_screen::startup_loading_screen::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::ops::range::RangeInclusive<u32>,tuple$<u32,u32> >,array$<u32,6>,country_core::resources::loading_screen::startup_loading_screen::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::ops::range::RangeInclusive<u32>,tuple$<u32,u32> >,array$<u32,6>,country_core::resources::loading_screen::startup_loading_screen::closure_env$0> >; module=1308, inline_record=110616; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::ops::range::RangeInclusive<u32>,tuple$<u32,u32> >,array$<u32,6>,country_core::resources::loading_screen::startup_loading_screen::closure_env$0> >; module=1308, inline_record=112824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::loading_screen::update_loading_screen
// 信息 ID: 53e31e413c6447678e472cf8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cf093ada564e24fb482867e0: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)> >; module=1325, inline_record=1874208; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::loading_screen::LoadingScreenProgress>,bevy_ecs::change_detection::Res<country_core::resources::logo_animation::LogoAnimation>,utils::timer::FilteredWallTime)>; module=1361, inline_record=24012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
